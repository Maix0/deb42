use owo_colors::OwoColorize;


pub fn set_logger() -> eyre::Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}",
                format_args!("[{}] {}: {}", record.level(), record.target(), message).color(
                    match record.level() {
                        log::Level::Error => owo_colors::AnsiColors::Red,
                        log::Level::Warn => owo_colors::AnsiColors::Yellow,
                        log::Level::Info => owo_colors::AnsiColors::White,
                        log::Level::Debug => owo_colors::AnsiColors::Magenta,
                        log::Level::Trace => owo_colors::AnsiColors::Cyan,
                    }
                )
            ))
        })
        .level(match crate::ARGS.verbose {
            0 => log::LevelFilter::Info,
            1 => log::LevelFilter::Debug,
            2.. => log::LevelFilter::Trace,
        })
        .level_for("hyper", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}