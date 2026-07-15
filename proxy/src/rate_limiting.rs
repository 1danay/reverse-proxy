use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use tokio::time::interval;

pub fn start_rate_limiter_reset(counter: Arc<AtomicUsize>, interval_secs: u64) {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(interval_secs));

        loop {
            ticker.tick().await;

            counter.store(0, Ordering::SeqCst);
        }
    });
}
