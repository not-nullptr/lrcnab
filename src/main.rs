use crate::client::LrcLib;

mod client;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lrcnab=debug".into()),
        )
        .init();

    let client = LrcLib::new();

    Ok(())
}
