#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: ./generate-migration.sh <migration_name>"
    exit 1
fi

TIMESTAMP=$(date +%Y-%m-%d-%H%M%S)
MIGRATION_NAME="$1"
MIGRATION_DIR="migrations/${TIMESTAMP}_${MIGRATION_NAME}"

mkdir -p "$MIGRATION_DIR"
touch "$MIGRATION_DIR/up.sql"
touch "$MIGRATION_DIR/down.sql"

echo "Created migration files:"
echo "  $MIGRATION_DIR/up.sql"
echo "  $MIGRATION_DIR/down.sql"