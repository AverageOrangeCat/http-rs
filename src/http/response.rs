use std::io;
use std::io::Write;
use std::net::TcpStream;

pub enum HttpStatus {
    Ok,                     // 200
    NotFound,               // 404
    InternalServerError,    // 500
}

pub struct Headers { 
    stream: TcpStream,
    inner: String
}

impl Headers {
    pub fn new(stream: TcpStream) -> Headers {
        Headers { 
            stream, 
            inner: String::new()
        }
    }

    pub fn set<S: ToString>(&mut self, key: S, value: S) -> &mut Headers {
        self.inner += format!("{}: {}\r\n", key.to_string(), value.to_string()).as_str();
        self
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.inner += "\r\n";
        self.stream.write_all(self.inner.as_bytes())
    }
}

pub struct Body { 
    stream: TcpStream,
    inner: String
}

impl Body {
    pub fn new(stream: TcpStream) -> Body {
        Body { 
            stream, 
            inner: String::new()
        }
    }

    pub fn write<S: ToString>(&mut self, value: S) -> &mut Body {
        self.inner += format!("{}\r\n", value.to_string()).as_str();
        self
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.inner += "\r\n";
        self.stream.write_all(self.inner.as_bytes())
    }
}

pub struct Response { 
    headers: Headers,
    body: Body
}

impl Response {
    pub fn new(stream: &TcpStream) -> io::Result<Response> {
        let response = Response {
            headers: Headers::new(stream.try_clone()?),
            body: Body::new(stream.try_clone()?)
        };

        Ok(response)
    }

    pub fn headers(&mut self, http_status: HttpStatus) -> &mut Headers { 
        let inner = match http_status {
            HttpStatus::Ok => String::from("HTTP/1.1 200 Ok\r\n"),
            HttpStatus::NotFound => String::from("HTTP/1.1 404 Not found\r\n"),
            HttpStatus::InternalServerError => String::from("HTTP/1.1 500 Internal Server Error\r\n")
        };
        
        self.headers.inner = inner;
        &mut self.headers
    }

    pub fn body(&mut self) -> &mut Body { 
        self.body.inner = String::new();
        &mut self.body
    }
}
