use std::sync::{Arc, RwLock};

pub struct Queue<A> {
    /// Each cell is a message paired with a bitfield telling which logical consumers are YET to read the message
    ring: Arc<[RwLock<(A, u8)>; 32]>,
    /// The index of the cell to be written next
    writer_head: Arc<RwLock<usize>>,
}

impl<A: Default> Default for Queue<A> {
    fn default() -> Self {
        Self {
            ring: Default::default(),
            writer_head: Default::default(),
        }
    }
}

impl<A> Clone for Queue<A> {
    fn clone(&self) -> Self {
        Self {
            ring: Arc::clone(&self.ring),
            writer_head: Arc::clone(&self.writer_head),
        }
    }
}

impl<A: Clone> Queue<A> {
    pub fn push(&self, msg: A) -> bool {
        let mut head = self.writer_head.write().unwrap();
        let mut cell = self.ring[*head].write().unwrap();
        if cell.1 != 0 {
            // The queue is full. Some readers are yet to read the next cell to write
            false
        } else {
            *cell = (msg, 255 as u8);
            *head = (*head + 1) % 32;
            true
        }
    }

    pub fn read(&self, client_id: u8) -> Option<A> {
        let head = *self.writer_head.read().unwrap();
        // Queue is bounded and small, linear search is okay
        // But could be improved by having a head per consumer group
        for i in 0..=31 {
            let mut cell = self.ring[(head + i) % 32].write().unwrap();
            if cell.1 & (1 << client_id) != 0 {
                cell.1 ^= 1 << client_id;
                return Some(cell.0.clone());
            }
        }
        None
    }
}
