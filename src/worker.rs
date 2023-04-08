use std::{
    fmt,
    sync::Mutex,
}; // 0.3.5

use tokio::time::{sleep, Duration};
const BIGGEST_POSSIBLE_DURATION: u64 = 150000;
pub struct Worker {
    pump_lock: Mutex<bool>,
}

#[derive(Debug, PartialEq)]
pub enum PumpError {
    AlreadyOn,
    ImpossibleDuration,
}
impl fmt::Display for PumpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PumpError::AlreadyOn => write!(f, "the pump is already on, canceling the operation"),
            PumpError::ImpossibleDuration => write!(f, "the given duration is to big"),
        }
    }
}
impl Worker {
    pub fn new() -> Worker {
        // self.pump_lock.lock() = false;
        Worker {
            pump_lock: Mutex::new(false),
        }
    }

    pub async fn pump_water(&self, ms_duration: u64) -> Result<u64, PumpError> {
        let _lock = match self.pump_lock.try_lock() {
            Err(_) => return Err(PumpError::AlreadyOn),
            Ok(lock) => lock,
        };
        if ms_duration > BIGGEST_POSSIBLE_DURATION {
            return Err(PumpError::ImpossibleDuration);
        }
        log::debug!("before pump");
        sleep(Duration::from_millis(ms_duration)).await;
        log::debug!("after pump");
        Ok(ms_duration)
    }
}

#[cfg(test)]
mod tests {
use futures::future;
    use super::*;
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
        if res3!=Ok(10){
            panic!("the mutex wasnt unlocked properly");
        }
        if a == Err(PumpError::AlreadyOn) || b == Err(PumpError::AlreadyOn) {
            return;
        } else {
            panic!("no good");
        }
    }
}
