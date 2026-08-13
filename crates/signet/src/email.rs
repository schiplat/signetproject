use tracing::info;

/// Minimal outbound mailer abstraction.
///
/// Production delivery should be wired to a real SMTP provider (e.g. via
/// `SIGNET_SMTP_HOST`). Until then, outbound messages are emitted to the log
/// stream so flows (password reset, new-device alerts) are fully testable.
pub async fn send(to: &str, subject: &str, body: &str) {
    info!(to = %to, subject = %subject, "outbound email (log mailer)");
    info!("  body:\n{body}");
}
