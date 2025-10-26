#!/usr/bin/env bash
set -euo pipefail
# Small tool to initialize a local SSH CA and sign user keys
# Usage:
#   create_ssh_ca.sh init [ca_dir]
#   create_ssh_ca.sh sign <ca_dir> <user_pubkey> <principals> [validity]
# Examples:
#   create_ssh_ca.sh init ~/.local/share/ssh_ca
#   create_ssh_ca.sh sign ~/.local/share/ssh_ca ~/.ssh/id_ed25519.pub "username" +52w

CA_DIR_DEFAULT="$HOME/.local/share/ssh_ca"

cmd=${1:-}
if [ -z "$cmd" ]; then
  echo "Usage: $0 init|sign ..."; exit 2
fi

if [ "$cmd" = "init" ]; then
  ca_dir=${2:-$CA_DIR_DEFAULT}
  mkdir -p "$ca_dir"
  chmod 700 "$ca_dir"
  echo "Creating CA key in $ca_dir/ssh_ca (private) and ssh_ca.pub (public). Keep private offline and secure." 
  ssh-keygen -f "$ca_dir/ssh_ca" -C "Local SSH CA" -N ""
  echo "CA created. Public key is: $ca_dir/ssh_ca.pub"
  exit 0
fi

if [ "$cmd" = "sign" ]; then
  ca_dir=${2:-$CA_DIR_DEFAULT}
  user_pubkey=${3:-}
  principals=${4:-}
  validity=${5:-+52w}

  if [ -z "$user_pubkey" ] || [ -z "$principals" ]; then
    echo "Usage: $0 sign <ca_dir> <user_pubkey> <principals> [validity]"; exit 2
  fi

  ca_priv="$ca_dir/ssh_ca"
  if [ ! -f "$ca_priv" ]; then
    echo "CA private key $ca_priv not found. Run 'init' first."; exit 1
  fi

  # output certificate next to user key
  cert_out="${user_pubkey%-*.pub}-cert.pub"
  echo "Signing $user_pubkey -> $cert_out (principals: $principals, validity: $validity)"
  ssh-keygen -s "$ca_priv" -I "$principals" -n "$principals" -V "$validity" "$user_pubkey"
  echo "Signed certificate created (same directory as public key)."
  exit 0
fi

echo "Unknown command: $cmd"; exit 2
