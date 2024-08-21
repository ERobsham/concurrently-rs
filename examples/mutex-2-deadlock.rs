use std::{
    sync::{Arc, Mutex},
    time::Duration,
    usize,
};

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("must be able to build a single-threaded runtime for this example");

    rt.block_on(run())
}

async fn run() {
    let global_state = Arc::new(Mutex::new(0usize));

    let f1 = {
        let state = global_state.clone();
        counter_loop::<1>(state)
    };
    let f2 = {
        let state = global_state.clone();
        counter_loop::<2>(state)
    };

    let _ = tokio::join!(f1, f2);
}

async fn counter_loop<const N: usize>(state: Arc<Mutex<usize>>) {
    for count in 0usize.. {
        let mut state_count = state.lock().unwrap();
        *state_count += 1;
        println!("nice_loop{N} ==> {count} | {state_count}");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
