use crate::sources::source::Config;
use std::fs;

pub fn load_config() -> anyhow::Result<Config> {
    let mut path = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("No config dir"))?;

    path.push("rss-reader");

    // ensure directory exists
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    path.push("config.toml");

    // if file doesn't exist → create with defaults
    if !path.exists() {
        let default_config = r#"
[[rss]]
url = "https://feeds.arstechnica.com/arstechnica/index"
tag = "Ars Technica"

[[rss]]
url = "http://rss.slashdot.org/Slashdot/slashdotMain"
tag = "Slashdot"
"#;

        fs::write(&path, default_config)?;
    }

    // now read it
    let content = fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&content)?;

    Ok(config)
}
