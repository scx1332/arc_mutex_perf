use std::cell::{UnsafeCell};

pub struct MutexOnlySync {
    val: UnsafeCell<i32>,
    notify: tokio::sync::Notify,
}

impl Default for MutexOnlySync {
    fn default() -> Self {
        Self::new()
    }
}

impl MutexOnlySync {
    pub fn new() -> Self {
        Self {
            val: UnsafeCell::new(0),
            notify: tokio::sync::Notify::new(),
        }
    }

    pub async fn lock(&self) {

        unsafe {
            if *self.val.get() == 0 {
                *self.val.get() = 1;
            } else {
                *self.val.get() += 1;
                self.notify.notified().await;
            }
        }
    }

    pub fn unlock(&self) {
        unsafe {
            if *self.val.get() == 1 {
                *self.val.get() = 0;
            } else {
                *self.val.get() -= 1;
                self.notify.notify_one();
            }
        }
    }

    pub fn lock_count(&self) -> i32 {
        unsafe {
            *self.val.get()
        }
    }
}
