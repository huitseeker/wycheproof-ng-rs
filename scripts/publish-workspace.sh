#!/usr/bin/env bash
set -euo pipefail

mode="${1:-}"
case "${mode}" in
  --dry-run | --publish)
    shift
    ;;
  *)
    echo "usage: $0 (--dry-run|--publish) [cargo publish args...]" >&2
    exit 2
    ;;
esac

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"

packages=(
  wycheproof-ng-core
  wycheproof-ng-aead
  wycheproof-ng-symmetric
  wycheproof-ng-fpe
  wycheproof-ng-ecdsa
  wycheproof-ng-dh
  wycheproof-ng-dsa
  wycheproof-ng-eddsa
  wycheproof-ng-bls
  wycheproof-ng-rsa-encryption
  wycheproof-ng-rsa-signature
  wycheproof-ng-mlkem
  wycheproof-ng-mldsa
  wycheproof-ng-kdf-jose
  wycheproof-ng
)

families=(
  wycheproof-ng-aead
  wycheproof-ng-symmetric
  wycheproof-ng-fpe
  wycheproof-ng-ecdsa
  wycheproof-ng-dh
  wycheproof-ng-dsa
  wycheproof-ng-eddsa
  wycheproof-ng-bls
  wycheproof-ng-rsa-encryption
  wycheproof-ng-rsa-signature
  wycheproof-ng-mlkem
  wycheproof-ng-mldsa
  wycheproof-ng-kdf-jose
)

cd "${repo_root}"

dry_run_patch_args() {
  local package="$1"

  case "${package}" in
    wycheproof-ng-core)
      return 0
      ;;
    wycheproof-ng)
      printf '%s\n' --config 'patch.crates-io.wycheproof-ng-core.path="crates/core"'
      for family in "${families[@]}"; do
        printf '%s\n' --config "patch.crates-io.${family}.path=\"crates/${family#wycheproof-ng-}\""
      done
      ;;
    *)
      printf '%s\n' --config 'patch.crates-io.wycheproof-ng-core.path="crates/core"'
      ;;
  esac
}

if [[ "${mode}" == "--dry-run" ]]; then
  for package in "${packages[@]}"; do
    patch_args=()
    while IFS= read -r arg; do
      patch_args+=("${arg}")
    done < <(dry_run_patch_args "${package}")
    if ((${#patch_args[@]})); then
      cargo publish -p "${package}" --dry-run --allow-dirty "${patch_args[@]}" "$@"
    else
      cargo publish -p "${package}" --dry-run --allow-dirty "$@"
    fi
  done
  exit 0
fi

crate_version() {
  cargo pkgid -p "$1" | sed 's/.*@//'
}

version_status() {
  local package="$1"
  local version="$2"

  curl -sS \
    -o /dev/null \
    -w '%{http_code}' \
    -A "wycheproof-ng-rs-release-script" \
    --retry 3 \
    --connect-timeout 15 \
    --max-time 60 \
    "https://crates.io/api/v1/crates/${package}/${version}" \
    || return 1
}

version_state() {
  local package="$1"
  local version="$2"
  local status

  status="$(version_status "${package}" "${version}")" || {
    echo "failed to query crates.io for ${package}@${version}" >&2
    return 2
  }

  case "${status}" in
    200)
      echo "exists"
      ;;
    404)
      echo "missing"
      ;;
    *)
      echo "unexpected crates.io status for ${package}@${version}: ${status}" >&2
      return 2
      ;;
  esac
}

wait_for_version() {
  local package="$1"
  local version="$2"
  local attempts="${PUBLISH_INDEX_WAIT_ATTEMPTS:-30}"
  local sleep_seconds="${PUBLISH_INDEX_SETTLE_SECONDS:-20}"
  local state

  for _ in $(seq 1 "${attempts}"); do
    state="$(version_state "${package}" "${version}")" || return 1
    if [[ "${state}" == "exists" ]]; then
      return 0
    fi
    sleep "${sleep_seconds}"
  done

  echo "${package}@${version} did not become visible on crates.io" >&2
  return 1
}

for package in "${packages[@]}"; do
  version="$(crate_version "${package}")"
  state="$(version_state "${package}" "${version}")" || exit 1
  if [[ "${state}" == "exists" ]]; then
    echo "${package}@${version} already exists on crates.io; skipping"
    continue
  fi

  cargo publish -p "${package}" "$@"
  wait_for_version "${package}" "${version}"
done
