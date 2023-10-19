use color_eyre;
use tracing_appender::{non_blocking, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry,
};
use time::{macros::format_description, UtcOffset};
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;


// 初始化tracing
pub fn setup_global_subscriber() -> tracing_appender::non_blocking::WorkerGuard{
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    let formatting_layer = fmt::layer()
        .pretty()
        .with_timer(local_time.clone())
        .with_writer(std::io::stderr);

    // 输出到文件中
    let file_appender = rolling::daily("logs", "sensitive_detect.log");

    let (non_blocking_appender, guard) = non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender)
        .with_timer(local_time)
        .with_span_events(FmtSpan::CLOSE);

    // 注册
    Registry::default()
        .with(env_filter)
        .with(ErrorLayer::default())
        .with(file_layer)
        .with(formatting_layer)
        .init();

    // 安裝 color-eyre 的 panic 处理句柄
    color_eyre::install().unwrap();

    guard
}