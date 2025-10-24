use tracing::subscriber::{set_global_default, SetGlobalDefaultError};
use tracing::Level;
use tracing_subscriber::EnvFilter;

/// Dioxus logger but with custom level and filtering
pub fn init_logger(level: Level) -> Result<(), SetGlobalDefaultError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Create a filter that keeps the default level but excludes rumqttd logs
        let level_str = match level {
            Level::ERROR => "error",
            Level::WARN => "warn",
            Level::INFO => "info",
            Level::DEBUG => "debug",
            Level::TRACE => "trace",
        };

        let filter_str = format!("{},rumqttd=off", level_str);
        let filter = EnvFilter::builder().parse_lossy(&filter_str);

        let sub = tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter);

        // if !dioxus_cli_config::is_cli_enabled() {
        //     return set_global_default(sub.finish());
        // }

        // todo(jon): this is a small hack to clean up logging when running under the CLI
        // eventually we want to emit everything as json and let the CLI manage the parsing + display
        set_global_default(sub.without_time().with_target(false).finish())
    }
}
