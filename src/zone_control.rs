use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use crate::health::HealthMonitor;

#[derive(Clone)]
struct ZoneState {
    occupied_by: Option<u64>,
}

impl ZoneState {
    fn new() -> Self {
        ZoneState { occupied_by: None }
    }

    fn is_free(&self) -> bool {
        self.occupied_by.is_none()
    }

    fn occupy(&mut self, robot_id: u64) {
        self.occupied_by = Some(robot_id);
    }

    fn occupied_by(&self, robot_id: u64) -> bool {
        self.occupied_by == Some(robot_id)
    }

    fn release(&mut self) {
        self.occupied_by = None;
    }
}

#[derive(Clone)]
pub struct ZoneAccess {
    zones: Arc<Mutex<HashMap<u64, ZoneState>>>,
}

impl ZoneAccess {
    /// Creates a new zone controller
    pub fn new() -> Self {
        ZoneAccess {
            zones: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn enter_zone(&self, zone_id: u64, health: HealthMonitor, robot_id: u64, shutdown: Arc<AtomicBool>) -> bool {
        loop {
            if shutdown.load(Ordering::SeqCst) {
                return false;
            }

            let mut zones_guard = self.zones.lock().unwrap();

            // Get the state of this zone 
            let zone_state = zones_guard
                .entry(zone_id)
                // Insert the zone state if the zone does not exist yet.
                .or_insert_with(ZoneState::new);

            if zone_state.is_free() {
                zone_state.occupy(robot_id);
                return true;
            }

            // If the zone is occupied drop the guard first and wait for 3 sec

            println!("[Robot {}] Zone {} is occupied, waiting...", robot_id, zone_id);
            drop(zones_guard);
            for _ in 1..=10 {
                if shutdown.load(Ordering::SeqCst) {
                    return false;
                }
                health.heartbeat(robot_id);
                thread::sleep(Duration::from_millis(300));
            }
            
        }
    }

    pub fn leave_zone(&self, zone_id: u64, robot_id: u64) {
        let mut zones_guard = self.zones.lock().unwrap();

        // Set the zone state back to free only when called by zone owner.
        if let Some(zone_state) = zones_guard.get_mut(&zone_id) {
            if zone_state.occupied_by(robot_id) {
                zone_state.release();
                return;
            }
        }
    }
}


