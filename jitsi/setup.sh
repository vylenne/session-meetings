#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Create config directories
mkdir -p ~/.jitsi-meet-cfg/{web,transcripts,prosody/config,prosody/prosody-plugins-custom,jicofo,jvb,jigasi,jibri}

# Create .env from template if it doesn't exist
if [ ! -f .env ]; then
    cp env.example .env
    bash gen-passwords.sh
    echo "Created jitsi/.env with generated passwords."
    echo "Edit jitsi/.env — set JWT_APP_SECRET to match your backend secret."
else
    echo "jitsi/.env already exists, skipping."
fi

# Create shared Docker network
if ! docker network inspect session-meeting >/dev/null 2>&1; then
    docker network create session-meeting
    echo "Created Docker network: session-meeting"
else
    echo "Docker network session-meeting already exists."
fi

echo "Done. Run 'docker compose up -d' from jitsi/ to start Jitsi stack."
