use std::{
    sync::{Arc, Mutex},
    thread,
};

use executor::Executor;
use storage::{Database, Storage};

pub struct Runtime {
    db: Arc<Mutex<Database>>,
    executor: Executor,
}

impl Runtime {
    pub fn new(db: Arc<Mutex<Database>>, key_pair: executor::KeyPair) -> Self {
        let executor = Executor::new(db.clone(), key_pair);
        Runtime { db, executor }
    }

    pub fn start(&self) {
        let executor_clone = self.executor.clone();
        thread::spawn(move || executor_clone.run());
    }

    pub fn submit_task(&self, task: executor::Task) {
        self.db.lock().unwrap().push_task(task);
    }

    pub fn get_database(&self) -> Arc<Mutex<Database>> {
        self.db.clone()
    }
}

impl Default for Runtime {
    fn default() -> Self {
        let db = Arc::new(Mutex::new(Database::new()));
        let key_pair = executor::KeyPair::generate(&mut OsRng);
        Runtime::new(db, key_pair)
    }
                       }
