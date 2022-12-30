pub mod queue {
    use std::{collections::VecDeque, sync::{Mutex, Arc}};

    // Queue stores all the messages
    pub struct Queue {
        queue_mutex: Arc<Mutex<VecDeque<String>>>
    }


    // create a new Queue
    pub fn new_queue() -> Queue {
        let queue_mutex = Arc::new(Mutex::new(VecDeque::new()));
        let queue = Queue{queue_mutex};
        return queue
    }


    impl Queue {
        // add message to queue
        pub fn add_message(&mut self, message: String) {
            let mut queue = self.queue_mutex.lock().unwrap();
            queue.push_front(message);
        }

        // pop message from queue
        pub fn retrieve_message(&mut self) -> String {
            let mut queue = self.queue_mutex.lock().unwrap();

            if queue.is_empty() {
                return String::new();
            }
            return queue.pop_back().unwrap();
        }
    }

}