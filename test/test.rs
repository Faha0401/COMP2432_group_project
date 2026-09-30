//pub fn add(a: i32, b: i32) -> i32 {
//    a + b
//}

// This is a really bad adding function, its purpose is to fail in this
// example.

use crate::add;
use crate::robot_worker;
use crate::spawn_workers;
use crate::produce_tasks;



#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

// FOR TESTING
// here is a some scenario that might need test:
//     1) 2 bot 1 task 1 zone -> test bot offline
//     2) 2 bot 2 task 1 zone -> test race condition 
//     3) 2 bot 2 task 2 zone -> test concurrency

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}





















/* 
// tests/scalability_test.rs

use projb_myself::HealthMonitor;      // ← Change "projb" to your actual [package.name] in Cargo.toml
use projb_myself::TaskQueue;
use projb_myself::ZoneAccess;
use projb_myself::{produce_tasks, robot_worker, spawn_workers};   // reuse your functions

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
struct RobotTask {
    zone_id: u32,
    description: String,
}

// Helper to run one scalability test
fn run_test(num_robots: usize, num_tasks: usize, test_name: &str) {
    println!("\n=== Scalability Test: {} ===", test_name);
    println!("Robots: {} | Tasks: {}", num_robots, num_tasks);

    let start = Instant::now();

    let task_queue = TaskQueue::<RobotTask>::new();
    let zone_access = ZoneAccess::new();
    let health_monitor = HealthMonitor::new();

    let _monitor_handle = health_monitor.start_monitor_thread(Duration::from_secs(12));

    let mut handles = vec![];

    // Reuse your spawn_workers function
    for i in 1..=num_robots {
        let tq = task_queue.clone();
        let za = zone_access.clone();
        let hm = health_monitor.clone();

        spawn_workers(&mut handles, i as u32, tq, za, hm);
    }

    // Reuse your produce_tasks function
    produce_tasks(&task_queue, num_tasks, 4);   // 4 zones

    println!("All tasks submitted. Waiting for robots to finish...");

    for handle in handles {
        let _ = handle.join();
    }

    let duration = start.elapsed();
    println!("Test '{}' completed in {:.2} seconds", test_name, duration.as_secs_f64());
    println!("Average time per task: {:.2} ms\n", 
             (duration.as_millis() as f64) / (num_tasks as f64));
}

// ==================== Tests ====================

#[test]
fn scalability_small() {
    run_test(5, 20, "Small Scale");
}

#[test]
fn scalability_medium() {
    run_test(20, 100, "Medium Scale");
}

#[test]
fn scalability_large() {
    run_test(50, 300, "Large Scale");
}

#[test]
#[ignore]   // remove #[ignore] when you want to run it
fn scalability_stress() {
    run_test(100, 500, "Stress Test");
}


    */