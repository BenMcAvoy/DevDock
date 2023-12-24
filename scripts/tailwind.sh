#!/usr/bin/env bash

OS=$(uname -s)
FILE_NAME="tailwindcss"

case "$OS" in
  Linux*)
    FILE_NAME="${FILE_NAME}-linux-x64"
    ;;
  Darwin*)
    FILE_NAME="${FILE_NAME}-macos-arm64"
    ;;
  CYGWIN*|MINGW32*|MSYS*|MINGW*)
    FILE_NAME="${FILE_NAME}-windows-x64"
    ;;
  *)
    echo "Unsupported operating system: $OS"
    exit 1
    ;;
esac

if [ -e "tailwind" ]; then
  echo "File \"tailwind\" already exists. Skipping download."
else
  curl -sL "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/$FILE_NAME" -o "tailwind"
fi

chmod +x tailwind

./tailwind "$@"
