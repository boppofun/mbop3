#!/usr/bin/env bash
# Generates the synthetic regression corpus with sox + lame. Deterministic for a
# given sox/lame version (recorded in VERSIONS). The output is not committed.
set -euo pipefail
out=${1:-corpus/generated}
rm -rf "$out"
mkdir -p "$out/wav"
tmp="$out/wav"

sox --version > "$out/VERSIONS"
lame --version | head -1 >> "$out/VERSIONS"

# signal name -> sox synth args (stereo unless noted). -R makes noise repeatable.
declare -A sig=(
  [sweep]="synth 4 sine 20-20000 sine 20000-20"
  [bursts]="synth 4 whitenoise synth 4 square amod 3"
  [stereo]="synth 4 sine 440 sine 660 synth 4 pinknoise mix"
  [square_fs]="synth 2 square 440 square 150"
  [silence]="synth 2 sine 0 vol 0"
  [tones]="synth 3 sine 1000 sine 1000 vol 0.9"
  [pink]="synth 3 pinknoise pinknoise"
)

render() { # name rate
  local f="$tmp/$1_$2.wav"
  [ -f "$f" ] || sox -R -n -r "$2" -c 2 -b 16 "$f" ${sig[$1]} 2>/dev/null
  echo "$f"
}

enc() { # outname rate signal lame-args...
  local name=$1 rate=$2 s=$3; shift 3
  lame --quiet --resample $(awk "BEGIN{print $rate/1000}") "$@" "$(render "$s" "$rate")" "$out/$name.mp3"
}

rates="8000 11025 12000 16000 22050 24000 32000 44100 48000"
for r in $rates; do
  for s in sweep bursts stereo; do
    enc "cbr_j_${s}_${r}" "$r" "$s" -m j
    enc "cbr_m_${s}_${r}" "$r" "$s" -m m
    enc "vbr0_${s}_${r}" "$r" "$s" -V 0
    enc "vbr9_${s}_${r}" "$r" "$s" -V 9 -m m
  done
done

for s in sweep bursts stereo square_fs silence tones pink; do
  for m in s j f d m; do enc "mode_${m}_${s}" 44100 "$s" -m "$m" -b 128; done
  enc "b320_${s}" 48000 "$s" -b 320
  enc "b8_${s}" 8000 "$s" -b 8 -m m
  enc "abr64_${s}" 32000 "$s" --abr 64
done

for s in bursts stereo sweep; do
  enc "nores_${s}" 44100 "$s" --nores -b 128
  enc "crc_${s}" 44100 "$s" -p -b 128
  enc "q0_${s}" 48000 "$s" -q 0 -b 96 -m m
  enc "q9_${s}" 48000 "$s" -q 9 -b 96 -m m
  enc "iso_${s}" 44100 "$s" --strictly-enforce-ISO -b 192
  enc "notag_${s}" 22050 "$s" -t -b 64
  enc "id3v2_${s}" 44100 "$s" -b 128 --add-id3v2 --tt title --ta artist --tc comment
  enc "id3v1_${s}" 44100 "$s" -b 128 --id3v1-only --tt title
  enc "free640_${s}" 48000 "$s" --freeformat -b 640
  enc "free32_${s}" 32000 "$s" --freeformat -b 40 -m m
  # Boppo-like: 48 kHz mono speech rate
  enc "boppo_${s}" 48000 "$s" -m m -b 96
done

# Streams whose format changes mid-stream (concatenated mp3s).
cat "$out/cbr_m_sweep_48000.mp3" "$out/cbr_j_bursts_44100.mp3" "$out/vbr9_stereo_8000.mp3" > "$out/concat_rates_modes.mp3"
cat "$out/mode_m_tones.mp3" "$out/mode_s_tones.mp3" "$out/mode_m_tones.mp3" > "$out/concat_mono_stereo.mp3"

rm -rf "$tmp"
echo "generated $(ls "$out"/*.mp3 | wc -l) files in $out ($(du -sh "$out" | cut -f1))"
