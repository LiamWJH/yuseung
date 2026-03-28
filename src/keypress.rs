use std::sync::mpsc::{self, Receiver};
use std::thread;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub struct Input {
    rx: Receiver<KeyEvent>,
}

impl Input {
    pub fn new() -> std::io::Result<Self> {
        enable_raw_mode()?;
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            loop {
                match event::read() {
                    Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
                        if tx.send(key_event).is_err() {
                            break;
                        }
                    }
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        });
        Ok(Self { rx })
    }

    pub fn try_read(&self) -> Option<KeyEvent> {
        self.rx.try_recv().ok()
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}