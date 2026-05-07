use std::{sync::Arc, time::Duration};

use tokio::sync::{Mutex, mpsc};

pub struct MbRateLimiter {
    permit_rx: Mutex<mpsc::Receiver<()>>,
}

impl MbRateLimiter {
    pub fn new() -> Arc<Self> {
        let (tx, rx) = mpsc::channel::<()>(1);

        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_millis(1200));

            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

            loop {
                ticker.tick().await;

                if tx.send(()).await.is_err() {
                    break;
                }
            }
        });

        Arc::new(Self {
            permit_rx: Mutex::new(rx),
        })
    }

    pub async fn acquire(&self) {
        let mut rx = self.permit_rx.lock().await;
        let _ = rx.recv().await;
    }
}
