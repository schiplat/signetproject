use signet::build_app;
use signet::config::Config;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,signet=debug")),
        )
        .init();

    let cfg = Config::from_env()?;
    let bind = cfg.http_bind;
    let issuer = cfg.issuer.clone();
    let app = build_app(cfg).await?;

    tracing::info!(%bind, %issuer, "signet listening");
    let listener = tokio::net::TcpListener::bind(bind).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
