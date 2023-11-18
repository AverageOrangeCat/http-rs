use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpStream;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct Request { pub inner: Vec<String> }

impl Request {
    pub fn new(stream: &TcpStream) -> io::Result<Request> {
        let buffer_reader = BufReader::new(stream);
        let mut inner = Vec::new();

        for result in buffer_reader.lines() {
            let line = result?;
            if line.is_empty() { break; }
            inner.push(line);
        }

        Ok( Request { inner } )
    }
}

impl Deref for Request {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Request {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
