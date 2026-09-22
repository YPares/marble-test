use std::{thread, time::Duration};

use marble_test::mpmc;

fn main() {
    let q: mpmc::Queue<u32> = mpmc::Queue::default();
    for client_id in 0..7 {
        let q_client = q.clone();
        thread::spawn(move || {
            // Each client sleeps a different amount of time
            thread::sleep(Duration::from_secs(1 + (client_id as u64)));
            let msg = q_client.read(client_id);
            println!("Client {} read: {:?}", client_id, msg);
        });
    }
    for msg in 0..100000 {
        let r = q.push(msg);
        if r {
            println!("Published {}", msg)
        } else {
            println!("Couldn't publish, queue full");
        }
        thread::sleep(Duration::from_secs(2));
    }
}

