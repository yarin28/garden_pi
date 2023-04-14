use log::LevelFilter;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use worker::WorkerError;
use std::error::Error;
use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;
use crate::worker::Worker;
mod worker;

#[tokio::main]




async fn main() -> Result<(), Box<dyn Error>> {
// #[cfg(feature = "raspberry_pi")]    
// {
//     println!("print only for raspberry pi")
// }
  println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());
    let mut pin = Gpio::new()?.get(10)?.into_output();
    match configure_logger() {
        Err(e) => return Err(e),
        _ => {
            log::info!("the logger is initialized")
        }
    };

    let worker = Worker::new();
    match worker.pump_water(100).await {
        Err(e) => log::error!(target:"roling","{}",format!("the punp return this error - {}",e)
        ),
        Ok(i) => println!("{}", i),
    };
    Ok(())
}

fn configure_logger() -> Result<(), Box<dyn Error>> {
    let log_line_pattern = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}";

    let trigger_size = byte_unit::n_kb_bytes!(300) as u64;
    let trigger = Box::new(SizeTrigger::new(trigger_size));

    let roller_pattern = "logs/log_{}.log";
    let roller_count = 5;
    let roller_base = 1;
    let roller = Box::new(
        FixedWindowRoller::builder()
            .base(roller_base)
            .build(roller_pattern, roller_count)
            .unwrap(),
    );

    let compound_policy = Box::new(CompoundPolicy::new(trigger, roller));

    let roling_log = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_line_pattern)))
        .build("logs/log_0.log", compound_policy)
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("roling_log", Box::new(roling_log)))
        .build(
            Root::builder()
                .appender("roling_log")
                .build(LevelFilter::Debug),
        )?;

    log4rs::init_config(config)?;
    Ok(())
}

fn initializeWorker()->Result<Worker,WorkerError>