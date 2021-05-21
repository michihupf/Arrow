use fern::colors::ColoredLevelConfig;

pub fn init_logger() -> Result<(), fern::InitError> {
    let color = ColoredLevelConfig::new();

    fern::Dispatch::new()
        .level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{}[{}] [{}] {}",
                        chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                        color.color(record.level()),
                        record.target(),
                        message
                    ))
                })
                .chain(std::io::stderr()),
        )
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{}[{}] [{}] {}",
                        chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .chain(fern::log_file("output.log")?),
        )
        .apply()?;

    Ok(())
}
