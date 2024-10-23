use anyhow::Result;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::Appender;
use log4rs::config::Config;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;

#[test]
fn test() -> Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("temp/log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    log::info!("Hello, world!");

    Ok(())
}
