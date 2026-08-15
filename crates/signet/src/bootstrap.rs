use crate::config::Config;
use anyhow::Result;
use sqlx::PgPool;

/// True when at least one active `admin` user exists. Generic over the SQL
/// executor so it can run against either a `&PgPool` or a `&mut Transaction`.
pub async fn admin_exists<'c, E>(executor: E) -> Result<bool, sqlx::Error>
where
    E: sqlx::PgExecutor<'c>,
{
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE role = 'admin' AND status = 'active'")
            .fetch_one(executor)
            .await?;
    Ok(count > 0)
}

/// Seed the SCIM bearer token from `SIGNET_SCIM_BEARER_TOKEN` on first boot only.
/// Once a token exists (whether seeded or generated from the dashboard), the env
/// var is ignored so that UI-based rotation becomes the source of truth.
pub async fn ensure_scim_token(pool: &PgPool, cfg: &Config) -> Result<()> {
    let Some(env_token) = cfg.scim_bearer_token.as_deref() else {
        return Ok(());
    };
    let hash = crate::crypto_util::sha256_hex(env_token);
    sqlx::query(
        r#"
        INSERT INTO scim_config (id, token_hash) VALUES (TRUE, $1)
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(hash)
    .execute(pool)
    .await?;
    Ok(())
}
