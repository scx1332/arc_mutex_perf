use arc_mutex_perf::mutex_not_send::MutexOnlySync;
use std::rc::Rc;
use std::time::Duration;

//tokio tests

#[tokio::test]
async fn test() {
    let lock = MutexOnlySync::new();
    lock.lock().await;
    lock.unlock();
    lock.lock().await;
    lock.unlock();
}

#[tokio::test]
async fn multiple_test() {
    let lock = Rc::new(MutexOnlySync::new());

    let lck = lock.clone();
    let fut1 = async move {
        lck.lock().await;
        println!("Fut1 started");
        tokio::time::sleep(Duration::from_millis(10)).await;
        println!("Lock count: {}", lck.lock_count());
        tokio::time::sleep(Duration::from_millis(200)).await;
        println!("Fut1 ended");
        lck.unlock();
    };
    let lck = lock.clone();
    let fut2 = async move {
        lck.lock().await;
        println!("Fut2 started");
        println!("Lock count: {}", lck.lock_count());
        tokio::time::sleep(Duration::from_millis(200)).await;
        println!("Fut2 ended");
        lck.unlock();
    };
    let lck = lock.clone();
    let fut3 = async move {
        lck.lock().await;
        println!("Fut3 started");
        println!("Lock count: {}", lck.lock_count());
        tokio::time::sleep(Duration::from_millis(200)).await;
        println!("Fut3 ended");
        lck.unlock();
    };

    tokio::join!(fut1, fut2, fut3);

    println!("Lock count: {}", lock.lock_count());

    assert_eq!(lock.lock_count(), 0)
}
