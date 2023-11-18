use std::thread;
use std::thread::JoinHandle;

use super::call::Call;
use super::response::HttpStatus;
use super::response::Response;
use super::streams::Streams;

fn handle_stream(streams: &mut Streams, call: Call) -> () {
    loop {
        let Some(stream) = ({ streams.lock().pop() }) else {
            thread::park();
            continue;
        };
    
        if let Err(error) = call(&stream) {
            let mut response = Response::new(&stream).unwrap();
    
            let _ = response.headers(HttpStatus::InternalServerError)
            .set("Content-Type", "text/html")
            .flush();
            
            let _ = response.body()
            .write(error.message)
            .flush();
        };
    }
}

pub struct Worker { inner: JoinHandle<()> }

impl Worker {
    pub fn spawn(mut streams: Streams, call: Call) -> Worker {
        Worker { inner: thread::spawn(move || handle_stream(&mut streams, call)) }
    }

    pub fn unpark(&self) -> () {
        self.inner.thread().unpark();
    }
}
