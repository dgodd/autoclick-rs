use rdev::{listen, Event, EventType}; // , simulate};
use std::time::{SystemTime, Duration};

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
    let mut switch_on = RingBuffer::new();
    let mut switch_off = RingBuffer::new();
    let one_second = Duration::new(1, 0);
    let callback = move |event: Event| {
        // println!("My callback {:?}", event);
        match event.event_type {
            EventType::KeyPress(_) => {
                if let Some(key) = event.name {
                    if key == "c" {
                        let oldest = switch_on.add();
                        println!("User pressed {:?} -- {:?}", key, oldest);
                        if oldest <= one_second {
                            println!("  SWITCH ON");
                        }
                    }
                    if key == "d" {
                        let oldest = switch_off.add();
                        println!("User pressed {:?} -- {:?}", key, oldest);
                        if oldest <= one_second {
                            println!("  SWITCH OFF");
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
}

// #[cfg(test)]
// mod tests {
//     use super::{RingBuffer};
//
//     #[test]
//     fn sample1() {
//         let mut buff = RingBuffer::new();
//         buff.insert(1);
//         buff.insert(3);
//         buff.insert(5);
//         buff.insert(7);
//         buff.insert(9);
//         assert_eq!(buff.oldest(), 3);
//     }
// }
