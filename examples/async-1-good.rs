use std::time::Duration;

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("must be able to build a single-threaded runtime for this example");

    rt.block_on(run())
}

async fn run() {
    let _ = tokio::join!(first_loop(), second_loop());
}

async fn first_loop() {
    for count in 0usize.. {
        println!("first_loop  ==> {count}");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn second_loop() {
    for count in 0usize.. {
        println!("second_loop ==> {count}");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
