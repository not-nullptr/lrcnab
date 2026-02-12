use std::{path::PathBuf, time::Duration};

use notify_debouncer_full::{
    new_debouncer,
    notify::{EventKind, RecursiveMode},
};
use tokio::sync::mpsc;

pub struct Watcher {
    rx: mpsc::Receiver<PathBuf>,
}

impl Watcher {
    pub fn new(path: PathBuf) -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self::task(tx, path);
        Self { rx }
    }

    pub async fn recv(&mut self) -> Option<PathBuf> {
        self.rx.recv().await
    }

    fn task(tx: mpsc::Sender<PathBuf>, path: PathBuf) {
        std::thread::spawn(move || {
            let (debouncer_tx, debouncer_rx) = std::sync::mpsc::channel();
            let mut debouncer = match new_debouncer(Duration::from_millis(50), None, debouncer_tx) {
                Ok(debouncer) => debouncer,
                Err(e) => {
                    tracing::error!(error = %e, "error creating debouncer");
                    return;
                }
            };

            if let Err(e) = debouncer.watch(&path, RecursiveMode::Recursive) {
                tracing::error!(error = %e, "error watching path");
                return;
            }

            while let Ok(events) = debouncer_rx.recv() {
                match events {
                    Ok(mut events) => {
                        for e in &mut events {
                            // this is so dumb
                            if !matches!(e.kind, EventKind::Create(_) | EventKind::Modify(_)) {
                                continue;
                            }

                            let paths = std::mem::take(&mut e.paths);
                            for path in paths {
                                if let Err(e) = tx.blocking_send(path) {
                                    tracing::error!(error = %e, "error sending path through channel");
                                    break;
                                }
                            }
                        }
                    }

                    Err(errors) => {
                        for error in errors {
                            tracing::error!(error = %error, "error in debouncer event");
                        }
                    }
                }
            }
        });
    }
}
