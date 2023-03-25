use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

#[tokio::main]
async fn main() {
    const THREAD_COUNT: u8 = 20;
    let mut handles = vec![];
    let interrupted = Arc::new(AtomicBool::new(false));

    for _ in 0..THREAD_COUNT - 1 {
        let interrupted_clone = interrupted.clone();
        handles.push(tokio::spawn(async {
            do_calculations_until_cancelled(interrupted_clone).await
        }));
    }

    sleep(Duration::from_secs(10));

    interrupted.store(true, Ordering::Relaxed);

    let mut number_of_calcs: u128 = 0;
    for handle in handles {
        number_of_calcs += handle.await.unwrap();
    }

    println!("{}", number_of_calcs);
}

async fn do_calculations_until_cancelled(interrupted: Arc<AtomicBool>) -> u128 {
    let mut number_of_calcs: u128 = 0;
    while !interrupted.load(Ordering::Relaxed) {
        let _ = 100 * 100;
        number_of_calcs += 1;
    }
    number_of_calcs
}
