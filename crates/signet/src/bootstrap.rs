use crate::config::Config;
use crate::password::hash_password;
use anyhow::{bail, Result};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn ensure_admin(pool: &PgPool, cfg: &Config) -> Result<()> {
    let admin_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE role = 'admin' AND status = 'active'")
            .fetch_one(pool)
            .await?;

    if admin_count > 0 {
        tracing::info!("admin user already present; skipping bootstrap");
        return Ok(());
    }

    let (Some(email), Some(password)) = (
        cfg.bootstrap_admin_email.as_deref(),
        cfg.bootstrap_admin_password.as_deref(),
    ) else {
        bail!(
            "no admin user found; set SIGNET_BOOTSTRAP_ADMIN_EMAIL and \
             SIGNET_BOOTSTRAP_ADMIN_PASSWORD for first-time bootstrap"
        );
    };

    if email.trim().is_empty() || password.len() < 8 {
        bail!("bootstrap admin email must be non-empty and password at least 8 characters");
    }

    let id = Uuid::new_v4();
    let sub = id.to_string();
    let password_hash = hash_password(password)?;
    let display_name = email.split('@').next().unwrap_or("admin").to_string();

    sqlx::query(
        r#"
        INSERT INTO users (id, sub, email, display_name, password_hash, status, role)
        VALUES ($1, $2, $3, $4, $5, 'active', 'admin')
        "#,
    )
    .bind(id)
    .bind(&sub)
    .bind(email.trim().to_lowercase())
    .bind(display_name)
    .bind(password_hash)
    .execute(pool)
    .await?;

    tracing::info!(email = %email.trim().to_lowercase(), "bootstrap admin created");
    Ok(())
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
