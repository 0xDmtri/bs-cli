#! /bin/bash

# First check OS.
OS="$(uname)"
if [[ "${OS}" == "Linux" ]]
then
  HOMEBREW_ON_LINUX=1
elif [[ "${OS}" != "Darwin" ]]
then
  abort "BS-CLI is only supported on macOS and Linux."
fi

cd bs-cli && cargo build --release && chmod -x target/release/bs-cli

if [ -d ~/.local/bin/bs-cli ]; then
  rm -rf ~/.local/bin/bs-cli
fi

if [ ! -d ~/.local ]; then 
  mkdir ~/.local
  if [ ! -d ~/.local/bin/ ]; then
    mkdir ~/.local/bin
  fi
fi

mkdir ~/.local/bin/bs-cli
mv bs-cli/target/release/bs-cli ~/.local/bin/bs-cli

echo 'Binaries placed in ~/.local/bin/bs-cli'
echo '---------------------------------------------------'
echo 'To make available globally, add to your profile:'

echo 'export PATH="$PATH:/$HOME/.local/bin/bs-cli"'