use async_lock::Mutex;
use std::cmp::min;
use std::time::Duration;

#[derive(Debug)]
pub struct Limiter {
    tik_duration: Duration,
    slots: Mutex<u64>,
    max_slots: u64,
    slots_per_tik: u64,
}

impl Limiter {
    pub fn new(tik_duration: Duration, max_slots: u64, slots_per_tik: u64) -> Limiter {
        Limiter {
            tik_duration,
            max_slots,
            slots_per_tik,
            slots: Mutex::from(0),
        }
    }

    pub async fn run_tik_loop(&self) {
        loop {
            tokio::time::sleep(self.tik_duration).await;
            {
                let mut slots = self.slots.lock().await;
                *slots = min(*slots + self.slots_per_tik, self.max_slots);
            }
        }
    }

    pub async fn wait_slot(&self) {
        while self.check().await.is_err() {
            tokio::time::sleep(self.tik_duration).await
        }
    }

    pub async fn check(&self) -> Result<(), ()> {
        let mut slots = self.slots.lock().await;
        if *slots > 0 {
            *slots = *slots - 1;
            return Ok(());
        }

        Err(())
    }
}
