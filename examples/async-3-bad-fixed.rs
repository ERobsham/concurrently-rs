use std::time::Duration;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .worker_threads(1)
        .max_blocking_threads(1)
        .build()
        .expect("must be able to build a single-threaded runtime for this example");

    rt.block_on(run())
}

async fn run() {
    let _ = tokio::join!(nice_loop(), evil_loop_fixed());
}

async fn nice_loop() {
    for count in 0usize.. {
        println!("nice_loop ==> {count}");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn evil_loop_fixed() {
    for count in 0usize.. {
        println!("evil_loop ==> {count}");

        let rt = tokio::runtime::Handle::current();
        rt.spawn_blocking(move || std::thread::sleep(Duration::from_millis(100)))
            .await
            .unwrap();
    }
}
