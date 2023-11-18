use std::io;
use std::net::TcpListener;
use std::net::ToSocketAddrs;

use super::call::Call;
use super::streams::Streams;
use super::worker::Worker;

pub struct Host { call: Call }

impl Host {
    pub fn new(call: Call) -> Host {
        Host { call }
    }

    pub fn bind<A: ToSocketAddrs>(&self, address: A) -> io::Result<()> {
        let listener = TcpListener::bind(address)?;
        let streams = Streams::new();
    
        let worker_01 = Worker::spawn(streams.clone(), self.call);
        let worker_02 = Worker::spawn(streams.clone(), self.call);
        let worker_03 = Worker::spawn(streams.clone(), self.call);
        let worker_04 = Worker::spawn(streams.clone(), self.call);
        let worker_05 = Worker::spawn(streams.clone(), self.call);
    
        for stream in listener.incoming().filter_map(Result::ok) {
            streams.lock().push(stream);
    
            worker_01.unpark();
            worker_02.unpark();
            worker_03.unpark();
            worker_04.unpark();
            worker_05.unpark();
        }
    
        Ok(())
    }
}
