# mbop3 development commands. `just` lists them.

minimp3_commit := `cat reference/minimp3/COMMIT`
# Boppo's device files: real-world 48 kHz mono speech/music mp3s (local only, not redistributable).
boppo_corpus := env_var_or_default("BOPPO_CORPUS", env_var("HOME") / "Projects/boppo/device_files/build/sd")
regress := "target/release/mbop3-regress"
# Built with mbop3's "exact" feature (minimp3's float evaluation order).
regress_exact := "target/exact/release/mbop3-regress"
tiers := "iso=corpus/external/minimp3/vectors generated=corpus/generated"

default:
    @just --list

# Download the minimp3 test vectors (ISO conformance + minimp3's own) and generate the synthetic corpus.
corpus: fetch-vectors gen-corpus

fetch-vectors:
    #!/usr/bin/env bash
    set -euo pipefail
    d=corpus/external/minimp3
    if [ ! -d $d ]; then git clone -q https://github.com/lieff/minimp3 $d; fi
    git -C $d checkout -q {{minimp3_commit}}
    echo "minimp3 vectors at {{minimp3_commit}}: $(ls $d/vectors/*.bit | wc -l) files"

gen-corpus:
    scripts/gen_corpus.sh corpus/generated

build:
    cargo build --release --workspace
    cargo build --release -p mbop3-regress --features exact --target-dir target/exact

# Unit tests plus the fast regression check. Run before every commit.
test: build
    cargo test --release --workspace -q
    just check

# Per-commit regression, in two passes over the ISO vectors, the generated corpus,
# 300 Boppo files and 10 mutations per file (plus ISO .pcm compliance):
#  1. the "exact" build must be bit-identical to C minimp3;
#  2. the default (fast) build must be within the ISO 11172-4 full accuracy
#     limits of C minimp3.
check: build
    {{regress_exact}} check --criteria exact --mutations 10 {{tiers}} boppo={{boppo_corpus}}:300
    {{regress}} check --criteria iso --mutations 10 {{tiers}} boppo={{boppo_corpus}}:300

# Everything: the whole Boppo corpus and 100 mutations per file.
check-full: build
    {{regress_exact}} check --criteria exact --mutations 100 {{tiers}} boppo={{boppo_corpus}}
    {{regress}} check --criteria iso --mutations 100 {{tiers}} boppo={{boppo_corpus}}

# Host decode speed of mbop3 vs C minimp3.
bench: build
    {{regress}} bench --reps 7 corpus/generated {{boppo_corpus}}:150

# Host peak stack of mbop3 vs C minimp3.
stack: build
    {{regress}} stack corpus/external/minimp3/vectors corpus/generated

sizes: build
    {{regress}} sizes

# Code size of the decoder for the ESP32-S3 (Xtensa), in bytes.
code-size:
    scripts/code_size.sh

# Stack, memory, speed and code size summary, appended to metrics/history.tsv with the current jj change.
record: build
    scripts/record_metrics.sh

fmt:
    cargo fmt --all

clippy:
    cargo clippy --workspace --release -- -D warnings

# Count of `unsafe` in the decoder crate (excluding the dev-only profile feature).
unsafe-count:
    @grep -rn "unsafe" mbop3/src --exclude=profile.rs | grep -v "(unsafe_code)" | wc -l

# Differential fuzzing against C minimp3 (needs cargo-fuzz and nightly). `exact` compares
# PCM bit for bit; `fast` fuzzes the default build and compares frame structure.
fuzz mode="exact" secs="300":
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p fuzz/corpus/differential
    cp -n corpus/external/minimp3/vectors/*.bit corpus/generated/*.mp3 fuzz/corpus/differential/ 2>/dev/null || true
    flags=""; [ "{{mode}}" = fast ] && flags="--no-default-features"
    cd fuzz && cargo +nightly fuzz run $flags differential corpus/differential -- -max_total_time={{secs}} -max_len=65536

# Release (release-plz, run locally: the checks need the Boppo corpus).
#   1. just release-prepare   checks, then bump the version and update mbop3/CHANGELOG.md
#   2. edit the changelog
#   3. just release-commit    commit, move main to it and push
#   4. just release-execute   checks again, publish to crates.io, tag and push the tag

# A no_std target without an allocator, to catch stray std (or alloc) use.
nostd_target := "riscv32imc-unknown-none-elf"

release-checks: build
    cargo fmt --all --check
    just clippy
    cargo clippy -p mbop3 --features alloc,exact --target {{nostd_target}} -- -D warnings
    cargo build -p mbop3 --release --target {{nostd_target}}
    cargo build -p mbop3 --release --target {{nostd_target}} --features alloc
    cargo test --release --workspace -q
    cargo test --release -p mbop3 -q --features alloc,exact
    RUSTDOCFLAGS="-D warnings" cargo doc -p mbop3 --no-deps --features alloc
    just check-full
    cargo publish -p mbop3 --dry-run

# @ must be empty and @- must be main.
_require-main:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "$(jj log -r main -T commit_id --no-graph)" != "$(jj log -r @- -T commit_id --no-graph)" ]; then
        echo "@- is not main"; exit 1
    fi
    if [ -n "$(jj diff --summary)" ]; then
        echo "the working copy has changes"; exit 1
    fi

release-prepare: _require-main release-checks
    release-plz update
    @echo "Now edit mbop3/CHANGELOG.md, then run just release-commit"

release-commit:
    #!/usr/bin/env bash
    set -euo pipefail
    version=$(cargo metadata --format-version=1 --no-deps | jq -r '.packages[] | select(.name == "mbop3") | .version')
    jj commit -m "chore(release): v$version"
    jj bookmark set main -r @-
    jj git push -b main

_require-main-pushed:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "$(jj log -r main -T commit_id --no-graph)" != "$(jj log -r main@origin -T commit_id --no-graph)" ]; then
        echo "main is not pushed"; exit 1
    fi

release-execute-dry-run: _require-main _require-main-pushed release-checks
    release-plz release --dry-run

release-execute: _require-main _require-main-pushed release-checks
    release-plz release
