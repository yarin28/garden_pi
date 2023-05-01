use std::error::Error;
use std::fmt;
use embedded_hal::digital::v2::OutputPin;
use thiserror::Error;

// 0.3.5
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
const BIGGEST_POSSIBLE_DURATION: u64 = 150000;
pub struct Worker<T: OutputPin> {
    pump_lock: Mutex<bool>,
    water_pump_gpio:T,

}

#[derive(Debug, PartialEq,Error)]
pub enum PumpError<GpioError> {
    #[error("the pump is already on, canceling the operation")]
    AlreadyOn,
    #[error("the given duration is to big")]
    ImpossibleDuration,
    #[error("the gpio pin couldent work")]
    GpioError(#[from] GpioError)
}
#[derive(Debug)]
pub enum WorkerError{
    NotRunningOnRaspberryPi,
    CantOpenPumpGpioPin(Box<dyn Error>)
}
impl Error for WorkerError {
}
impl fmt::Display for WorkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self{
    WorkerError::NotRunningOnRaspberryPi =>  write!(f,"not running on a raspberry pi at the moment"),
    WorkerError::CantOpenPumpGpioPin(e) => write!(f,"cant open punp gpio pin num {}",e),
    }
  }
}

impl<T: OutputPin> Worker<T> {
    pub fn new( water_pump_gpio:T) -> Self {

        Worker {
            pump_lock: Mutex::new(false),
            water_pump_gpio: water_pump_gpio,
        }
    }

    pub async fn pump_water(&mut self, ms_duration: u64) -> Result<u64, PumpError<T::Error>> {
        let _lock = match self.pump_lock.try_lock() {
            Err(_) => return Err(PumpError::AlreadyOn),
            Ok(lock) => lock,
        };
        if ms_duration > BIGGEST_POSSIBLE_DURATION {
            return Err(PumpError::ImpossibleDuration);
        }
        log::debug!("before pump");
        self.water_pump_gpio.set_high()?;
        sleep(Duration::from_millis(ms_duration)).await;
        self.water_pump_gpio.set_low()?;
        log::debug!("after pump");
        Ok(ms_duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future;
    #[tokio::test]
    async fn the_number_is_impossible_err() {
        let worker = Worker::new();
        let res = worker.pump_water(BIGGEST_POSSIBLE_DURATION + 1).await;
        assert_eq!(res, Err(PumpError::ImpossibleDuration));
    }
    #[tokio::test]
    async fn two_async_calls_cant_access_pump_together() {
        let worker = Worker::new();
        let res = worker.pump_water(10);
        let res2 = worker.pump_water(10);
        let (a, b) = future::join(res, res2).await;
        let res3 = worker.pump_water(10).await;
        if res3 != Ok(10) {
            panic!("the mutex wasn`t unlocked properly");
        }
        if a == Err(PumpError::AlreadyOn) || b == Err(PumpError::AlreadyOn) {
            return;
        } else {
            panic!("no good");
        }
    }
}
