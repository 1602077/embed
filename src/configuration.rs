use config::ConfigError;

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
}

#[derive(Debug, serde::Deserialize)]
pub struct ServerSettings {
    pub address: String,
    pub port: u16,
    pub log: LogSettings,
    pub embedder: EmbedSettings,
}

#[derive(Debug, serde::Deserialize)]
pub struct LogSettings {
    pub name: String,
    // TODO: convert to more strict typing
    pub level: String,
    pub json: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct EmbedSettings {
    pub r#impl: EmbedderType,
    pub model: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbedderType {
    Transformer,
}

impl EmbedderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmbedderType::Transformer => "transformer",
        }
    }
}

impl TryFrom<String> for EmbedderType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "transformer" => Ok(Self::Transformer),
            other => Err(format!("{} is not a supported embedder type.", other)),
        }
    }
}

/// get_config retrieves application configuration from file according to the
/// application runtime environment set with the APPLICATION_RUNTIME_ENVIRONMENT
/// variable.
///
/// Configuration can also be optionally overwritten using an environment
/// variable prefixed with `APPLICATION__`. e.g. `APPLICATION__SETTINGS_PORT`
/// sets `Settings.port`.
pub fn get_config() -> Result<Settings, ConfigError> {
    let base_path = std::env::current_dir().expect("failed to determine cwd");
    let configuration_directory = base_path.join("config");

    let environment: Environment = std::env::var("APPLICATION__RUNTIME_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APPLICATION__RUNTIME_ENVIRONMENT.");

    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .add_source(
            config::Environment::with_prefix("APPLICATION")
                .prefix_separator("__")
                .separator("_"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

/// Application environment runtime.
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "prod",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "prod" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `prod`.",
                other
            )),
        }
    }
}
