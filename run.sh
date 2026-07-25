#!/usr/bin/env bash
# Interactive runner for the project's cargo binaries.
# Pick a binary by number; you'll be prompted for arguments when needed.

set -euo pipefail

# name -> needs-args (1 = requires an argument, 0 = no arguments)
binaries=(
    "get_subscription:1"
    "delete_subscription:1"
    "update_subscription:1"
    "show_subscriptions:0"
    "write_subscription:0"
)

echo "Available binaries:"
for i in "${!binaries[@]}"; do
    name="${binaries[$i]%%:*}"
    needs="${binaries[$i]##*:}"
    hint=""
    [[ "$needs" == "1" ]] && hint=" (requires args)"
    printf '  %d] %s%s\n' "$((i + 1))" "$name" "$hint"
done

echo
read -rp "Select a binary to run (1-${#binaries[@]}): " choice

if ! [[ "$choice" =~ ^[0-9]+$ ]] || (( choice < 1 || choice > ${#binaries[@]} )); then
    echo "ERROR: Invalid selection" >&2
    exit 1
fi

entry="${binaries[$((choice - 1))]}"
name="${entry%%:*}"
needs="${entry##*:}"

args=()
if [[ "$needs" == "1" ]]; then
    read -rp "Enter arguments for '$name' (e.g. subscription id): " user_args
    # shellcheck disable=SC2206
    args=($user_args)
fi

echo
echo "Running: cargo run --bin $name -- ${args[*]:-}"
cargo run --bin "$name" -- ${args[@]+"${args[@]}"}
