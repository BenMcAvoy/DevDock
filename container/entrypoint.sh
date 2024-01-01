#!/bin/sh
set -eu

# Allow users to have scripts run on container startup to prepare workspace.
# https://github.com/coder/code-server/issues/5177
if [ -d "${ENTRYPOINTD}" ]; then
  find "${ENTRYPOINTD}" -type f -executable -print -exec {} \;
fi

code-server --install-extension ms-python.python

exec dumb-init /usr/bin/code-server "$@"
