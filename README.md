# mbop3

A small `no_std` MP3 (MPEG-1/2/2.5 Layer III) decoder in Rust, derived from
[minimp3](https://github.com/lieff/minimp3) (CC0) via a c2rust translation, then
cleaned up and tuned for microcontrollers (the ESP32-S3 in particular).

Priorities, in order: stack use, memory, CPU, code size.

The decoder is 100% safe Rust: the crate is `#![forbid(unsafe_code)]` (the only exception is
the development-only `profile` feature). Corrupt or malicious input cannot cause memory
unsafety, unlike C minimp3, which reads uninitialized memory on some corrupt streams (see
[Reference build notes](#reference-build-notes)). It is fuzzed differentially against
C minimp3 on arbitrary input (see [Fuzzing](#fuzzing)).

Pronounced mmm-bop-3.

## Layout

| Path | What |
|---|---|
| `mbop3/` | The decoder crate (`no_std`, needs no allocator; the optional `alloc` feature adds `Decoder::new_boxed()`). |
| `c2rust/` | The unedited c2rust output of minimp3. The first commit used it directly; it was then replaced by the safe port in `mbop3/src`, and is kept for reference. |
| `reference/` | The original C minimp3 at a pinned commit (`reference/minimp3/COMMIT`), built with the same configuration (Layer 3 only, no SIMD) in int16 and float variants. The golden reference. |
| `regress/` | `mbop3-regress`, the regression harness (see below). |
| `sizecheck/` | Xtensa staticlib used to measure code size. |
| `scripts/` | Corpus generation and metrics scripts. |
| `metrics/history.tsv` | Metrics recorded after each commit (`just record`). |

## Regression testing

```sh
just corpus        # fetch minimp3's test vectors + generate the synthetic corpus (once)
just check         # per-commit regression (a few seconds)
just check-full    # the whole Boppo corpus and 100 mutations per file
just record        # check + measure stack/memory/speed/code size, append to metrics/history.tsv
```

`just check` decodes every corpus file with both mbop3 and C minimp3 in lockstep and compares
each frame's info and PCM. Every file is run through three input drivers: the whole file in one
buffer, a 2048 byte window that is topped up before each call (how awedio drives the decoder),
and the whole file with no PCM buffer (parse only).

Corpus tiers:

- **iso**: minimp3's `vectors/` (ISO/IEC 11172-4 and 13818-4 conformance bitstreams plus
  minimp3's own nonstandard/robustness cases). Fetched at the pinned commit, not committed.
  These also get a compliance check against their reference `.pcm` with minimp3_test's
  criteria (PSNR >= 96 dB, sample count). 5 vectors fail that check for both C minimp3 and
  mbop3 because they test `mp3dec_ex` features (VBR tag skipping, gapless trimming) that the
  frame API doesn't do. They are reported as `known`, and only regressions relative to C fail.
- **generated**: `scripts/gen_corpus.sh` (sox + lame): every MPEG-1/2/2.5 sample rate, mono,
  stereo, joint/forced mid-side, dual channel, CBR 8-640 kbps, VBR, ABR, free format, CRC,
  no bit reservoir, ID3v1/v2 tags, format changes mid-stream, full scale square waves (clipping).
- **boppo**: Boppo's device files (48 kHz mono, `$BOPPO_CORPUS`, default
  `~/Projects/boppo/device_files/build/sd`). Local only.
- **mutated**: deterministic corruptions of the iso and generated files (truncation, bit flips,
  garbage, inserted junk, splices of two files, header bit flips), generated in memory.

Criteria (`just check` runs both):

1. **exact**: built with mbop3's `exact` feature, which evaluates floating point in exactly
   minimp3's order. Every sample must be bit-identical to C minimp3, for i16 and f32 output,
   on every tier including the mutated one.
2. **iso**: the default (fast) build, which reorders some floating point sums for speed on the
   ESP32-S3. Output must be within the ISO/IEC 11172-4 "full accuracy" limits of C minimp3
   (RMS error < 2^-15/sqrt(12) of full scale, every sample within 2^-14, i.e. 2 LSB at 16 bit),
   with frame info exactly equal. For mutated and nonstandard/ILL streams (corrupt or out of
   spec, with values far beyond full scale) only the frame structure must match.

Note that neither C minimp3 nor mbop3 is bit-exact on the ESP32-S3 itself: both gcc and the
Xtensa LLVM backend fuse multiply-adds into `madd.s`. See `metrics/device.md`.

### Reference build notes

minimp3 reads its uninitialized stack scratch buffer on some corrupt streams (MSan finds it in
`L3_huffman`), so its output on those streams depends on stack garbage. The reference is built
with `-ftrivial-auto-var-init=zero`, which makes it deterministic and matches the translation
(c2rust zero-initializes locals). It is also built with `-ffp-contract=off` so C float math is
plain IEEE single precision like Rust's.

## Fuzzing

`just fuzz exact 600` runs a cargo-fuzz differential target (nightly) that decodes arbitrary
input with mbop3 and C minimp3 (whole input and a small window) and requires bit-identical
output; `just fuzz fast 600` fuzzes the default build and compares frame structure.

## LLM Generated

This code was virtually entirely generated by an LLM.

## License

MIT OR Apache-2.0. minimp3 (and the vendored copy in `reference/minimp3`) is CC0 1.0.
