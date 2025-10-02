use std::collections::VecDeque;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                let item = loop {
                    if let Some(item) = q.pop_front() {
                        break item;
                    } else {
                        q = not_empty.wait(q).unwrap();
                    }
                };
                drop(q);
                dbg!(item);
            }
        });

        for i in 0.. {
            // Mutex is locked, pushed and immedeately
            // released
            queue.lock().unwrap().push_back(i);
            // if there are more than one thread waiting
            // on the same condition variable, and notify_one
            // is called, then only one(random, choosed by OS)
            // thread will awake
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
