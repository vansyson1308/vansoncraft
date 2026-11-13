#!/usr/bin/env bash
# Check VansonCraft's Linux packages as a user gets them: nothing is built, only the release files
# in ASSETS are used. Used by .github/workflows/release.yml (smoke-linux).
#
#   packaging/smoke/linux.sh ASSETS VERSION OUT [UNPACK]
#
# Checks the files against SHA256SUMS.txt, unpacks the tar.gz into a folder under UNPACK (default:
# OUT-unpacked; not the temporary folder, where `setup` rightly refuses to point a client at a
# program) whose name has spaces and Vietnamese letters, runs the CLI, the MCP
# checks (mcp_smoke.py), the desktop app under Xvfb with the MCP bridge (twice: a cold and a warm
# start) and the AppImage. Writes report.json, the logs and the window's screenshot to OUT, and no
# program files. Needs python3, Xvfb and the libraries a Linux desktop has.
set -euo pipefail
assets="$1" v="$2" out="$3" unpack="${4:-$3-unpacked}"
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$out"
now() { date +%s.%N; }
since() { python3 -c "import sys; print(round(float(sys.argv[2]) - float(sys.argv[1]), 3))" "$1" "$(now)"; }

tarball="vansoncraft-$v-linux-x86_64.tar.gz"
appimage="vansoncraft-$v-linux-x86_64.AppImage"
(cd "$assets" && sha256sum -c --ignore-missing SHA256SUMS.txt)
for f in "$tarball" "$appimage"; do
  grep -q "  $f\$" "$assets/SHA256SUMS.txt" || { echo "::error::$f is not in SHA256SUMS.txt"; exit 1; }
done

root="$unpack/Thử nghiệm VansonCraft"
mkdir -p "$root"
t=$(now)
tar -C "$root" -xzf "$assets/$tarball"
unpack_s=$(since "$t")
dir="$root/vansoncraft-$v-linux-x86_64"
cli="$dir/bin/vansoncraft-cli"
app="$dir/bin/vansoncraft"
for f in "$cli" "$app" "$dir/share/doc/vansoncraft/LICENSE-MIT" "$dir/share/doc/vansoncraft/NOTICE"; do
  [[ -f "$f" ]] || { echo "::error::the tar.gz has no ${f#"$dir"/}"; exit 1; }
done
said="$("$cli" --version)"
[[ "$said" == "vansoncraft-cli $v" ]] || { echo "::error::vansoncraft-cli --version said '$said'"; exit 1; }
echo "ok   $said"

python3 -I "$here/mcp_smoke.py" --cli "$cli" --work "$out/work" --version "$v" --report "$out/report-headless.json"
python3 -I "$here/mcp_smoke.py" --cli "$cli" --work "$out/setup" --setup --report "$out/report-setup.json"

# The desktop app, in a display of its own, with settings of its own.
export DISPLAY=:77
Xvfb "$DISPLAY" -screen 0 1600x1000x24 >"$out/xvfb.log" 2>&1 &
xvfb=$!
trap 'kill "$xvfb" 2>/dev/null || true' EXIT
sleep 1
# Sets app_pid and started_s. Not called in $( ): a subshell would lose app_pid.
start_app() {
  local port="$1" log="$2" t
  t=$(now)
  VANSONCRAFT_CONFIG_DIR="$out/config" "$app" --control "$port" >"$log" 2>&1 &
  app_pid=$!
  for _ in $(seq 1 600); do
    if (echo >"/dev/tcp/127.0.0.1/$port") 2>/dev/null; then started_s=$(since "$t"); return 0; fi
    kill -0 "$app_pid" 2>/dev/null || { echo "::error::the app exited before it listened"; cat "$log"; return 1; }
    sleep 0.1
  done
  echo "::error::the app did not listen on $port within 60 s"; cat "$log"; return 1
}
stop_app() { kill "$app_pid" 2>/dev/null || true; wait "$app_pid" 2>/dev/null || true; }
start_app 7979 "$out/app-cold.log"
cold_s=$started_s
python3 -I "$here/mcp_smoke.py" --cli "$cli" --work "$out/work" --desktop 7979 --version "$v" --report "$out/report-desktop.json" || { stop_app; exit 1; }
stop_app
start_app 7979 "$out/app-warm.log"
warm_s=$started_s
stop_app
echo "ok   desktop app listened after $cold_s s (cold) and $warm_s s (warm)"

chmod +x "$assets/$appimage"
said="$(APPIMAGE_EXTRACT_AND_RUN=1 "$assets/$appimage" --version 2>/dev/null | tail -1)"
[[ "$said" == *"$v"* ]] || { echo "::error::the AppImage's --version said '$said'"; exit 1; }
echo "ok   AppImage: $said"

python3 - "$out" "$unpack_s" "$cold_s" "$warm_s" <<'EOF'
import json, os, sys
out, unpack, cold, warm = sys.argv[1], *map(float, sys.argv[2:])
report = {"unpack_s": unpack, "app_cold_start_s": cold, "app_warm_start_s": warm}
for name in ("report-headless.json", "report-desktop.json"):
    report.update({k: v for k, v in json.load(open(os.path.join(out, name))).items() if k != "cli"})
json.dump(report, open(os.path.join(out, "report.json"), "w"), indent=1)
print(json.dumps(report))
EOF
