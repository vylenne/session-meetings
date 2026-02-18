# Session Meeting

Video conferencing app built with Jitsi Meet, Nuxt 4 and Rust (Actix Web).

## Prerequisites

- Docker & Docker Compose
- Node.js 20+
- Rust 1.77+

## Quick Start

### 1. Environment

```bash
cp .env.example .env
# Edit .env — set real secrets for DB_PASSWORD, APP_JWT_SECRET, JWT_APP_SECRET
```

### 2. Jitsi Stack

```bash
cd jitsi
bash setup.sh          # creates config dirs, .env, Docker network
# Edit jitsi/.env — set JWT_APP_SECRET to the same value as in root .env
docker compose up -d
cd ..
```

Jitsi will be available at `https://meet.localhost:8443`.

### 3. App Stack

```bash
docker compose up -d   # postgres + backend + frontend
```

- Frontend: `http://localhost:3000`
- Backend API: `http://localhost:8080`
- PostgreSQL: `localhost:5432`

### Stopping

```bash
docker compose down              # app stack
cd jitsi && docker compose down  # jitsi stack
```
