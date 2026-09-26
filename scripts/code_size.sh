#!/usr/bin/env bash
# Code size (text + rodata + data) of mbop3 and C minimp3 for the ESP32-S3.
# Output: "mbop3_O3 N", "mbop3_Os N", "minimp3_O2 N", "minimp3_Os N" in bytes.
set -euo pipefail
cd "$(dirname "$0")/.."
target=xtensa-esp32s3-none-elf
size=xtensa-esp-elf-size
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

measure_lib() { # staticlib: sum sections of every member except compiler_builtins
  local lib=$1 total=0
  (cd "$tmp" && rm -f ./*.o && ar x "$lib")
  for o in "$tmp"/*.o; do
    case "$(basename "$o")" in compiler_builtins*) continue ;; esac
    local n
    n=$($size -A "$o" | awk '$1 ~ /^\.(text|literal|rodata|data)/ {s += $2} END {print s + 0}')
    total=$((total + n))
  done
  echo "$total"
}

for profile in release release-s; do
  (cd sizecheck && cargo +esp build -q --profile "$profile" --target $target -Zbuild-std=core 2>&1 | grep -v "^warning" || true)
  dir=$profile; [ "$profile" = release ] && name=mbop3_O3 || name=mbop3_Os
  echo "$name $(measure_lib "$PWD/sizecheck/target/$target/$dir/libmbop3_sizecheck.a")"
done

for opt in O2 Os; do
  cat > "$tmp/ref.c" <<'C'
#define MINIMP3_IMPLEMENTATION
#define MINIMP3_ONLY_MP3
#define MINIMP3_NO_SIMD
#include "minimp3.h"
C
  xtensa-esp-elf-gcc -mlongcalls -"$opt" -c -I reference/minimp3 "$tmp/ref.c" -o "$tmp/ref_c.o"
  echo "minimp3_$opt $($size -A "$tmp/ref_c.o" | awk '$1 ~ /^\.(text|literal|rodata|data)/ {s += $2} END {print s + 0}')"
done
