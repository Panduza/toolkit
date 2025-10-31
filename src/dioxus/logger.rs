use tracing::subscriber::{set_global_default, SetGlobalDefaultError};
use tracing::Level;
use tracing_subscriber::EnvFilter;

#[derive(Default, Clone, Debug)]
/// Builder for the Dioxus logger
pub struct LoggerBuilder {
    /// Minimum log level
    pub level: Option<Level>,
    /// Additional filters
    pub filters: Vec<String>,
    /// Whether to display the target of the log
    pub display_target: bool,
}

impl LoggerBuilder {
    /// Create a new LoggerBuilder
    pub fn with_level(mut self, level: Level) -> Self {
        self.level = Some(level);
        self
    }

    pub fn display_target(mut self, display: bool) -> Self {
        self.display_target = display;
        self
    }

    /// Add a filter to the logger
    pub fn add_filter(mut self, filter: &str) -> Self {
        self.filters.push(filter.into());
        self
    }

    pub fn filter_rumqttd(mut self) -> Self {
        self.filters.push("rumqttd=off".into());
        self
    }

    pub fn filter_dioxus_core(mut self) -> Self {
        self.filters.push("dioxus_core=off".into());
        self
    }

    pub fn filter_dioxus_signals(mut self) -> Self {
        self.filters.push("dioxus_signals=off".into());
        self
    }

    pub fn filter_warnings(mut self) -> Self {
        self.filters.push("warnings=off".into());
        self
    }

    pub fn filter_rmcp(mut self) -> Self {
        self.filters.push("rmcp=off".into());
        self
    }

    pub fn build(self) -> Result<(), SetGlobalDefaultError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Create a filter that keeps the default level but excludes rumqttd logs
            let level_str = match self.level.unwrap_or(Level::INFO) {
                Level::ERROR => "error",
                Level::WARN => "warn",
                Level::INFO => "info",
                Level::DEBUG => "debug",
                Level::TRACE => "trace",
            };

            let filter_str = format!("{},{}", level_str, self.filters.join(","));
            let filter = EnvFilter::builder().parse_lossy(&filter_str);

            let sub = tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter);

            // if !dioxus_cli_config::is_cli_enabled() {
            //     return set_global_default(sub.finish());
            // }

            // todo(jon): this is a small hack to clean up logging when running under the CLI
            // eventually we want to emit everything as json and let the CLI manage the parsing + display
            set_global_default(sub.without_time().with_target(self.display_target).finish())
        }
    }
}
