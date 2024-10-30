use std::{thread::sleep, time::Duration};

use glam::Vec3;
use ode::object::Object;

fn main() {
    let bridge = ode::run();
    {
        let mut objects = bridge.get_objects_mut();
        for i in 0..100 {
            for j in 0..100 {
                for k in 0..100 {
                    objects.push(Object::new(Vec3::new(i as f32, j as f32, k as f32)));
                }
            }
        }
    }
    loop {
        sleep(Duration::from_millis(10));
        if bridge.window_killed() {
            break;
        }

        if bridge.get_sync() {
            println!("sync");
        }
    }
}
