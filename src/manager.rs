use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::health::HealthMonitor;
use crate::task_queue::TaskQueue;
use crate::zone_control::ZoneAccess;

#[derive(Clone, Debug)]
pub struct RobotTask {
    zone_id: u64,
    description: String,
}

impl RobotTask {
    pub fn new(zone_id: u64, description: String) -> Self {
        RobotTask { zone_id, description }
    }

    pub fn zone_id(&self) -> u64 {
        self.zone_id
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

// start the os
pub fn start(bot_num : u64, task_num : u64, zone_num: u64, timeout: u64) {
    let shutdown = Arc::new(AtomicBool::new(false));
    let robot_id = Arc::new(Mutex::new(0));
    let task_queue = TaskQueue::new(shutdown.clone());
    let zone_access = ZoneAccess::new();
    let health_monitor = HealthMonitor::new();
    
    let monitor_handle = health_monitor.start_monitor_thread(Duration::from_secs(timeout), shutdown.clone());

    let mut robots = vec![];

    for _ in 1..=bot_num {
        //cloning the modules prevent the other threads use the moved value
        let tq = task_queue.clone();
        let za = zone_access.clone();
        let hm = health_monitor.clone();
        let sd = shutdown.clone();

        //lock the id and do an increasement so every robot should have a unique id
        let mut id = robot_id.lock().unwrap();
        *id += 1;
        spawn_workers(&mut robots, *id, tq, za, hm, sd);
    }

    produce_tasks(&task_queue, task_num, zone_num);

   for robot in robots {
       robot.join().unwrap();
   }
   monitor_handle.join().unwrap();
   println!("All threads terminated.");
}

pub fn robot_worker(id: u64, tasks: TaskQueue, zones: ZoneAccess, health: HealthMonitor, shutdown: Arc<AtomicBool>) {
    health.register_robot(id);
    loop {
        // A small dalay when creating a robot to let the task submit to the task queue first
        thread::sleep(Duration::from_secs(1));

        if shutdown.load(Ordering::SeqCst) {
            println!("[Robot {}] shutting down", id);
            return;
        }

        health.heartbeat(id);

        println!("[Robot {}] requesting task...", id);
        let task = match tasks.get_blocking(id) {
            Some(t) => t,
            None => {
                println!("[Robot {}] shutting down", id);
                return;
            }
        };

        println!(
            "[Robot {}] got task: {} (zone {})",
            id, task.description(), task.zone_id()
        );

        let entered = zones.enter_zone(task.zone_id(), health.clone(), id, shutdown.clone());
        if !entered {
            println!("[Robot {}] shutdown while waiting for zone", id);
            return;
        }
        println!("[Robot {}] entered zone {}", id, task.zone_id());

        // simulating work time
        for _ in 1..10 {
            if shutdown.load(Ordering::SeqCst) {
                zones.leave_zone(task.zone_id(), id);
                println!("[Robot {}] shutting down during work", id);
                return;
            }
            health.heartbeat(id);
            thread::sleep(Duration::from_millis(300));
        }
        
        println!("[Robot {}] task {} completed", id, task.description());

        println!("[Robot {}] leaving zone {}", id, task.zone_id());
        zones.leave_zone(task.zone_id(), id);
    }
}

pub fn spawn_workers(
    vec: &mut Vec<JoinHandle<()>>,
    id: u64,
    task_queue: TaskQueue,
    zone_access: ZoneAccess,
    health_monitor: HealthMonitor,
    shutdown: Arc<AtomicBool>,
) {
    //Spawn the new worker
    // let hm1 = health_monitor.clone();
    vec.push(thread::spawn(move || {
        robot_worker(id, task_queue, zone_access, health_monitor, shutdown);
    }));
    println!("Robot {} created", id);
}

pub fn produce_tasks(task_queue: &TaskQueue, task_num: u64, zone_num: u64) {
    if zone_num == 0 {
        return;
    }
    //Distributing tasks into zones
    for i in 1..task_num+1 {
        let zone = i % zone_num + 1;
        let task = RobotTask::new(zone, format!("Task #{}", i));
        task_queue.add_task(task);
        thread::sleep(Duration::from_millis(20));
    }
}

// fn healthbeat_tick(id: u64, health: HealthMonitor){
//     loop {
//         health.heartbeat(id);
//         thread::sleep(Duration::from_millis(1000));
//     }
// }

