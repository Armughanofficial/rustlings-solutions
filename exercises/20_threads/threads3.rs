use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    let qc = Arc::new(q);

    // Clone the sender for the first thread
    let tx1 = tx.clone();
    let q1 = Arc::clone(&qc);
    thread::spawn(move || {
        for val in &q1.first_half {
            println!("Sending {val:?}");
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    // Use the ORIGINAL tx for the second thread.
    // This is the key! When this thread finishes, the original tx is dropped.
    let q2 = Arc::clone(&qc);
    thread::spawn(move || {
        for val in &q2.second_half {
            println!("Sending {val:?}");
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        // This loop only ends when ALL senders are dropped
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}