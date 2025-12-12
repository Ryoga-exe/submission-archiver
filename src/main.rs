use std::path::Path;

use anyhow::Ok;

mod config;

fn main() -> anyhow::Result<()> {
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());
    let cfg = config::Config::load_from_path(Path::new(&config_path))?;

    std::fs::create_dir_all(&cfg.state_dir)?;

    Ok(())
}
