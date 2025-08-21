use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

// use threadpool::ThreadPool;
use hello::ThreadPool;

fn main() {
    #[cfg(any())]
    {
        // First we'll build a single-threaded web server working.
        // HTTP requests normally cannot listen to 7878, but TCP yes.
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // The bind here is like ::new in this scenario.
        // The function is called `bind` because in networking, connecting to a port to listen to is kniwn as `binding to a port`.
        // This returns Result<> because it is possible for binding to fail
        // For example, for port 80, you need to run the program as a root. So if not it will fail binding it.
        // Non administrater only higher than 1023
        // Of for example, we couldn't bind it if the port is already in use.

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established: {}", stream.peer_addr().unwrap());
        }
        // Right now we are iterating over the stream of incoming connection attempts. Not connections itself.
    }

    // Right now, the server will process each request in turn, meaning it won't process a second connection until the first is finished processing.
    // If the server received more and more requests, this serial execution would be less and less optimal. If the server receives a request that takes a long
    // time to process, subsequent requests will have to wait until the long reuqest is finished, even if the new requests can be processed quickly.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    // Is better to create a thread pool, so that we can limit the number of threads that are created.
    // We can use the `threadpool` crate for this. However, in this chapter we'll create our thread pool from scratch to understand how it works.
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // Multiple infinite threads for each connection (not good for DDOS attacks)
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
        pool.execute(move || {
            handle_connection(stream);
        })


        
    }

    // Let's now implement the `Thread pool` to be able to handle multiple requests at the same time.
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    // let http_request: Vec<_> = buf_reader;
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // The first unwrap takes care of the `Option` and stops if no items. The second unwrap takes care of the `Result` and stops if there is an error reading the line.
    // Here we check if the request is to / URI, so this response is concrete to that URI.
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}
