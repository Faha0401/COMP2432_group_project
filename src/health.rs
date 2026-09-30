use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone)]
// Storing the duration since last heartbeat and a flag for online or offline indication
pub struct RobotStatus {
    last_heartbeat: Instant,
    online: bool,
}

impl RobotStatus {
    fn new_online() -> Self {
        RobotStatus {
            last_heartbeat: Instant::now(),
            online: true,
        }
    }

    fn heartbeat(&mut self) {
        self.last_heartbeat = Instant::now();
        self.online = true;
    }

    fn elapsed_since_heartbeat(&self) -> Duration {
        self.last_heartbeat.elapsed()
    }

    fn is_online(&self) -> bool {
        self.online
    }

    fn set_online(&mut self, online: bool) {
        self.online = online;
        if online {
            self.last_heartbeat = Instant::now();
        }
    }
}

#[derive(Clone)]
// Storing the robot id with the RobotStatus
pub struct HealthMonitor {
    statuses: Arc<Mutex<HashMap<u64, RobotStatus>>>,
}

impl HealthMonitor {
    pub fn new() -> Self {
        HealthMonitor {
            statuses: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_robot(&self, robot_id: u64) {
        let mut health = self.statuses.lock().unwrap();
        health.insert(robot_id, RobotStatus::new_online());
    }

    // Insert the robot id and the last hearbeat as the current time and set the online status as true
    pub fn heartbeat(&self, robot_id: u64) {
        let mut health = self.statuses.lock().unwrap();
        if let Some(status) = health.get_mut(&robot_id) {
            status.heartbeat();
        } else {
            health.insert(robot_id, RobotStatus::new_online());
        }
    }

    // Getter
    pub fn is_online(&self, robot_id: u64) -> Option<bool> {
        let health = self.statuses.lock().unwrap();
        health.get(&robot_id).map(|status| status.is_online())
    }

    // Setter
    pub fn set_online(&self, robot_id: u64, online: bool) {
        let mut health = self.statuses.lock().unwrap();
        if let Some(status) = health.get_mut(&robot_id) {
            status.set_online(online);
        } else if online {
            health.insert(robot_id, RobotStatus::new_online());
        }
    }

    pub fn start_monitor_thread(&self, timeout: Duration, shutdown: Arc<AtomicBool>) -> thread::JoinHandle<()> {
        let monitor = self.clone();
        let mut timer = Instant::now();

        thread::spawn(move || {
            println!("Monitor Created");
            thread::sleep(Duration::from_secs(1));
            loop {
                let mut map = monitor.statuses.lock().unwrap();
                let mut max_duration = Duration::from_millis(0);
                let mut all_offline = true;

                for (robot_id, status) in map.iter_mut() {
                    // Calculate if the duration from the last heartbeat till now is larger than the timeout threshhold
                    let duration = status.elapsed_since_heartbeat();

                    // find max duration
                    if duration > max_duration {
                        max_duration = duration;
                    }
                    // check timeout
                    if status.is_online() && duration > timeout {
                        status.set_online(false);
                        println!("[Monitor] Robot {} timeout -> marked offline", robot_id);
                    }
                    // update all_offline
                    if status.is_online() {
                        all_offline = false;
                    }
                }
                drop(map);
                // Terminate when the last 
                    if all_offline {
                        if timer.elapsed() >= Duration::from_secs(10) {
                            shutdown.store(true, Ordering::SeqCst);
                        println!("---------------------------------------");
                        println!("[Monitor] Monitor terminated");
                        return;
                    }
                } else {
                    timer = Instant::now();
                }
                thread::sleep(Duration::from_secs((timeout.saturating_sub(max_duration)).as_secs().max(1)));
            }
        })
    }

    // Return the 
    // pub fn get_status(&self, robot_id: u32) -> bool {
    //     let health = self.statuses.lock().unwrap();
    //     let on = health.get(&robot_id);
    //     return on.unwrap().online
    // }
}

