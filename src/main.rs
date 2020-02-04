use futures::{
    future::{BoxFuture, FutureExt},
    stream::{FuturesUnordered, StreamExt},
};
use rand::distributions::{Distribution, Uniform};
use std::thread;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;

fn main() {
    let start_time = Instant::now();

    let mut rt = Runtime::new().unwrap();
    rt.block_on(wait());

    // println!(
    //     "Program finished in {} ms",
    //     start_time.elapsed().as_millis()
    // );
}

async fn wait() {
    let between = Uniform::from(4000..9000);
    let mut rng = rand::thread_rng();

    let mut futures: FuturesUnordered<BoxFuture<Result<String, String>>> = FuturesUnordered::new();

    for future_number in 0..1000 {
        let random_millis = between.sample(&mut rng);
        futures.push(sleep(future_number, random_millis));
    }

    futures.push(never_ends());

    loop {
        if let Some(result) = futures.next().await {
            match result {
                Ok(val) => println!("👍 {}", val),
                Err(e) => eprintln!("👎 {}", e),
            }
        }
    }
}

fn never_ends() -> BoxFuture<'static, Result<String, String>> {
    async move {
        let mut tick: u64 = 0;
        let dur = Duration::from_millis(2000);
        let mut interval = tokio::time::interval(dur);
        loop {
            interval.tick().await;
            // print this every two seconds
            println!("tick {} complete", tick);
            tick += 1;
        }
    }
    .boxed()
}

fn sleep(num: u32, millis: u64) -> BoxFuture<'static, Result<String, String>> {
    async move {
        let dur = Duration::from_millis(millis);
        tokio::time::delay_for(dur).await;
        println!(
            "future {} slept for {} ms on {:?}",
            num,
            millis,
            thread::current().id()
        );
        let num_str = num.to_string();
        match num % 2 {
            0 => Err(num_str),
            _ => Ok(num_str),
        }
    }
    .boxed()
}
