use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                println!("guard locked");
                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard); // after guard thread is dropped value is accesible by
                // other threads, and they can sleep at the same time when they're
                // done with changing guarded value
                println!("guard dropped");
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
}
// This shows the importance of keeping the amount of time a mutex is locked as short as possible. Keeping a mutex locked longer than necessary can completely nullify any benefits of parallelism, effectively forcing everything to happen serially instead.
