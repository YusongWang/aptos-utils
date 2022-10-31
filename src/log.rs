use std::io;

use crossterm::tty::IsTty;
use tracing_subscriber::{
    fmt::{format::Writer, time::FormatTime},
    EnvFilter,
};

pub fn initialize_logger(verbosity: u8, filename: &str, filepath: &str) {
    match verbosity {
        0 => std::env::set_var("RUST_LOG", "info"),
        1 => std::env::set_var("RUST_LOG", "debug"),
        2 | 3 => std::env::set_var("RUST_LOG", "trace"),
        _ => std::env::set_var("RUST_LOG", "info"),
    };

    struct LocalTimer;
    impl FormatTime for LocalTimer {
        fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
            write!(w, "{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))
        }
    }

    if !filepath.is_empty() && !std::io::stdout().is_tty() && std::fs::metadata(&filepath).is_err()
    {
        std::fs::create_dir(&filepath).unwrap();
    }

    let file_appender = tracing_appender::rolling::hourly(filepath, filename);

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // 设置日志输出时的格式，例如，是否包含日志级别、是否包含日志来源位置、
    // 设置日志的时间格式 参考: https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/struct.SubscriberBuilder.html#method.with_timer
    let format = tracing_subscriber::fmt::format()
        .with_target(false)
        .with_level(verbosity == 3 || verbosity == 1)
        .with_line_number(verbosity == 3 || verbosity == 1)
        .with_source_location(verbosity == 3 || verbosity == 1)
        .with_timer(LocalTimer);

    // Filter out undesirable logs.
    let filter = EnvFilter::from_default_env()
        .add_directive("mio=off".parse().unwrap())
        .add_directive("tokio_util=off".parse().unwrap())
        // .add_directive("hyper::proto::h1::conn=off".parse().unwrap())
        // .add_directive("hyper::proto::h1::decode=off".parse().unwrap())
        // .add_directive("hyper::proto::h1::io=off".parse().unwrap())
        // .add_directive("hyper::proto::h1::role=off".parse().unwrap())
        .add_directive("jsonrpsee=off".parse().unwrap());

    if std::io::stdout().is_tty() {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_writer(io::stdout) // 写入标准输出
            .with_ansi(true) // 如果日志是写入文件，应将ansi的颜色输出功能关掉
            .event_format(format)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_writer(non_blocking) // 写入文件，将覆盖上面的标准输出
            .with_ansi(false) // 如果日志是写入文件，应将ansi的颜色输出功能关掉
            .event_format(format)
            .init();
    }

    // let default_subscriber = tracing_subscriber::fmt()
    //     .with_env_filter(filter)
    //     .with_writer(non_blocking) // 写入文件，将覆盖上面的标准输出
    //     .with_ansi(std::io::stdout().is_tty())
    //     //.with_writer(move || LogWriter::new(&log_sender))
    //     .with_level(true)
    //     .with_target(true)
    //     .with_line_number(true)
    //     .with_file(true)
    //     .with_timer(LocalTimer)
    //     .finish();

    // tracing::subscriber::set_global_default(default_subscriber).unwrap();
    //tracing::subscriber::set_global_default(subscriber.with(file_layer)).
    // unwrap();
}
