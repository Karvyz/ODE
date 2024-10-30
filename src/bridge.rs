use std::sync::{atomic::AtomicBool, Arc, Mutex, MutexGuard};

use crate::object::Object;

#[derive(Default, Clone)]
pub struct Bridge {
    objects: Arc<Mutex<Vec<Object>>>,

    sync_flag: Arc<AtomicBool>,

    window_killed: Arc<AtomicBool>,
}

impl Bridge {
    pub fn get_objects_mut(&self) -> MutexGuard<Vec<Object>> {
        self.objects.lock().unwrap()
    }

    pub fn try_get_objects_mut(&self) -> Option<MutexGuard<Vec<Object>>> {
        self.objects.try_lock().ok()
    }

    pub fn get_sync(&self) -> bool {
        let value = self.sync_flag.load(std::sync::atomic::Ordering::Relaxed);
        self.sync_flag
            .store(false, std::sync::atomic::Ordering::Relaxed);
        value
    }

    pub fn set_sync(&self) {
        self.sync_flag
            .store(true, std::sync::atomic::Ordering::Relaxed);
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
