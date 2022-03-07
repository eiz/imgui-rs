#!/bin/bash
set -euo pipefail

SCRIPT_DIR=$(dirname ${0} | python3 -c 'import os, sys; print(os.path.abspath(sys.stdin.read().strip()))' )

cd ${SCRIPT_DIR}
./_update-imgui.sh ~/code/vendor/imgui 0f14933577a1de01d90f8e87622296c466146f21 ./imgui-master/imgui
./_update-imgui.sh ~/code/vendor/imgui f3373780668fba1f9bd64c208d05c20b781c9a39 ./imgui-docking/imgui
