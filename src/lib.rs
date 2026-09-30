// // src/lib.rs
// pub mod health;
// pub mod task_queue;
// pub mod zone_control;

// // Re-export the types so tests can use them easily
// pub use health::HealthMonitor;
// pub use task_queue::TaskQueue;
// pub use zone_control::ZoneAccess;


// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::thread::JoinHandle;
// use std::time::Duration;
// use std::time::Instant;


// #[derive(Clone, Debug)]
// struct RobotTask {
//     zone_id: u32,
//     description: String,
// }

// pub fn robot_worker(id: u32, tasks: TaskQueue<RobotTask>, zones: ZoneAccess, health: HealthMonitor) {
//     health.register_robot(id);

//     loop {
//         thread::sleep(Duration::from_millis(400));
//         // Signal a heartbeat everytime the robot awakes
//         health.heartbeat(id);

//         println!("[R{}] requesting task...", id);
//         let task = tasks.get_blocking();

//         println!(
//             "[R{}] got task: {} (zone {})",
//             id, task.description, task.zone_id
//         );

//         zones.enter_zone(task.zone_id);
//         println!("[R{}] entered zone {}", id, task.zone_id);

//         // Simulate work in zone (2–5 seconds)
//         let work_ms = 2000 + (id * 300) % 3000;
//         thread::sleep(Duration::from_millis(work_ms.into()));

//         println!("[R{}] leaving zone {}", id, task.zone_id);
//         zones.leave_zone(task.zone_id);
//     }
// }

// //A function to spawn a robot

// //Role: starts robot threads and returns handles
// //What it does:
// //for i in 1..=robots, clone shared resources
// //spawn a thread that runs robot_worker(i, ...)
// //store each JoinHandle in Vec
// //return handles

// pub fn spawn_workers(
//     vec: &mut Vec<JoinHandle<()>>,
//     id: u32,
//     task_queue: TaskQueue<RobotTask>,
//     zone_access: ZoneAccess,
//     health_monitor: HealthMonitor,
// ) {
//     //Spawn the new worker
//     vec.push(thread::spawn(move || {
//         robot_worker(id, task_queue, zone_access, health_monitor);
//     }));
// }

// pub fn produce_tasks(task_queue: &TaskQueue<RobotTask>, task_num: usize, zone_num: usize) {
//     //Distributing tasks into zones
//     for i in 1..task_num {
//         let zone = (i % zone_num).try_into().unwrap();
//         task_queue.submit(RobotTask {
//             zone_id: zone,
//             description: format!("Delivery #{}", i + 1),
//         });
//         thread::sleep(Duration::from_millis(300));
//     }
// }





