# mbop3

A small `no_std` MP3 (MPEG-1/2/2.5 Layer III) decoder in Rust, derived from
[minimp3](https://github.com/lieff/minimp3) (CC0) via a c2rust translation, then
cleaned up and tuned for microcontrollers (the ESP32-S3 in particular).

Priorities, in order: stack use, memory, CPU, code size.

## Layout

| Path | What |
|---|---|
| `mbop3/` | The decoder crate (`no_std`, no alloc). |
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
each frame's info and PCM. Every file is run through two input drivers: the whole file in one
buffer, and a 2048 byte window that is topped up before each call (how awedio drives the decoder).

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

Criteria:

- `just check` (default, `exact`): every sample must be bit-identical to C minimp3.
- `just check iso`: ISO 11172-4 "full accuracy" limits relative to C minimp3
  (RMS error < 2^-15/sqrt(12) of full scale, max error 2^-14), with frame info still exact.
  For optimizations that change floating point evaluation order.

### Reference build notes

minimp3 reads its uninitialized stack scratch buffer on some corrupt streams (MSan finds it in
`L3_huffman`), so its output on those streams depends on stack garbage. The reference is built
with `-ftrivial-auto-var-init=zero`, which makes it deterministic and matches the translation
(c2rust zero-initializes locals). It is also built with `-ffp-contract=off` so C float math is
plain IEEE single precision like Rust's.

## License

MIT OR Apache-2.0. minimp3 (and the vendored copy in `reference/minimp3`) is CC0 1.0.
