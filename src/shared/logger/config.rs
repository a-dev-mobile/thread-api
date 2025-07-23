
use std::fmt;

use std::io::{Error, ErrorKind};
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

/// Supported log format types
#[derive(Debug, Clone, PartialEq)]
pub enum LogFormat {
    Plain,
    Json,
}

impl fmt::Display for LogFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogFormat::Plain => write!(f, "plain"),
            LogFormat::Json => write!(f, "json"),
        }
    }
}

impl From<&str> for LogFormat {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => LogFormat::Json,
            _ => LogFormat::Plain,
        }
    }
}

pub fn init_logger(log_level: &str, log_format: &str, is_prod: bool) -> Result<(), Error> {
    // Parse and validate the log level, falling back to "info" if invalid
    let filter = EnvFilter::try_new(log_level)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid log level"))?;

    // Create builders with appropriate time settings
    if is_prod {
        // Production mode without timestamps
        let builder = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .without_time();

        // Initialize with the specified format
        let format = LogFormat::from(log_format);
        match format {
            LogFormat::Json => builder.json().init(),
            LogFormat::Plain => builder.init(),
        }
    } else {
        // Development mode with timestamps
        let builder = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);

        // Initialize with the specified format
        let format = LogFormat::from(log_format);
        match format {
            LogFormat::Json => builder.json().init(),
            LogFormat::Plain => builder.init(),
        }
    }

    Ok(())
    // Err(())
}
