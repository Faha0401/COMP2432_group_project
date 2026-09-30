use std::collections::VecDeque;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

use crate::manager::RobotTask;

#[derive(Clone)]
// Using a VecDeque so the first item in the vec can be poped
pub struct TaskQueue {
    queue: Arc<Mutex<VecDeque<RobotTask>>>,
    shutdown: Arc<AtomicBool>,
}

impl TaskQueue {
    pub fn new(shutdown: Arc<AtomicBool>) -> Self {
        TaskQueue {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            shutdown,
        }
    }

    // Add a new task to the queue
    pub fn add_task(&self, task: RobotTask) {
        let mut queue = self.queue.lock().unwrap();
        let task_des = task.description().to_string();
        queue.push_back(task);
        println!("Task [{}] submitted to queue", task_des);
    }

    // Robot calls this to get a task. 
    // It will sleep until a task is available.
    pub fn get_blocking(&self, id: u64 ) -> Option<RobotTask> {
        let mut not_checked = true;
        loop {
            // terminate
            if self.shutdown.load(Ordering::SeqCst) {
                return None;
            }
            // Lock the queue so we can safely check it
            let mut queue = self.queue.lock().unwrap();
            
            // See if there is any task in the front of queue
            let task = queue.pop_front();
            match task{
                Some(task) => {
                    return Some(task); 
                }
                None => {
                    // Release the lock if there is no task left
                    // Wait for 2 second before reqesting another task
                    if not_checked {
                        println!("[Robot {}] get no task (Task queue is empty)", id);
                        not_checked = false;
                    }
                    drop(queue);
                    thread::sleep(Duration::from_millis(2000));
                }
            }
        }
    }
}