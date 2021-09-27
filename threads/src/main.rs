use std::time::Duration;
use std::thread;
use std::sync::mpsc;

fn main() {
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }

    {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }

    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec!["hi", "from", "the", "thread"];
            for val in vals {
                tx.send(val.to_string()).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    {
        let (tx1, rx) = mpsc::channel();
        let tx2 = tx1.clone(); /*mpsc::Sender::clone(&tx1);*/

        thread::spawn(move || {
            let vals = vec!["hi", "from", "the", "thread"];

            for val in vals {
                tx1.send(val.to_string()).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec!["more", "messages", "for", "you"];

            for val in vals {
                tx2.send(val.to_string()).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
}
