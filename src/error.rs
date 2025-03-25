#[derive(Debug)]
pub enum Error {
    SWWW {
        exit_code: Option<i32>,
        error_message: String,
    },
    WMNotSupported,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SWWW {
                exit_code,
                error_message,
            } => write!(
                f,
                "swww failed to set your wallpaper (exit code {}):\n{}",
                exit_code
                    .map(|exit_code| exit_code.to_string())
                    .unwrap_or("stopped by a signal".to_string()),
                error_message
            ),
            Error::WMNotSupported => write!(f, "your window manager is not supported."),
        }
    }
}

impl std::error::Error for Error {}
