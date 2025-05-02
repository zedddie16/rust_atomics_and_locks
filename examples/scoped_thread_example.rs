use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("length is: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    }); // is okay since we are not modifying values

    let mut mutable_numbers = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            mutable_numbers.push(1);
        });
        s.spawn(|| {
            // mutable_numbers.push(2); // Error!
        });
    }); // not okay as it is taking mut reference twice
}
