// lib.rs
// Date: Thu Sep 25 2026
// Educational and Practice Rust Programming Language Code

// rustc 1.100.0-nightly (1303417c4 2026-09-21)
// binary: rustc
// commit-hash: 1303417c416e1595173d9689e7394c31e136ae95
// commit-date: 2026-09-21
// host: x86_64-unknown-linux-gnu
// release: 1.100.0-nightly
// LLVM version: 23.1.1

// cargo 1.100.0-nightly (495c385d0 2026-09-16)
// release: 1.100.0-nightly
// commit-hash: 495c385d0875c4ba51eb72ea0448a2d4c018b8d4
// commit-date: 2026-09-16
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.2.7-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage = self.value as f64 / self.max as f64;

        if percentage >= 1.0 {
            self.messenger.send("This is message one ...");
        } else if percentage >= 0.9 {
            self.messenger.send("This is message two ...");
        } else if percentage >= 0.8 {
            self.messenger.send("This is message three ...");
        }
    }
}

#[cfg(test)]
mod my_test {

    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        send_message: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_message: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.send_message.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_send_percentage_message() {
        let mock_object = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_object, 90);

        limit_tracker.set_value(80);

        println!(
            " -> Output message: {}",
            mock_object.send_message.borrow()[0]
        )
    }
}
