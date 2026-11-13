#!/usr/bin/env bash
# After publishing: download this OS's release files from the public release page, with no token,
# check them against the published SHA256SUMS.txt and run them. Used by
# .github/workflows/release.yml (verify-public), on Linux and on Windows (Git Bash).
#
#   packaging/smoke/public.sh REPO_URL TAG VERSION PRERELEASE OUT
#
# Also checks where /releases/latest leads: to TAG for a release, never to a pre-release.
set -euo pipefail
repo="$1" tag="$2" v="$3" prerelease="$4" out="$5"
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$out"
cd "$out"
fetch() { curl --proto '=https' -fsSL --retry 5 --retry-all-errors -o "$1" "$repo/releases/download/$tag/$1"; }

case "$(uname -s)" in
  Linux) files=("vansoncraft-$v-linux-x86_64.tar.gz" "vansoncraft-$v-linux-x86_64.AppImage") ;;
  MINGW* | MSYS* | CYGWIN*) files=("vansoncraft-$v-windows-x64-portable.zip" "vansoncraft-$v-windows-x64.msi") ;;
  *) echo "::error::unsupported OS $(uname -s)"; exit 1 ;;
esac
fetch SHA256SUMS.txt
fetch BUILDINFO.json
for f in "${files[@]}"; do
  t=$(date +%s)
  fetch "$f"
  echo "ok   downloaded $f ($(wc -c <"$f") bytes) in $(($(date +%s) - t)) s"
done
sha256sum -c --ignore-missing SHA256SUMS.txt
for f in "${files[@]}"; do grep -q "  $f\$" SHA256SUMS.txt || { echo "::error::$f is not in SHA256SUMS.txt"; exit 1; }; done

case "$(uname -s)" in
  Linux)
    mkdir -p unpacked
    tar -C unpacked -xzf "${files[0]}"
    cli="$PWD/unpacked/vansoncraft-$v-linux-x86_64/bin/vansoncraft-cli"
    ;;
  *)
    powershell.exe -NoProfile -Command "Expand-Archive -Path '${files[0]}' -DestinationPath unpacked"
    cli="$PWD/unpacked/vansoncraft-$v-windows-x64-portable/vansoncraft-cli.exe"
    ;;
esac
said="$("$cli" --version | tr -d '\r')"
[[ "$said" == "vansoncraft-cli $v" ]] || { echo "::error::vansoncraft-cli --version said '$said'"; exit 1; }
echo "ok   $said"
python_bin="$(command -v python3 || command -v python)"
script="$here/mcp_smoke.py" work="$out/work"
if command -v cygpath >/dev/null; then
  # Git Bash on Windows: Windows' Python and the CLI take Windows paths.
  cli="$(cygpath -w "$cli")" script="$(cygpath -w "$script")" work="$(cygpath -w "$work")"
fi
"$python_bin" -I "$script" --cli "$cli" --work "$work" --version "$v" --report report.json

# Where the release page's "latest" link leads (followed without a token).
latest="$(curl -sSIL -o /dev/null -w '%{http_code} %{url_effective}' "$repo/releases/latest")"
echo "/releases/latest: $latest"
if [[ "$prerelease" == "true" ]]; then
  [[ "$latest" != *"/tag/$tag" ]] || { echo "::error::/releases/latest leads to the pre-release $tag"; exit 1; }
else
  [[ "$latest" == *"/tag/$tag" ]] || { echo "::error::/releases/latest does not lead to $tag"; exit 1; }
fi
