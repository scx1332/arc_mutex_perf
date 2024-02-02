use std::cell::UnsafeCell;

pub struct MutexOnlySync {
    val: UnsafeCell<i32>,
    notify: UnsafeCell<Option<tokio::sync::Notify>>,
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
            notify: None.into(),
        }
    }

    pub async fn lock(&self) {
        unsafe {
            if *self.val.get() == 0 {
                *self.val.get() = 1;
            } else {
                *self.val.get() += 1;
                if (*self.notify.get()).is_none() {
                    *self.notify.get() = Some(tokio::sync::Notify::new());
                }
                (*self.notify.get()).as_mut().unwrap().notified().await;
            }
        }
    }

    pub fn unlock(&self) {
        unsafe {
            if *self.val.get() == 1 {
                *self.val.get() = 0;
            } else {
                *self.val.get() -= 1;
                (*self.notify.get()).as_mut().unwrap().notify_one();
            }
        }
    }

    pub fn lock_count(&self) -> i32 {
        unsafe { *self.val.get() }
    }
}
