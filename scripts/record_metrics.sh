#!/usr/bin/env bash
# Measures the current working copy and appends a row to metrics/history.tsv.
set -euo pipefail
cd "$(dirname "$0")/.."
regress=target/release/mbop3-regress
hist=metrics/history.tsv
boppo=${BOPPO_CORPUS:-$HOME/Projects/boppo/device_files/build/sd}

if [ ! -f "$hist" ]; then
  printf "date\tchange\tdescription\tcheck\tdecoder_bytes\thost_stack\thost_stack_c\thost_time_ratio_vs_c\txtensa_O3\txtensa_Os\tc_xtensa_Os\tunsafe\n" > "$hist"
fi

change=$(jj log -r @ --no-graph -T 'change_id.short(8)' 2>/dev/null || echo "?")
desc=$(jj log -r @ --no-graph -T 'description.first_line()' 2>/dev/null || echo "")
[ -z "$desc" ] && desc=$(jj log -r @- --no-graph -T 'description.first_line()' 2>/dev/null || echo "")

check=FAIL
just check > /tmp/mbop3_check.$$ 2>&1 && check=PASS
dec=$($regress sizes | awk -F'= ' '/mbop3::Decoder/ {print $2}')
stack_out=$($regress stack corpus/external/minimp3/vectors corpus/generated)
stack=$(echo "$stack_out" | awk '/mbop3 / {print $2}')
stack_c=$(echo "$stack_out" | awk '/minimp3 / {print $2}')
ratio=$($regress bench --reps 7 corpus/generated "$boppo:150" | awk '/time ratio/ {print $NF}')
sizes=$(scripts/code_size.sh)
o3=$(echo "$sizes" | awk '/mbop3_O3/ {print $2}')
os=$(echo "$sizes" | awk '/mbop3_Os/ {print $2}')
cos=$(echo "$sizes" | awk '/minimp3_Os/ {print $2}')
unsafe=$(grep -rn "unsafe" mbop3/src | grep -v "^\S*:\s*//" | wc -l)

row=$(printf "%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s" "$(date -Iseconds)" "$change" "$desc" "$check" "$dec" "$stack" "$stack_c" "$ratio" "$o3" "$os" "$cos" "$unsafe")
echo "$row" >> "$hist"
column -t -s $'\t' "$hist" | tail -n 3
[ "$check" = PASS ] || { echo "regression check FAILED, see /tmp/mbop3_check.$$"; exit 1; }
rm -f /tmp/mbop3_check.$$
