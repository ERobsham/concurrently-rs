use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};

type GlobalCounter = Arc<Mutex<Option<usize>>>;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .build()
        .expect("must be able to build a runtime for this example");

    rt.block_on(run())
}

async fn run() {
    let global_state: GlobalCounter = Arc::new(Mutex::new(None));
    setup_global_state(&global_state).await;

    for i in 0usize.. {
        let h = tokio::spawn(update_and_log::<1>(global_state.clone()));

        if i % 5 == 1 {
            sleep(Duration::from_millis(50)).await;
            h.abort();
            println!("cancelled");
        } else {
            match h.await {
                Ok(_) => (),
                Err(_) => unreachable!("update_and_log is safe!"),
            };
        }
    }
}

async fn setup_global_state(state: &GlobalCounter) {
    let mut counter = state.lock().await;
    match *counter {
        Some(_) => {}
        None => {
            *counter = Some(0);
        }
    }
}

async fn update_and_log<const N: usize>(state: GlobalCounter) {
    let mut state_guard = state.lock().await;
    
    let mut count = state_guard.take().expect("always initialized at start!");
    println!("update_and_log{N} ==> {count}");
    count += 1;

    sleep(Duration::from_millis(100+N as u64)).await;
    
    *state_guard = Some(count);
}
