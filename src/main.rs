use std::io;
use std::net::TcpStream;
use std::sync::Mutex;

use experimental::http::error;
use experimental::http::host::Host;
use experimental::http::request::Request;
use experimental::http::response::HttpStatus;
use experimental::http::response::Response;
use experimental::http::result;
use once_cell::sync::Lazy;

static COUNTER: Lazy<Mutex<u8>> = Lazy::new(|| Mutex::new(0));

fn handle_stream(stream: &TcpStream) -> result::Result<()> {
    let Ok(request) = Request::new(stream) else {
        return Err(error::Error { message: "Request failed"});
    };

    let Ok(mut response) = Response::new(stream) else {
        return Err(error::Error { message: "Response failed"});
    };

    let headers: Vec<&str> = match request.get(0) {
        Some(request_item) => request_item.split(" ").collect(),
        None => return Err(error::Error { message: "Invalid Request"}),
    };

    if headers[0] == "GET" && headers[1] == "/" {
        let mut counter = COUNTER.lock().unwrap();

        *counter += 1;

        let _ = response.headers(HttpStatus::Ok)
        .set("Content-Type", "text/html")
        .flush();

        let _ = response.body()
        .write("<h1> 200 Ok </h1>")
        .write(format!("<p> {} </p>", counter))
        .flush();

        return Ok(());
    }
    
    let _ = response.headers(HttpStatus::NotFound)
    .set("Content-Type", "text/html")
    .flush();

    let _ = response.body()
    .write("<h1> 404 Not found </h1>")
    .flush();

    Ok(())
}

fn main() -> io::Result<()> {
    Host::new(handle_stream).bind("127.0.0.1:7878")?;
    Ok(())
}