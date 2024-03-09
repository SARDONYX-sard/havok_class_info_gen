use anyhow::Result;
use tracing::{level_filters::LevelFilter, subscriber::DefaultGuard};
use tracing_appender::non_blocking::WorkerGuard;

/// Multithread init logger.
///
/// File I/O is No ANSI color, output to stdout has ANSI color.
/// - `test_name`: If [`None`] then stdout only prints.
///
/// # Returns
/// Guards
/// - If this variable is dropped, the logger stops.
pub(crate) fn init_tracing(
    test_name: Option<&str>,
    filter: impl Into<LevelFilter>,
) -> Result<(Option<WorkerGuard>, DefaultGuard)> {
    use tracing_subscriber::{fmt, layer::SubscriberExt};

    let (worker_guard, default_guard) = match test_name {
        Some(test_name) => {
            std::fs::create_dir_all("../../logs")?;
            let (file_writer, guard) = tracing_appender::non_blocking(std::fs::File::create(
                format!("../../logs/{test_name}.log"),
            )?);
            let thread_guard = tracing::subscriber::set_default(
                fmt::Subscriber::builder()
                    .compact()
                    .pretty()
                    .with_file(true)
                    .with_line_number(true)
                    .with_max_level(filter)
                    .with_target(false)
                    .finish()
                    .with(
                        fmt::Layer::default()
                            .compact()
                            .with_ansi(false)
                            .with_file(true)
                            .with_line_number(true)
                            .with_target(false)
                            .with_writer(file_writer),
                    ),
            );

            (Some(guard), thread_guard)
        }
        None => (
            None,
            tracing::subscriber::set_default(
                fmt::Subscriber::builder()
                    .compact()
                    .pretty()
                    .with_file(true)
                    .with_line_number(true)
                    .with_max_level(filter)
                    .with_target(false)
                    .finish(),
            ),
        ),
    };
    Ok((worker_guard, default_guard))
}
