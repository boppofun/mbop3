//! Regression harness for mbop3. Compares mbop3 against the original C minimp3
//! and against the ISO reference PCM, and measures stack, memory and speed.
//!
//! Run `mbop3-regress help` for usage (normally invoked through the justfile).

mod corpus;
mod decoders;
mod run;
mod stack;

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use decoders::{Decoder, Mbop3, RefF32, RefI16, Sample};
use run::{Criteria, Driver, Mismatch, Stats};

const USAGE: &str = "\
usage:
  mbop3-regress check [--criteria exact|iso] [--mutations N] [--window N] TIER...
      TIER is NAME=DIR or NAME=DIR:COUNT (deterministic subset of COUNT files).
      Tiers named 'iso' also get a compliance check against sibling .pcm files.
      Mutations are generated from the files of every tier except 'boppo'.
  mbop3-regress bench [--reps N] FILE_OR_DIR...
  mbop3-regress stack FILE_OR_DIR...
  mbop3-regress sizes
";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = match args.first().map(String::as_str) {
        Some("check") => check(&args[1..]),
        Some("bench") => bench(&args[1..]),
        Some("stack") => stack_cmd(&args[1..]),
        Some("sizes") => sizes(),
        Some("mutant") => {
            // mutant FILE OTHER SEED OUT: writes a mutated input for debugging.
            let a = std::fs::read(&args[1]).unwrap();
            let b = std::fs::read(&args[2]).unwrap();
            let seed = u64::from_str_radix(args[3].trim_start_matches("0x"), 16).unwrap();
            let (desc, d) = corpus::mutate(&a, &b, seed);
            std::fs::write(&args[4], d).unwrap();
            println!("{desc}");
            0
        }
        _ => {
            eprint!("{USAGE}");
            2
        }
    };
    std::process::exit(code);
}

struct Opts {
    criteria: Criteria,
    mutations: usize,
    window: usize,
    reps: usize,
    positional: Vec<String>,
}

fn parse_opts(args: &[String]) -> Opts {
    let mut o = Opts {
        criteria: Criteria::Exact,
        mutations: 0,
        window: 2048,
        reps: 5,
        positional: Vec::new(),
    };
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut val = || {
            it.next()
                .unwrap_or_else(|| panic!("{a} needs a value"))
                .clone()
        };
        match a.as_str() {
            "--criteria" => {
                o.criteria = match val().as_str() {
                    "exact" => Criteria::Exact,
                    "iso" => Criteria::iso_full_accuracy(),
                    c => panic!("unknown criteria {c}"),
                }
            }
            "--mutations" => o.mutations = val().parse().unwrap(),
            "--window" => o.window = val().parse().unwrap(),
            "--reps" => o.reps = val().parse().unwrap(),
            _ => o.positional.push(a.clone()),
        }
    }
    o
}

/// Files named by `paths`. A directory may have a `:COUNT` suffix to take a
/// deterministic subset of its files.
fn files_of(paths: &[String]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for p in paths {
        let (p, count) = match p.rsplit_once(':') {
            Some((d, c)) if c.parse::<usize>().is_ok() => (d, Some(c.parse().unwrap())),
            _ => (p.as_str(), None),
        };
        let p = Path::new(p);
        if p.is_dir() {
            let files = corpus::find(p);
            out.extend(match count {
                Some(n) => corpus::sample(files, n),
                None => files,
            });
        } else {
            out.push(p.to_path_buf());
        }
    }
    out
}

enum JobKind {
    File,
    Mutant { seed: u64, other: PathBuf },
}

struct Job {
    tier: String,
    path: PathBuf,
    kind: JobKind,
}

#[derive(Default)]
struct TierTotals {
    files: u64,
    frames: u64,
    samples: u64,
    differing: u64,
    max_abs: f64,
    failures: u64,
}

struct Failure {
    tier: String,
    label: String,
    mismatch: Mismatch,
}

fn compare_catching<S: Sample, A: Decoder<S>, B: Decoder<S>>(
    data: &[u8],
    driver: Driver,
    criteria: Criteria,
) -> (Stats, Option<Mismatch>) {
    let r = std::panic::catch_unwind(|| run::compare::<S, A, B>(data, driver, criteria));
    r.unwrap_or_else(|e| {
        let msg = e
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_default();
        (
            Stats::default(),
            Some(Mismatch {
                driver,
                frame: 0,
                offset: 0,
                what: format!("{}: panic: {msg}", A::name()),
            }),
        )
    })
}

fn check(args: &[String]) -> i32 {
    let o = parse_opts(args);
    let mut tiers: Vec<(String, Vec<PathBuf>)> = Vec::new();
    for t in &o.positional {
        let (name, rest) = t.split_once('=').expect("TIER must be NAME=DIR[:COUNT]");
        let (dir, count) = match rest.rsplit_once(':') {
            Some((d, c)) if c.parse::<usize>().is_ok() => (d, Some(c.parse().unwrap())),
            _ => (rest, None),
        };
        let mut files = corpus::find(Path::new(dir));
        if files.is_empty() {
            eprintln!("warning: tier {name}: no mp3 files in {dir}");
        }
        if let Some(n) = count {
            files = corpus::sample(files, n);
        }
        tiers.push((name.to_string(), files));
    }

    let mut jobs = Vec::new();
    for (tier, files) in &tiers {
        for f in files {
            jobs.push(Job {
                tier: tier.clone(),
                path: f.clone(),
                kind: JobKind::File,
            });
        }
    }
    if o.mutations > 0 {
        let seeds: Vec<PathBuf> = tiers
            .iter()
            .filter(|(t, _)| t != "boppo")
            .flat_map(|(_, f)| f.iter().cloned())
            .collect();
        for (i, f) in seeds.iter().enumerate() {
            for k in 0..o.mutations {
                let seed = corpus::hash(format!("{}#{k}", f.display()).as_bytes());
                let other = seeds[(i + 1 + k) % seeds.len()].clone();
                jobs.push(Job {
                    tier: "mutated".into(),
                    path: f.clone(),
                    kind: JobKind::Mutant { seed, other },
                });
            }
        }
    }

    let drivers = [Driver::Slice, Driver::Window(o.window)];
    let totals: Mutex<std::collections::BTreeMap<String, TierTotals>> = Default::default();
    let failures: Mutex<Vec<Failure>> = Default::default();
    let next = AtomicUsize::new(0);
    let start = Instant::now();
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(job) = jobs.get(i) else { break };
                    let orig = std::fs::read(&job.path).expect("read corpus file");
                    let (label, data) = match &job.kind {
                        JobKind::File => (job.path.display().to_string(), orig),
                        JobKind::Mutant { seed, other } => {
                            let other = std::fs::read(other).expect("read corpus file");
                            let (m, d) = corpus::mutate(&orig, &other, *seed);
                            (format!("{} [{m}, seed {seed:#x}]", job.path.display()), d)
                        }
                    };
                    let mut local = TierTotals {
                        files: 1,
                        ..Default::default()
                    };
                    let runs = drivers.iter().flat_map(|&d| [(d, false), (d, true)]);
                    for (driver, float) in runs {
                        let (stats, mismatch) = if float {
                            compare_catching::<f32, Mbop3, RefF32>(&data, driver, o.criteria)
                        } else {
                            compare_catching::<i16, Mbop3, RefI16>(&data, driver, o.criteria)
                        };
                        local.frames += stats.frames;
                        local.samples += stats.samples;
                        local.differing += stats.differing_samples;
                        local.max_abs = local.max_abs.max(stats.max_abs_diff);
                        if let Some(mismatch) = mismatch {
                            local.failures = 1;
                            failures.lock().unwrap().push(Failure {
                                tier: job.tier.clone(),
                                label: label.clone(),
                                mismatch,
                            });
                        }
                    }
                    let mut t = totals.lock().unwrap();
                    let e = t.entry(job.tier.clone()).or_default();
                    e.files += local.files;
                    e.frames += local.frames;
                    e.samples += local.samples;
                    e.differing += local.differing;
                    e.max_abs = e.max_abs.max(local.max_abs);
                    e.failures += local.failures;
                }
            });
        }
    });

    let criteria = match o.criteria {
        Criteria::Exact => "bit-exact",
        Criteria::Tolerance { .. } => "ISO full-accuracy tolerance",
    };
    println!(
        "mbop3 vs minimp3 ({criteria}), i16 and f32 output, drivers: slice + window{}",
        o.window
    );
    println!(
        "{:<10} {:>7} {:>10} {:>13} {:>11} {:>10} {:>7}",
        "tier", "files", "frames", "samples", "diff samp", "max diff", "failed"
    );
    let totals = totals.into_inner().unwrap();
    let mut failed_files = 0;
    for (tier, t) in &totals {
        println!(
            "{:<10} {:>7} {:>10} {:>13} {:>11} {:>10.2e} {:>7}",
            tier, t.files, t.frames, t.samples, t.differing, t.max_abs, t.failures
        );
        failed_files += t.failures;
    }
    let mut failures = failures.into_inner().unwrap();
    failures.sort_by(|a, b| (&a.tier, &a.label).cmp(&(&b.tier, &b.label)));
    for f in failures.iter().take(25) {
        println!(
            "FAIL [{}] {} ({}, frame {}, input offset {}): {}",
            f.tier,
            f.label,
            f.mismatch.driver,
            f.mismatch.frame,
            f.mismatch.offset,
            f.mismatch.what
        );
    }
    if failures.len() > 25 {
        println!("... and {} more failures", failures.len() - 25);
    }

    let mut compliance_failed = 0;
    for (tier, files) in &tiers {
        if tier == "iso" {
            compliance_failed += compliance(files);
        }
    }
    println!("check took {:.1}s", start.elapsed().as_secs_f64());
    if failed_files > 0 || compliance_failed > 0 {
        println!(
            "RESULT: FAIL ({failed_files} files differ, {compliance_failed} compliance failures)"
        );
        1
    } else {
        println!("RESULT: PASS");
        0
    }
}

/// PSNR against the ISO/minimp3 reference `.pcm` next to each vector, using
/// minimp3's own pass criteria (PSNR >= 96 dB, sample count rules).
fn compliance(files: &[PathBuf]) -> u64 {
    println!("compliance vs reference .pcm (minimp3_test criteria: PSNR >= 96 dB):");
    let mut failed = 0;
    let mut checked = 0;
    let mut skipped = 0;
    let mut known = 0;
    for f in files {
        let Ok(ref_bytes) = std::fs::read(f.with_extension("pcm")) else {
            continue;
        };
        let data = std::fs::read(f).unwrap();
        let reference: Vec<i16> = ref_bytes
            .as_chunks::<2>()
            .0
            .iter()
            .map(|c| i16::from_le_bytes([c[0], c[1]]))
            .collect();
        let (c_out, _) = run::decode_all::<i16, RefI16>(&data);
        if c_out.is_empty() {
            // Layer 1/2 vector: neither decoder handles it.
            skipped += 1;
            continue;
        }
        let (ours, _) = run::decode_all::<i16, Mbop3>(&data);
        checked += 1;
        let name = f.file_name().unwrap().to_string_lossy();
        let ours_score = score(&name, &reference, &ours);
        let c_score = score(&name, &reference, &c_out);
        if !ours_score.pass && c_score.pass {
            failed += 1;
            println!("  FAIL {name}: mbop3 {ours_score} but minimp3 {c_score}");
        } else if !ours_score.pass {
            known += 1;
            println!("  known {name}: {ours_score} (C minimp3's frame API too: {c_score})");
        }
    }
    println!(
        "  {checked} layer 3 vectors checked: {failed} failed, {known} fail the same way in C \
         (they test mp3dec_ex features: VBR tag skipping, gapless), {skipped} layer 1/2 skipped"
    );
    failed
}

struct Score {
    pass: bool,
    samples: usize,
    ref_samples: usize,
    max_diff: i32,
    psnr: f64,
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} samples (ref {}), max_diff {}, PSNR {:.2} dB",
            self.samples, self.ref_samples, self.max_diff, self.psnr
        )
    }
}

/// minimp3_test's pass criteria for a decode against a reference .pcm.
fn score(name: &str, reference: &[i16], got: &[i16]) -> Score {
    let nonstandard = name.contains("nonstandard") || name.contains("ILL");
    let (r, g) = (reference.len(), got.len());
    let len_ok = if nonstandard {
        r == g
    } else {
        r == g || r + 1152 == g || r + 2304 == g
    };
    let mut sq = 0f64;
    let mut max_diff = 0;
    for i in 0..r.min(g) {
        let d = (got[i] as i32 - reference[i] as i32).abs();
        max_diff = max_diff.max(d);
        sq += (d * d) as f64;
    }
    let mse = sq / g.max(1) as f64;
    let psnr = if mse == 0.0 {
        99.0
    } else {
        10.0 * ((32767.0f64 * 32767.0) / mse).log10()
    };
    Score {
        pass: len_ok && psnr >= 96.0,
        samples: g,
        ref_samples: r,
        max_diff,
        psnr,
    }
}

fn bench(args: &[String]) -> i32 {
    let o = parse_opts(args);
    let files = files_of(&o.positional);
    let datas: Vec<Vec<u8>> = files.iter().map(|f| std::fs::read(f).unwrap()).collect();
    let mut audio_secs = 0.0;
    for d in &datas {
        let (pcm, info) = run::decode_all::<i16, RefI16>(d);
        if let Some(i) = info {
            audio_secs += pcm.len() as f64 / i.channels as f64 / i.hz as f64;
        }
    }
    fn time<D: Decoder<i16>>(datas: &[Vec<u8>]) -> f64 {
        let t = Instant::now();
        for d in datas {
            std::hint::black_box(run::decode_all::<i16, D>(d));
        }
        t.elapsed().as_secs_f64()
    }
    let (mut best_ours, mut best_c) = (f64::MAX, f64::MAX);
    for _ in 0..o.reps {
        best_ours = best_ours.min(time::<Mbop3>(&datas));
        best_c = best_c.min(time::<RefI16>(&datas));
    }
    println!(
        "bench: {} files, {:.1}s of audio, best of {} runs",
        files.len(),
        audio_secs,
        o.reps
    );
    println!(
        "  mbop3   {:.4}s ({:.0}x realtime)",
        best_ours,
        audio_secs / best_ours
    );
    println!(
        "  minimp3 {:.4}s ({:.0}x realtime)",
        best_c,
        audio_secs / best_c
    );
    println!("  mbop3/minimp3 time ratio: {:.3}", best_ours / best_c);
    0
}

fn peak_stack<D: Decoder<i16> + Send>(data: &[u8]) -> usize {
    let mut dec = D::new();
    let mut pcm = Box::new([0i16; decoders::MAX_SAMPLES_PER_FRAME]);
    stack::measure(|| {
        let mut pos = 0;
        while pos < data.len() {
            let (_, info) = dec.decode(&data[pos..], Some(&mut pcm));
            if info.frame_bytes == 0 {
                break;
            }
            pos += info.frame_bytes as usize;
        }
    })
}

fn stack_cmd(args: &[String]) -> i32 {
    let o = parse_opts(args);
    let files = files_of(&o.positional);
    let (mut ours, mut c) = (0, 0);
    for f in &files {
        let data = std::fs::read(f).unwrap();
        ours = ours.max(peak_stack::<Mbop3>(&data));
        c = c.max(peak_stack::<RefI16>(&data));
    }
    let create = stack::measure(|| {
        std::hint::black_box(Box::new(mbop3::Decoder::new()));
    });
    println!("stack to create Box<Decoder>: {create} bytes");
    println!("peak stack over {} files (host, x86_64):", files.len());
    println!("  mbop3   {ours} bytes");
    println!("  minimp3 {c} bytes");
    0
}

fn sizes() -> i32 {
    println!(
        "size_of::<mbop3::Decoder>() = {}",
        std::mem::size_of::<mbop3::Decoder>()
    );
    println!(
        "minimp3 sizeof(mp3dec_t) = {}",
        mbop3_reference::i16::Decoder::decoder_size()
    );
    println!(
        "minimp3 sizeof(mp3dec_scratch_t) (on the stack in every decode) = {}",
        mbop3_reference::i16::Decoder::scratch_size()
    );
    0
}
