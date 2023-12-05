use app_core::add;

/// early return with Err() and write log error.
macro_rules! bail {
    ($err:expr) => {{
        tracing::error!("{}", $err);
        return Err($err.to_string());
    }};
}

#[tauri::command]
pub(crate) async fn add_num(num1: u64, num2: u64) -> Result<u64, String> {
    add(num1, num2).or_else(|err| bail!(err))
}

#[tauri::command]
pub(crate) async fn change_log_level(log_level: Option<&str>) -> Result<(), String> {
    tracing::debug!("Selected log level: {:?}", log_level);
    crate::logging::change_log_level(log_level.unwrap_or("error")).or_else(|err| bail!(err))
}
