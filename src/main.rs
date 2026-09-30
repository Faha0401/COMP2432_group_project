use crate::manager::start;

mod health;
mod task_queue;
mod zone_control;
mod manager;

//demo
fn main() {
    let bot_num= 5;
    let task_num= 10;
    let zone_num= 4;
    let timeout= 8;
    start(bot_num, task_num, zone_num, timeout);
}