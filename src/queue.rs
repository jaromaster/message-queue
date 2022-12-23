pub mod queue {
    use std::collections::VecDeque;

    // Queue stores all the messages
    pub struct Queue {
        queue: VecDeque<String>
    }


    // create a new Queue
    pub fn new_queue() -> Queue {
        let queue = Queue{queue: VecDeque::new()};
        return queue
    }


    impl Queue {
        // add message to queue
        pub fn add_message(&mut self, message: String) {
            self.queue.push_front(message);
        }

        // pop message from queue
        pub fn retrieve_message(&mut self) -> String {
            if self.queue.is_empty() {
                return String::new();
            }
            return self.queue.pop_back().unwrap();
        }
    }

}