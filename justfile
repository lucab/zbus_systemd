@default: generate fmt check

check:
  cargo check --all-features

doc:
  cargo doc --all-features

fmt:
  cargo fmt

generate:
  cargo run --manifest-path ./codegen/Cargo.toml --locked

current_sd_tag := 'v249'
bump-systemd sd_tag=current_sd_tag:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Updating git-module 'codegen/systemd' to '{{sd_tag}}'."
  git submodule update --init -- codegen/systemd
  cd codegen/systemd
  git fetch -q origin "{{sd_tag}}"
  git checkout "{{sd_tag}}"
