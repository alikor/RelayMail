use std::sync::OnceLock;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use super::layers::filter_from;

static INIT: OnceLock<()> = OnceLock::new();

/// Initialize tracing for the process. Idempotent — later calls are no-ops.
///
/// When `json == true`, emits one JSON line per event on stdout.
pub fn install_tracing(level: &str, json: bool) {
    let _ = INIT.get_or_init(|| install(level, json));
}

fn install(level: &str, json: bool) {
    let filter = filter_from(level);
    if json {
        let layer = tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .json()
            .with_current_span(true)
            .with_span_list(false);
        let _ = tracing_subscriber::registry()
            .with(filter)
            .with(layer)
            .try_init();
    } else {
        let layer = tracing_subscriber::fmt::layer().with_target(false);
        let _ = tracing_subscriber::registry()
            .with(filter)
            .with(layer)
            .try_init();
    }
}
