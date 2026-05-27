#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 3 ]]; then
  echo "usage: $0 <generated-notes-json> <cargo-dist-body> <output-notes>" >&2
  exit 2
fi

generated_notes_json="$1"
cargo_dist_body="$2"
output_notes="$3"

jq -r '.body // ""' "$generated_notes_json" > "$output_notes"

release_tag=""
if release_tag="$(
  grep -Eo 'https://github\.com/kubeply/rudolint/releases/download/[^/[:space:]]+' "$cargo_dist_body" \
    | head -n 1 \
    | sed -E 's#.*/download/##'
)"; then
  :
else
  release_tag=""
fi

if [[ -s "$cargo_dist_body" ]]; then
  {
    echo
    echo "## Install, Downloads, Checksums, And Attestations"
    echo
    sed -E \
      's#https://github\.com/kubeply/rudolint/releases/download/([^/[:space:]]+)/rudolint-installer\.sh#https://kubeply.com/rudolint/\1/install.sh#g' \
      "$cargo_dist_body"
  } >> "$output_notes"
fi

if [[ -n "$release_tag" ]]; then
  major_tag="${release_tag%%.*}"
  {
    echo
    echo "## Container Image"
    echo
    echo '```sh'
    printf '%s\n' "docker run --rm -v \"\$PWD:/workspace\" ghcr.io/kubeply/rudolint check /workspace"
    echo '```'
    echo
    echo "The container image is published as:"
    echo
    echo "- \`ghcr.io/kubeply/rudolint:${release_tag}\`"
    echo "- \`ghcr.io/kubeply/rudolint:${major_tag}\`"
    echo "- \`ghcr.io/kubeply/rudolint:latest\`"
  } >> "$output_notes"
fi
