# Signet

Unified identity authentication (SSO / OIDC IdP) service.

> Language: English · [简体中文](./README.zh-CN.md)

- **Production entry (future)**: `https://sso.example.com`
- **Default dev Issuer**: `http://localhost:8443` (configurable via `SIGNET_ISSUER`)
- **First integrating app**: Cella (internal subsystem); clients register through the generic channel (Dashboard → Clients or RFC 7591 dynamic registration), with no built-in presets

> Naming note: this project is an SSO, **not** Alibaba Cloud Object Storage (OSS).

## Documentation

| Document | Contents |
|----------|----------|
| [docs/design.md](./docs/design.md) | Overall design, roles, security, roadmap |
| [docs/security.md](./docs/security.md) | **Security design summary** (credential storage, auth, sessions, MFA, OIDC, audit, keys) |
| [docs/client-integration.md](./docs/client-integration.md) | **Client OIDC integration** (authorize/token/PKCE/IP allowlist) |
| [docs/integrations.md](./docs/integrations.md) | **Integrations** (RFC 7591 dynamic registration · Webhooks/Feishu · SCIM v2) |
| [docs/api-v1.md](./docs/api-v1.md) | **Dashboard HTTP API** (unified `/api/v1/...`) |
| [docs/mfa.md](./docs/mfa.md) | TOTP / recovery codes / global & per-user enforcement |
| [docs/dashboard.md](./docs/dashboard.md) | Dashboard pages & permissions |

## Local development

### 1. Configuration

```bash
cp .env.example .env
# Edit SIGNET_DATABASE_URL, etc.
```

On first run with no admin, open the dashboard in a browser. It redirects to
**`/setup`**, where you create the first administrator account (email, optional
display name, and password).

### 2. Backend

```bash
cargo run -p signet
```

Health check: `GET http://localhost:8443/health`

### 3. Dashboard (Vue)

```bash
cd dashboard
pnpm install
pnpm dev          # http://localhost:5173, proxied to :8443
# or build and embed into the binary:
pnpm build
cd .. && cargo build -p signet
```

In production/staging, Rust serves `dashboard/dist` via `rust-embed`.

### 4. Account model

- **No public registration**; staff accounts are provisioned under **Dashboard → Users** (frontend route `/users`, API `/api/v1/admin/users`)  
- Roles: `admin` / `manager` / `member` (see design doc)  
- First-run **`/setup`** page creates the initial `admin`  
- Optional **MFA** (global or per-user enforced); users may self-enroll from the account menu  

### 5. Observability

- Every request/response carries **`x-request-id`** (passed through or auto-generated UUIDv4) for trace linking
- Access logs are emitted uniformly by `crates/signet/src/access_log.rs`: single line, structured fields (`request_id` / `method` / `path` / `query` / `ip` / `status` / `latency_ms`), with `2xx/3xx` at `INFO`, `4xx` at `WARN`, `5xx` at `ERROR`
- Logging goes through `tracing`; see [.cursor/rules/logging.mdc](./.cursor/rules/logging.mdc): static messages + structured fields, errors in an `error` field, single-line JSON in production (`APP_ENV=production`)
- `GET /metrics` exposes Prometheus metrics

## Main endpoints

| Path | Description |
|------|-------------|
| `GET /health` | Health check |
| `GET /metrics` | Prometheus metrics (public) |
| `GET /.well-known/openid-configuration` | OIDC Discovery |
| `GET /oauth/authorize` | Authorization (redirects to `/login` if not signed in, `/consent` if not consented) |
| `POST /oauth/token` | Token exchange (code + PKCE / refresh) |
| `POST /oauth/consent` | Consent submission |
| `GET /oauth/jwks` | JWKS |
| `GET /oauth/userinfo` | UserInfo (includes `groups`) |
| `GET/POST /oauth/end_session` | Single logout |
| `POST /oauth/revoke` | RFC 7009 revocation |
| `POST /oauth/register` | RFC 7591 dynamic client registration |
| `GET /api/v1/setup/status` | First-run setup probe (`needs_setup`) |
| `POST /api/v1/setup` | Create the first admin on first run |
| `POST /api/v1/password-reset/*` | Password reset (request/confirm) |
| `GET/POST/DELETE /api/v1/me/passkeys/*` | Passkey (WebAuthn) register/sign-in/manage |
| `/scim/v2/*` | SCIM v2 user/group sync (Bearer auth) |
| `/api/v1/*` | Dashboard / session / admin API (**unified prefix**, see [api-v1.md](./docs/api-v1.md)) |

Examples: `POST /api/v1/login`, `GET /api/v1/admin/users`, `GET /api/v1/admin/stats`.

## Repository structure

```text
crates/signet/     Axum OIDC IdP + /api/v1
dashboard/         Vue 3 + Tailwind CSS v4
migrations/        Postgres migrations (MFA, audit IP, client IP allowlist, password reset, webhook, SCIM, WebAuthn)
docs/              design · security · client-integration · integrations · api-v1 · mfa · dashboard
```
