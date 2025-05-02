use std::thread;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello from the main thread");

    t1.join().unwrap();
    t2.join().unwrap();

    let numbers = vec![1, 2, 3];
    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();
}

fn f() {
    println!("Hello fom another thread");

    let id = thread::current().id();
    println!("This is my thread id {id:?}");
}
