use std::sync::{atomic::AtomicBool, Arc, Mutex, MutexGuard};

use crate::object::Object;

#[derive(Default, Clone)]
pub struct Bridge {
    objects: Arc<Mutex<Vec<Object>>>,

    window_killed: Arc<AtomicBool>,
}

impl Bridge {
    pub fn get_objects_mut(&self) -> MutexGuard<Vec<Object>> {
        self.objects.lock().unwrap()
    }

    pub fn try_get_objects_mut(&self) -> Option<MutexGuard<Vec<Object>>> {
        self.objects.try_lock().ok()
    }

    pub fn window_killed(&self) -> bool {
        self.window_killed
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_window_killed(&self, value: bool) {
        self.window_killed
            .store(value, std::sync::atomic::Ordering::Relaxed)
    }
}
