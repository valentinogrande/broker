SECRETS_DIR="/srv/broker-secrets"
KEY_FILE="$SECRETS_DIR/master.key"
VERSION_FILE="master.key.v"

mkdir -p "$SECRETS_DIR"

if [ ! -f "$KEY_FILE" ]; then
  # generar key
  openssl rand -hex 32 > "$KEY_FILE"
  chmod 600 "$KEY_FILE"
  chown root:root "$KEY_FILE"

  # versionar SOLO porque la key cambió
  if [ ! -f "$VERSION_FILE" ]; then
    echo "1" > "$VERSION_FILE"
  else
    v=$(cat "$VERSION_FILE")
    echo $((v + 1)) > "$VERSION_FILE"
  fi
fi
