use std::{
    collections::HashMap,
    net::IpAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

use tokio::time::interval;

pub fn start_rate_limiter_reset(counter: Arc<Mutex<HashMap<IpAddr, usize>>>, interval_secs: u64) {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(interval_secs));

        loop {
            ticker.tick().await;

            if let Ok(mut map) = counter.lock() {
                map.clear();
            }
        }
    });
}
