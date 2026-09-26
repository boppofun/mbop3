# Device measurements (BenDev, ESP32-S3 @ 240 MHz, Boppo firmware)

`mbop3_decode_perf_test` / `mp3_decode_perf_test` (rmp3 = C minimp3): decode 48000 samples
(1 s) of `faces/farm/facts/en-US/cow_1.mp3` (48 kHz mono, 96 kbps) from RAM through awedio.
Stack = drop in the high water mark of a fresh 48 KB thread (`... stack` mode).

| date | mbop3 change | mbop3 decode 1 s | rmp3 decode 1 s | mbop3 stack | rmp3 stack | mbop3 heap | rmp3 heap |
|---|---|---|---|---|---|---|---|
| 2026-09-27 | swtvrxmv (ring buffer, mono synth) | 0.122 s | 0.138 s | 16.4 KB (Box::default builds Decoder on stack) | 21.7 KB | 19.5 KB | 13.5 KB |
| 2026-09-27 | lvuyvuwx (new_boxed) | 0.118 s | 0.134 s | < 3 KB (below the test's own logging) | 18.8 KB | 19.3 KB | 13.5 KB |
| 2026-09-27 | (profile, bounds provable synth) | 0.107-0.117 s | 0.134-0.141 s | < 3 KB | 18.8 KB | 19.3 KB | 13.5 KB |

Heap includes awedio's 4.6 KB output and 2 KB input buffers for both decoders.

## Profile (mbop3 "profile" feature, ms per 1 s of audio)

| change | side_info | scalefactors | huffman | stereo | reorder+aa | imdct | dct_ii | synth |
|---|---|---|---|---|---|---|---|---|
| lvuyvuwx+profile | 4.0 | 6.6 | 13.1 | 0.2 | 4.8 | 14.8 | 15.2 | 64.6 |
| bounds provable synth | 3.5 | 4.3 | 11.1 | 0.1 | 4.2 | 12.7 | 12.1 | 44.1 |

## Output hashes (FNV-1a of samples 1..48001 of cow_1.mp3)

Host (mbop3 and C minimp3, bit-exact): `6d229e4f99c2bf58`. On the device both decoders
differ from the host in the low bits because the Xtensa compilers fuse multiply-adds into
`madd.s` (gcc by default; LLVM's Xtensa backend too): mbop3 `0b81ecc2e1320f44`, rmp3
`ef3abb297772905c`. The device hash is a useful check that a change did not alter the
Xtensa output.
