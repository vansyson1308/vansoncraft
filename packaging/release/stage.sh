#!/usr/bin/env bash
# Check the release files in ASSETS, then write beside them SHA256SUMS.txt and BUILDINFO.json, and
# (with NOTES) the release notes. Used by .github/workflows/release.yml.
#
#   packaging/release/stage.sh ASSETS VERSION COMMIT RUN_URL TOOLCHAIN [NOTES PRERELEASE]
#
# Fails when a file the release must have is missing or empty, or when ASSETS holds anything else.
set -euo pipefail
assets="$1" v="$2" sha="$3" run="$4" toolchain="$5" notes="${6:-}" prerelease="${7:-false}"

expected=(
  "vansoncraft-$v-windows-x64.msi"
  "vansoncraft-$v-windows-x64-portable.zip"
  "vansoncraft-$v-linux-x86_64.tar.gz"
  "vansoncraft-$v-linux-x86_64.AppImage"
)
optional=("vansoncraft-$v-linux-x86_64.AppImage.zsync")
cd "$assets"
for f in "${expected[@]}"; do
  [[ -s "$f" ]] || { echo "::error::missing or empty: $f"; exit 1; }
done
for f in *; do
  case " ${expected[*]} ${optional[*]} " in
    *" $f "*) ;;
    *) echo "::error::unexpected file in the release: $f"; exit 1 ;;
  esac
done

files=(*)
sha256sum -- "${files[@]}" > SHA256SUMS.txt
python3 - "$v" "$sha" "$run" "$toolchain" "${files[@]}" <<'EOF'
import hashlib, json, sys
v, sha, run, toolchain, *names = sys.argv[1:]
kinds = {
    f"vansoncraft-{v}-windows-x64.msi": ("windows", "x64", "MSI installer, per-user by default (app and CLI)", "windows-2025"),
    f"vansoncraft-{v}-windows-x64-portable.zip": ("windows", "x64", "portable ZIP (app and CLI)", "windows-2025"),
    f"vansoncraft-{v}-linux-x86_64.tar.gz": ("linux", "x86_64", "tar.gz, bin/ and share/ (app and CLI), glibc 2.35+", "ubuntu-22.04"),
    f"vansoncraft-{v}-linux-x86_64.AppImage": ("linux", "x86_64", "AppImage (app and CLI), glibc 2.35+", "ubuntu-22.04"),
    f"vansoncraft-{v}-linux-x86_64.AppImage.zsync": ("linux", "x86_64", "AppImage delta-update data", "ubuntu-22.04"),
}
files = []
for name in names:
    data = open(name, "rb").read()
    os_, arch, kind, host = kinds[name]
    files.append({"name": name, "os": os_, "arch": arch, "kind": kind, "built_on": host, "size": len(data),
                  "sha256": hashlib.sha256(data).hexdigest(), "signed": False})
json.dump({"product": "VansonCraft", "version": v, "commit": sha, "workflow_run": run, "rust": toolchain, "files": files},
          open("BUILDINFO.json", "w"), indent=1)
EOF
cat SHA256SUMS.txt

[[ -n "$notes" ]] || exit 0
repo_url="${GITHUB_SERVER_URL:-https://github.com}/${GITHUB_REPOSITORY:-vansyson1308/vansoncraft}"
{
  echo "VansonCraft $v for Windows x64 and Linux x86_64, built from [\`${sha:0:12}\`]($repo_url/commit/$sha) by [this workflow run]($run) with Rust $toolchain."
  echo
  if [[ "$prerelease" == "true" ]]; then
    echo "**Pre-release.** It is not this repository's \"latest\" release, and the in-app updater does not offer it."
    echo
  fi
  echo "**Unsigned.** These files carry no code signature. Windows SmartScreen may warn that the publisher is unknown, and on Windows the in-app updater gives a download link rather than installing an unsigned update itself. Check a download against \`SHA256SUMS.txt\`: \`Get-FileHash <file>\` in PowerShell, \`sha256sum -c SHA256SUMS.txt --ignore-missing\` on Linux."
  echo
  echo "| File | For | Size | SHA-256 |"
  echo "|---|---|---|---|"
  python3 - <<'EOF'
import json
for f in json.load(open("BUILDINFO.json"))["files"]:
    print(f"| `{f['name']}` | {f['os']} {f['arch']}: {f['kind']} | {f['size'] / 1048576:.1f} MB | `{f['sha256']}` |")
EOF
  echo
  echo "Before it was published, the release workflow checked these exact files (against \`SHA256SUMS.txt\`) on fresh GitHub-hosted runners, Windows Server 2025 and Ubuntu 24.04, with the Rust toolchain removed and nothing built there: the version; the MCP server's tools, drawing, preview, a change to one object, saving and exporting in a folder whose name has Vietnamese letters and spaces, and a write outside the allowed folder refused; the desktop app opening and the MCP bridge editing the document in its window; the MSI's per-user install and uninstall with VectorCraft installed beside it. Nobody has tried it on a Windows 10 or 11 desktop or on other Linux distributions yet."  # brand-ok: the app this test installs beside VansonCraft
  echo
  echo "Set it up for Claude or Codex: [docs/mcp.md]($repo_url/blob/$sha/docs/mcp.md)."
} > "$notes"
cat "$notes"
