use futures::{future::select, stream::select};
use std::pin::Pin;
use tokio::time::{Duration, interval, sleep};
use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    println!("=== Stream Examples ===\n");
    // Example 1: Converting iterator to stream
    basic_stream_example().await;
    // Example 2: Stream from async channel
    channel_stream_example().await;

    // Example 3: Timer/interval stream
    timer_stream_example().await;

    // Example 4: Combining/merging streams
    // merging_streams_example().await;

    // Example 5: Stream processing with timeouts
    // timeout_stream_example().await;
    //
    //
    //
    // =======================
    // Real world examples where it can be useful
    // ======================
    // Simulate multiple users sending messages
    for user_id in 1..=3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            for msg_num in 1..=3 {
                let message = format!("User{}: Message {}", user_id, msg_num);
                tx.send(message).await.unwrap();
                sleep(Duration::from_millis(100 * user_id as u64)).await;
            }
        });
    }
    drop(tx); // Close sender

    // Process chat messages with filtering and broadcasting
    let processed_messages: Vec<String> = tokio_stream::wrappers::ReceiverStream::new(rx)
        .filter(|msg| !msg.contains("spam")) // Filter spam
        .map(|msg| format!("📢 BROADCAST: {}", msg)) // Add broadcast prefix
        .timeout(Duration::from_secs(1)) // Timeout slow messages
        .filter_map(|result| async move {
            // Handle timeouts
            match result {
                Ok(msg) => Some(msg),
                Err(_) => None,
            }
        })
        .collect()
        .await;

    for msg in processed_messages {
        println!("{}", msg);
    }
    println!();
}

// 4. Database Batch Processing
// Problem: Process millions of database records in batches without overwhelming memory
async fn database_batch_example() {
    println!("--- 4. Database Batch Processing ---");

    // Simulate database records
    let user_ids: Vec<u32> = (1..=20).collect();

    let processed_users: Vec<String> = stream::iter(user_ids)
        .chunks(5) // Process in batches of 5
        .then(|batch| async move {
            // Simulate database batch operation
            sleep(Duration::from_millis(50)).await;
            format!("✅ Processed batch: {:?}", batch)
        })
        .collect()
        .await;

    for result in processed_users {
        println!("{}", result);
    }
    println!();
}

// 5. Sensor Data Monitoring
// Problem: Monitor IoT sensors with alerts, data validation, and real-time processing
async fn sensor_monitoring_example() {
    println!("--- 5. Sensor Monitoring ---");

    let (tx, rx) = tokio::sync::mpsc::channel(50);

    // Simulate sensor sending temperature data
    tokio::spawn(async move {
        let temperatures = [20.5, 21.0, 19.8, 25.2, 30.1, 35.7, 40.2, 22.1];
        for temp in temperatures {
            tx.send(temp).await.unwrap();
            sleep(Duration::from_millis(200)).await;
        }
    });

    tokio_stream::wrappers::ReceiverStream::new(rx)
        .map(|temp| {
            if temp > 35.0 {
                format!("🔥 ALERT: High temperature {}°C", temp)
            } else if temp < 15.0 {
                format!("🧊 ALERT: Low temperature {}°C", temp)
            } else {
                format!("✅ Normal temperature {}°C", temp)
            }
        })
        .for_each(|alert| async move {
            println!("{}", alert);
            // In real world: send to monitoring system, save to database, etc.
        })
        .await;

    println!();
}

/*
Real-World Stream Benefits:

1. **Memory Efficiency**: Process huge datasets without loading everything
2. **Backpressure**: Slow consumers don't crash fast producers
3. **Composition**: Chain complex operations easily
4. **Error Handling**: Built-in timeout and error propagation
5. **Concurrency Control**: Limit concurrent operations (buffer_unordered)

Common Real-World Use Cases:

🌐 **Web APIs**: Rate limiting, batch requests, response processing
📁 **File Processing**: Large CSV/JSON files, log analysis, data migration
💬 **Real-time Systems**: Chat, notifications, live updates
🗄️ **Database Operations**: Batch processing, streaming queries, ETL pipelines
📊 **Data Processing**: Analytics, monitoring, sensor data, financial transactions
🔄 **Event Systems**: Message queues, event sourcing, pub/sub systems
📱 **UI Applications**: User input streams, real-time updates, infinite scroll

When NOT to use streams:
❌ Simple one-time operations
❌ Small datasets that fit in memory easily
❌ When you need random access to data
❌ Synchronous, blocking operations
*/

async fn basic_stream_example() {
    print!("Example basic_stream_example from Iterator");
    let numbers = vec![1, 2, 3, 4, 5];
    let mut stream = stream::iter(numbers).map(|x| x * 2);

    while let Some(value) = stream.next().await {
        println!("Doubled {value}");
        sleep(Duration::from_millis(100)).await;
    }
}

async fn channel_stream_example() {
    println!("Example channel_stream_example (Stream from channel)");
    let (tx, rx) = tokio::sync::mpsc::channel(10);

    // Spawn task to send messages over time channel
    tokio::spawn(async move {
        let messages = ["Hello", "from", "async", "channel", "stream"];
        for msg in messages {
            if tx.send(msg.to_string()).await.is_err() {
                break;
            }
            sleep(Duration::from_millis(100)).await;
        }
    });

    let mut stream = tokio_stream::wrappers::ReceiverStream::new(rx);

    while let Some(message) = stream.next().await {
        println!("Received: {message}")
    }
}

async fn timer_stream_example() {
    println!("Example timer_stream_example (Timer stream)");

    let mut interval_stream =
        tokio_stream::wrappers::IntervalStream::new(interval(Duration::from_millis(200)));

    let mut counter = 0;
    while let Some(value) = interval_stream.next().await {
        counter += 1;
        println!("Time tick #{counter}");

        if counter >= 5 {
            break;
        }
    }
}

async fn merging_streams_example() {
    println!("--- Example 4: Merging Streams ---");

    // Create a fast stream (numbers every 100ms)
    let fast_stream = stream::iter(1..=5).then(|n| async move {
        sleep(Duration::from_millis(100)).await;
        format!("Fast-{}", n)
    });

    // Create a slow stream (letters every 250ms)
    let slow_stream = stream::iter(['A', 'B', 'C']).then(|c| async move {
        sleep(Duration::from_millis(250)).await;
        format!("Slow-{}", c)
    });

    // Merge them - items come as they're ready
    // let mut merged = select(fast_stream, slow_stream);

    // while let Some(item) = merged.next().await {
    //     println!("Merged: {}", item);
    // }
    println!();
}
