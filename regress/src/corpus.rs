//! Finding corpus files and generating deterministic mutations of them.

use std::path::{Path, PathBuf};

pub fn is_mp3(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .as_deref(),
        Some("mp3" | "bit" | "mp2" | "mpa")
    )
}

/// All mp3 files under `dir`, sorted for determinism.
pub fn find(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&d) else {
            continue;
        };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if is_mp3(&p) {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// FNV-1a, used to pick deterministic subsets and seed mutations.
pub fn hash(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Deterministically keeps about `n` of `files`, spread across the list.
pub fn sample(files: Vec<PathBuf>, n: usize) -> Vec<PathBuf> {
    if files.len() <= n {
        return files;
    }
    let mut keyed: Vec<_> = files
        .into_iter()
        .map(|p| (hash(p.to_string_lossy().as_bytes()), p))
        .collect();
    keyed.sort();
    let mut out: Vec<_> = keyed.into_iter().take(n).map(|(_, p)| p).collect();
    out.sort();
    out
}

pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed | 1)
    }
    pub fn next(&mut self) -> u64 {
        // xorshift64*
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545F4914F6CDD1D)
    }
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next() % n as u64) as usize
        }
    }
}

/// A corrupted variant of `data`. `other` is a second file for splicing.
pub fn mutate(data: &[u8], other: &[u8], seed: u64) -> (String, Vec<u8>) {
    let mut rng = Rng::new(seed);
    let mut d = data.to_vec();
    let len = d.len().max(1);
    match rng.below(6) {
        0 => {
            let at = rng.below(len);
            d.truncate(at);
            (format!("truncate@{at}"), d)
        }
        1 => {
            let flips = 1 + rng.below(16);
            for _ in 0..flips {
                let i = rng.below(len);
                if i < d.len() {
                    d[i] ^= 1 << rng.below(8);
                }
            }
            (format!("bitflip x{flips}"), d)
        }
        2 => {
            let at = rng.below(len);
            let n = 1 + rng.below(512);
            for i in at..(at + n).min(d.len()) {
                d[i] = rng.next() as u8;
            }
            (format!("garbage@{at}+{n}"), d)
        }
        3 => {
            let at = rng.below(len);
            let n = 1 + rng.below(4096);
            let junk: Vec<u8> = (0..n).map(|_| rng.next() as u8).collect();
            d.splice(at.min(d.len())..at.min(d.len()), junk);
            (format!("insert@{at}+{n}"), d)
        }
        4 => {
            let at = rng.below(len);
            let from = rng.below(other.len().max(1));
            d.truncate(at);
            d.extend_from_slice(&other[from.min(other.len())..]);
            (format!("splice@{at}<-{from}"), d)
        }
        _ => {
            // Corrupt only header-ish bytes: those right after an 0xFF sync byte.
            let mut n = 0;
            for i in 0..d.len().saturating_sub(3) {
                if d[i] == 0xff && rng.below(64) == 0 {
                    let j = i + 1 + rng.below(3);
                    d[j] ^= 1 << rng.below(8);
                    n += 1;
                }
            }
            (format!("headerflip x{n}"), d)
        }
    }
}
