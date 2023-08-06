#!/bin/bash

# ref: http://redsymbol.net/articles/unofficial-bash-strict-mode/
function exit_trap() { echo -e "\n$0:${BASH_LINENO[0]} '$BASH_COMMAND' failed" >&2; }

trap exit_trap ERR
set -eua pipefail
IFS=$'\n\t'
