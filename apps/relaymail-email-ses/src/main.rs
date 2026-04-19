mod app;
mod config;
mod dry_run;
mod wire;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::process::ExitCode {
    match app::run().await {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("relaymail-email-ses: {err:#}");
            std::process::ExitCode::FAILURE
        }
    }
}
