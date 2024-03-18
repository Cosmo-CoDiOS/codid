use std::sync::{Arc, Mutex};
use std::thread;

pub struct Observer<T> {
    callbacks: Arc<Mutex<Vec<Box<dyn Fn(T) + Send>>>>,
}

impl<T> Observer<T> {
    pub fn new() -> Self {
        Observer {
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn subscribe<F>(&self, callback: F)
    where
        F: Fn(T) + Send + 'static,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.push(Box::new(callback));
    }

    pub fn notify(&self, data: T) {
        let callbacks = self.callbacks.lock().unwrap();
        for callback in callbacks.iter() {
            let data = data.clone();
            let callback = callback.clone();
            thread::spawn(move || {
                callback(data);
            });
        }
    }
}
