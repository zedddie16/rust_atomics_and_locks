use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello from the main thread");

    // no "move" will make thread take values by references,
    // thus resulting with compiler error, since thread might
    // outlive variable.
    // Thread might live until the very end of main thread,
    // thus spawn function require 'static lifetime bound on argument type.
    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    })
    .join()
    // join = await for thread to finish, any logic after the join call
    // will be done always after the logic in joined thread.
    .unwrap();
    t1.join().unwrap();
    t2.join().unwrap();
}

fn f() {
    println!("Hello fom another thread");

    let id = thread::current().id();
    println!("This is my thread id {id:?}");
}
