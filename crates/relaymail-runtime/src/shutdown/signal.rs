use tokio::signal::unix::{signal, SignalKind};

use super::token::ShutdownToken;

/// Spawn a task that watches for SIGTERM and SIGINT; cancels the token on
/// either.
///
/// Returns a [`ShutdownToken`] handle. Callers `.cancelled().await` on the
/// token to block until shutdown is requested.
pub fn install_shutdown_handler() -> std::io::Result<ShutdownToken> {
    let token = ShutdownToken::new();
    let spawn_token = token.clone();
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;
    tokio::spawn(async move {
        tokio::select! {
            _ = sigterm.recv() => {}
            _ = sigint.recv() => {}
        }
        spawn_token.cancel();
    });
    Ok(token)
}
