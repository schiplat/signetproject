# Signet

统一身份认证（SSO / OIDC IdP）服务。

- **生产入口（未来）**：`https://signet.ddl.sconts.com`
- **开发默认 Issuer**：`http://localhost:8443`（`SIGNET_ISSUER` 可配）
- **首个接入方**：Cella（内网子系统）；客户端统一走通用申请渠道（Dashboard → Clients 或 RFC 7591 动态注册），无内置预置

> 命名说明：本项目为 SSO，**不是**阿里云 Object Storage（OSS）。

## 文档

| 文档 | 内容 |
|------|------|
| [docs/design.md](./docs/design.md) | 总体设计、角色、安全、路线图 |
| [docs/security.md](./docs/security.md) | **安全设计汇总**（凭证存储、认证、会话、MFA、OIDC、审计、密钥） |
| [docs/client-integration.md](./docs/client-integration.md) | **业务客户端 OIDC 对接**（authorize/token/PKCE/IP 白名单） |
| [docs/integrations.md](./docs/integrations.md) | **集成对接**（RFC 7591 动态注册 · Webhooks/飞书 · SCIM v2） |
| [docs/api-v1.md](./docs/api-v1.md) | **Dashboard HTTP API**（统一 `/api/v1/...`） |
| [docs/mfa.md](./docs/mfa.md) | TOTP / 恢复码 / 全局与用户强制策略 |
| [docs/dashboard.md](./docs/dashboard.md) | 管理台页面与权限说明 |

## 本地开发

### 1. 配置

```bash
cp .env.example .env
# 编辑 SIGNET_DATABASE_URL / TEST_DATABASE_URL、bootstrap 管理员密码等
```

首次启动若库中无管理员，需配置：

- `SIGNET_BOOTSTRAP_ADMIN_EMAIL`
- `SIGNET_BOOTSTRAP_ADMIN_PASSWORD`（≥ 8 字符）

无管理员且未配置上述变量时，进程会 fail-fast 退出。

### 2. 后端

```bash
cargo run -p signet
```

探活：`GET http://localhost:8443/health`

### 3. Dashboard（Vue）

```bash
cd dashboard
pnpm install
pnpm dev          # http://localhost:5173 ，代理到 :8443
# 或构建并嵌入二进制：
pnpm build
cd .. && cargo build -p signet
```

生产/联调时由 Rust 通过 `rust-embed` 托管 `dashboard/dist`。

### 4. 账号模型

- **无公开注册**；staff 在 **Dashboard → Users**（前端路由 `/users`，API `/api/v1/admin/users`）开户  
- 角色：`admin` / `manager` / `member`（见设计文档）  
- 冷启动 bootstrap 创建首位 `admin`  
- 可选 **MFA**（全局或按用户强制）；账户菜单可自愿绑定  

默认 bootstrap（见 `.env.example`）：`admin@example.com` / `changeme-admin`

### 5. 可观测性

- 所有请求响应带 **`x-request-id`**（透传 / 自动生成 UUIDv4），用于链路关联
- 访问日志由 `crates/signet/src/access_log.rs` 统一输出：单行、结构化字段（`request_id` / `method` / `path` / `query` / `ip` / `status` / `latency_ms`），`2xx/3xx` 为 `INFO`，`4xx` 为 `WARN`，`5xx` 为 `ERROR`
- 日志走 `tracing`，规范见 [.cursor/rules/logging.mdc](./.cursor/rules/logging.mdc)：静态消息 + 结构化字段，错误用 `error` 字段，生产（`APP_ENV=production`）输出单行 JSON
- `GET /metrics` 暴露 Prometheus 指标

## 主要端点

| 路径 | 说明 |
|------|------|
| `GET /health` | 探活 |
| `GET /metrics` | Prometheus 指标（公开） |
| `GET /.well-known/openid-configuration` | OIDC Discovery |
| `GET /oauth/authorize` | 授权（未登录跳转 `/login`，未同意跳转 `/consent`） |
| `POST /oauth/token` | 换票（code + PKCE / refresh） |
| `POST /oauth/consent` | 同意页提交 |
| `GET /oauth/jwks` | JWKS |
| `GET /oauth/userinfo` | UserInfo（含 `groups`） |
| `GET/POST /oauth/end_session` | 统一登出 |
| `POST /oauth/revoke` | RFC 7009 吊销 |
| `POST /oauth/register` | RFC 7591 动态客户端注册 |
| `POST /api/v1/password-reset/*` | 密码重置（请求/确认） |
| `GET/POST/DELETE /api/v1/me/passkeys/*` | Passkey（WebAuthn）注册/登录/管理 |
| `/scim/v2/*` | SCIM v2 用户/组同步（Bearer 认证） |
| `/api/v1/*` | Dashboard / 会话 / 管理 API（**统一前缀**，见 [api-v1.md](./docs/api-v1.md)） |

示例：`POST /api/v1/login`、`GET /api/v1/admin/users`、`GET /api/v1/admin/stats`。

## 仓库结构

```text
crates/signet/     Axum OIDC IdP + /api/v1
dashboard/         Vue 3 + Tailwind CSS v4
migrations/        Postgres 迁移（MFA、审计 IP、客户端 IP 白名单、密码重置、Webhook、SCIM、WebAuthn）
docs/              design · security · client-integration · integrations · api-v1 · mfa · dashboard
```
