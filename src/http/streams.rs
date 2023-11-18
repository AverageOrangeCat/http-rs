use std::net::TcpStream;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

#[derive(Clone)]
pub struct Streams { inner: Arc<Mutex<Vec<TcpStream>>> }

impl Streams {
    pub fn new() -> Streams {
        Streams { inner: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn lock<'m>(&'m self) -> MutexGuard<'m, Vec<TcpStream>> {
        match self.inner.lock() {
            Ok(guard) => guard,
            Err(poison) => poison.into_inner(),
        }
    }
}

impl Deref for Streams {
    type Target = Arc<Mutex<Vec<TcpStream>>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Streams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
