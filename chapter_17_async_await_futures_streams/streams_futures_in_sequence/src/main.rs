use trpl::Stream;
use trpl::{self, ReceiverStream, StreamExt};
use std::time::Duration;
use std::{pin::pin};

fn main() {
    trpl::run(async {
        {
            // In this case, there is no need to do it async, as we only stream the messages.
            // there hello
            let mut messages = get_messages();

            while let Some(message) = messages.next().await {
                println!("{message}")
            }
        }

        {
            // Not let's do it again but implementing futures
            let mut messages = pin!(get_messages().timeout(Duration::from_millis(100)));

            while let Some(result) = messages.next().await {
                match result {
                    Ok(message) => println!("{message}"),
                    Err(reason) => println!("Problem {reason:?}")
                }
            }
        }
    })
}
fn get_messages() -> impl Stream<Item = String> {

    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            tx.send(format!("Message: '{message}'")).unwrap();
        }
    });

    ReceiverStream::new(rx)
}
