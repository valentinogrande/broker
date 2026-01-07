#!/bin/bash
set -euo pipefail

SECRETS_DIR="/srv/broker-secrets"
VERSION_FILE="$SECRETS_DIR/master.key.v"

mkdir -p "$SECRETS_DIR"

if [ ! -f "$VERSION_FILE" ]; then
  VERSION=1
else
  VERSION=$(cat "$VERSION_FILE")
  VERSION=$((VERSION + 1))
fi

KEY_FILE="$SECRETS_DIR/master.key.$VERSION"

openssl rand -hex 32 > "$KEY_FILE"
chmod 600 "$KEY_FILE"
chown root:root "$KEY_FILE"

echo "$VERSION" > "$VERSION_FILE"
