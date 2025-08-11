use std::time::{Duration, Instant};

fn main() {
    trpl::run(async {
        // Here it happens the same as with threads,
        // It may not finish the execution of async task.
        // To do it you should handle similar to join() with threads using await
        let handle_spawned_tasks = trpl::spawn_task(async {
            for i in 0..10 {
                println!("hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        for i in 0..15 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(100)).await;
        }

        // This is for waiting to finish the spawned_tasks before finishing main thread.
        handle_spawned_tasks.await.unwrap();
        //
        // To joing both async tasks you should do:
        let fut1 = async {
            for i in 0..5 {
                println!("hi number {i} from the THIRD task!");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        let fut2 = async {
            for i in 0..5 {
                println!("hi number {i} from the FOURTH task!");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        trpl::join(fut1, fut2).await;
        // Sharing data with mpsc
        let (tx, mut rx) = trpl::channel();
        let val = String::from("Hello");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Got: {received}");

        // let vals = vec![
        //     String::from("hi"),
        //     String::from("from"),
        //     String::from("the"),
        //     String::from("future"),
        // ];

        // Take into consideration that this is still not async. As the for loop is not async.
        // To make it async we should wrap it in async
        // In this case it will wait 100 millis, and then send all the messages.
        // for val in vals {
        //     tx.send(val).unwrap();
        //     trpl::sleep(Duration::from_millis(100)).await;
        // }
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received {value}")
            }
        };
        // Now it will print each message after 100 millis
        trpl::join(tx_fut, rx_fut).await;

        // We can also have multiple transmitors. (Scoped for not commenting code)
        {
            let (tx, mut rx) = trpl::channel();

            let tx1 = tx.clone();
            let tx1_fut = async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                for val in vals {
                    tx1.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let rx_fut = async {
                while let Some(value) = rx.recv().await {
                    println!("received '{value}'");
                }
            };

            let tx_fut = async move {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(1500)).await;
                }
            };

            trpl::join3(tx1_fut, tx_fut, rx_fut).await;
        }

        // When programming with async, with std library it uses only one thread. So if there is
        // big computation happening. It can block another async function. For this reason
        // you can use .await inside. When you use await it means that this line of code can await,
        // in this scenario the runtime will pass the function to another async function if any
        // awaiting.
        {
            let one_ns = Duration::from_nanos(1);
            let start = Instant::now();
            async {
                for _ in 1..1000 {
                    trpl::sleep(one_ns).await;
                }
            }
            .await;
            let time = Instant::now() - start;
            println!(
                "'sleep' version finished after {} seconds.",
                time.as_secs_f32()
            );

            let start = Instant::now();
            async {
                for _ in 1..1000 {
                    trpl::yield_now().await;
                }
            }
            .await;
            let time = Instant::now() - start;
            println!(
                "'yield' version finished after {} seconds.",
                time.as_secs_f32()
            );
        }
    });
}
