use std::{collections::VecDeque, sync::Mutex, thread, time::Duration};
fn main() {
    let queue = Mutex::new(VecDeque::new());
    // as I am seeing VecDeque type first time in my life i decided to
    // develop an understanding on how it works
    //
    // firstly, VecDeque is a "two-directional" Vec with ring buffer and
    // wraping underlying logic
    // for example in a common Vector called vec:
    // [1, 2, 3, 4] vec.push(5) will have O(1) time complexity
    // [1, 2, 3, 4, 5] vec.remove(0) as well as vec.insert(0, x)
    // will have O(n) time complexity as it push/move all items in vector
    // to add or remove item placed not on top. Vec is "stack"
    //
    // VecDeque have "ring buffer" logic under the hood which allows to push/pop from both
    // front and back with O(1) time complexity:
    // initially in memory [_, _, _, _, _, _,  _, _]
    // instead of always starting from index 0, VecDeque has a start index and end index
    // that wrap around.
    // let mut d = VecDeque::with_capacity(8);
    // d.push_back(1);
    // d.push_back(2);
    // d.push_back(3);
    //
    //            front↓ back↓
    // [_, _, _, _, _, 1, 2, 3]      (start at index 5)
    //
    // from now on if we push_front(0):
    //         front↓    back↓
    // [_, _, _, _, 0, 1, 2, 3]      (front moved to index 4)
    //
    // if we push_back(4):
    //  ↓back  front↓
    // [4, _, _, _, 0, 1, 2, 3]      (back wraps around)
    //
    // and when next push will overlap on existing item whole VecDeque
    // will be reallocated into bigger size memory chunk
    // (not the d as it have static capacity but the VecDeque
    // which size isn't known on compile time)
    //
    //

    thread::scope(|s| {
        let t = s.spawn(|| {
            loop {
                let item = queue.lock().unwrap().pop_front();
                if let Some(item) = item {
                    dbg!(item);
                } else {
                    thread::park(); // sleep if no items is in queue
                }
            }
        });
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark(); // thread is not sleeping only when it has what it need
            thread::sleep(Duration::from_secs(1));
        }
    });
}
