use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

use runtime::Runtime;
use executor::Task;

pub struct Substrate {
    runtime: Arc<Mutex<Runtime>>,
    task_queue: Arc<Mutex<VecDeque<Task>>>,
    storage: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl Substrate {
    pub fn new(runtime: Arc<Mutex<Runtime>>) -> Self {
        let task_queue = Arc::new(Mutex::new(VecDeque::new()));
        let storage = Arc::new(Mutex::new(HashMap::new()));
        Substrate { runtime, task_queue, storage }
    }

    pub fn submit_task(&self, task: Task) {
        self.task_queue.lock().unwrap().push_back(task);
        self.runtime.lock().unwrap().submit_task(task);
    }

    pub fn get_storage(&self, key: &str) -> Option<Vec<u8>> {
        self.storage.lock().unwrap().get(key).cloned()
    }

    pub fn set_storage(&self, key: &str, value: Vec<u8>) {
        self.storage.lock().unwrap().insert(key.to_string(), value);
    }

    pub fn run(&self) {
        self.runtime.lock().unwrap().start();
    }
}

impl Default for Substrate {
    fn default() -> Self {
        let runtime = Arc::new(Mutex::new(Runtime::default()));
        Substrate::new(runtime)
    }
    }
