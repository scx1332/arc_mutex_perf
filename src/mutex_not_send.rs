use std::cell::RefCell;

pub struct MutexOnlySync {
    val: RefCell<i32>,
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
            val: RefCell::new(0),
            notify: tokio::sync::Notify::new(),
        }
    }

    pub async fn lock(&self) {
        if *self.val.borrow() == 0 {
            *self.val.borrow_mut() = 1;
        } else {
            *self.val.borrow_mut() += 1;
            self.notify.notified().await;
        }
    }

    pub async fn busy_lock(&self) {
        loop {
            if *self.val.borrow() == 0 {
                *self.val.borrow_mut() = 1;
                break;
            } else {
                tokio::task::yield_now().await;
            }
        }
    }

    pub fn unlock(&self) {
        if *self.val.borrow() == 1 {
            *self.val.borrow_mut() = 0;
        } else {
            *self.val.borrow_mut() -= 1;
            self.notify.notify_one();
        }
    }

    pub fn lock_count(&self) -> i32 {
        *self.val.borrow()
    }
}
