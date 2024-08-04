
use std::sync::atomic::{AtomicU32, Ordering::SeqCst};

#[derive(Default)]
pub struct Counter {
    value: AtomicU32,
}

impl Counter {
    pub fn increment(&self) {
        let value = self.value.load(SeqCst);
        self.value.store(value + 1, SeqCst);
    }

    pub fn get(&self) -> u32 {
        self.value.load(SeqCst)
    }
}

#[test]
fn threaded_test() {
    let counter = Counter::default();

    let thread_count = 1000;
    let increment_count = 1000;

    std::thread::scope(|scope| {
        for _ in 0..thread_count {
            scope.spawn(|| {
                for _ in 0..increment_count {
                    counter.increment()
                }
            });
        }
    });

    assert_eq!(counter.get(), thread_count * increment_count);
}
