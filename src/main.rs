use rdev::{listen, Event, EventType, simulate, Button};
use std::time::{SystemTime, Duration};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use sysbar;

pub struct RingBuffer {
    idx: usize,
    buff: [SystemTime; 4],
}

impl RingBuffer {
    fn new() -> RingBuffer {
        RingBuffer { idx: 0, buff: [SystemTime::UNIX_EPOCH; 4] }
    }
    
    fn add(&mut self) -> Duration {
        self.buff[self.idx] = SystemTime::now();
        self.idx = (self.idx + 1) % 4;
        self.buff[self.idx].elapsed().unwrap()
    }
}

fn main() {
    let send_clicks = Arc::new(AtomicBool::new(false));
    let send_clicks_start = send_clicks.clone();
    thread::spawn(move || {
        let sleep_time = Duration::from_millis(100);
        let mut i: u64 = 0;
        loop {
            thread::sleep(sleep_time);
            i += 1;
            if send_clicks.load(Ordering::Relaxed) {
                println!("Click {}", i);
                match simulate(&EventType::ButtonPress(Button::Left)) {
                    Ok(()) => (),
                    Err(_) => {
                        println!("We could not send Left Button press");
                    }
                }
            }
        }
    });

    thread::spawn(move || {
        let mut switch_on = RingBuffer::new();
        let mut switch_off = RingBuffer::new();
        let one_second = Duration::new(1, 0);
        let callback = move |event: Event| {
            match event.event_type {
                EventType::KeyPress(_) => {
                    if let Some(key) = event.name {
                        if key == "c" {
                            let oldest = switch_on.add();
                            println!("User pressed {:?} -- {:?}", key, oldest);
                            if oldest <= one_second {
                                println!("  SWITCH ON");
                                send_clicks_start.store(true, Ordering::Relaxed);
                            }
                        }
                        if key == "d" {
                            let oldest = switch_off.add();
                            println!("User pressed {:?} -- {:?}", key, oldest);
                            if oldest <= one_second {
                                println!("  SWITCH OFF");
                                send_clicks_start.store(false, Ordering::Relaxed);
                            }
                        }
                    }
                },
                _ => (),
            }
        };

        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    });

    let mut bar = sysbar::Sysbar::new("Autoclick");
    bar.add_quit_item("Quit");
    bar.display();
}
