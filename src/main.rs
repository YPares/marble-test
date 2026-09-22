use std::{thread, time::Duration};

use marble_test::mpmc;

fn main() {
    let q: mpmc::Queue<u32> = mpmc::Queue::default();
    for client_id in 0..=7 {
        let q_client = q.clone();
        thread::spawn(move || {
            loop {
                // Each client sleeps a different amount of time
                thread::sleep(Duration::from_millis(1 + (client_id as u64)));
                let msg = q_client.read(client_id);
                println!("Client {} read: {:?}", client_id, msg);
            }
        });
    }
    let mut msg = 0;
    loop {
        let r = q.push(msg);
        if r {
            println!("Published {}", msg);
            msg += 1;
        } else {
            println!("Couldn't publish {}, queue full", msg);
        }
        thread::sleep(Duration::from_millis(2));
    }
}
