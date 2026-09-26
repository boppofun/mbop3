#!/usr/bin/env bash
# Stack frame sizes (bytes, from each function's `entry a1, N`) of the decoder
# built for the ESP32-S3 by sizecheck. The deepest call chain is roughly
# decode (the exported fn) + decode_granule + the largest leaf.
set -euo pipefail
cd "$(dirname "$0")/.."
target=xtensa-esp32s3-none-elf
profile=${1:-release}
(cd sizecheck && cargo +esp build -q --profile "$profile" --target $target -Zbuild-std=core 2>&1 | grep -v "^warning" || true)
dir=$profile
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
(cd "$tmp" && ar x "$OLDPWD/sizecheck/target/$target/$dir/libmbop3_sizecheck.a")
for o in "$tmp"/*.o; do
  case "$(basename "$o")" in compiler_builtins*) continue ;; esac
  xtensa-esp-elf-objdump -d --no-show-raw-insn -C "$o" |
    awk '/^[0-9a-f]+ <.*>:$/ {fn=$2} /entry\ta1, / {split($0, a, "entry\ta1, "); printf "%d %s\n", strtonum(a[2]), fn}'
done | sort -rn | grep -v "core::" | head -${2:-12}
