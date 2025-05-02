use std::thread;
fn main() {
    let numbers_from_zero_to_thousand = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers_from_zero_to_thousand.len();
        let sum = numbers_from_zero_to_thousand.iter().sum::<usize>();
        sum / len
    });

    println!("Average is: {}", t.join().unwrap());
}
