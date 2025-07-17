use std::env;
use serde::Serialize;


static ENVAR_CONFIG_PATH: &str = "MD_CONTEXT_CONFIG_PATH";
static DEFAULT_CONFIG_PATH: &str = "~/.config/md-context/config.toml";

#[derive(Debug, Serialize)]
struct Configuration {
    storage_mode: Option<StorageMode>,
    languages: Vec<LanguageConfiguration>
}

impl Configuration {
    fn load() -> Self {
        let config_path = env::var(ENVAR_CONFIG_PATH)
            .unwrap_or_else(|_| DEFAULT_CONFIG_PATH.to_string());
    }
}

#[derive(Debug, Default, Serialize)]
enum StorageMode {
    #[default]
    Project,
    ConfigFolder
}

#[derive(Debug, Serialize)]
struct LanguageConfiguration {
    extensions: Vec<String>,
    tag: String
}

