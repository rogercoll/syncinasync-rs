use std::time::Duration;

trait Generator {
    fn generate(&self) -> Vec<i32>;
}

struct PlainGenerator;

async fn please_sleep() {
    // std::thread::sleep(Duration::from_millis(100));
    tokio::time::sleep(Duration::from_millis(100)).await;
}

impl PlainGenerator {
    async fn generate_async(&self) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..10 {
            res.push(i);
            eprintln!("Pushed value: {i}");
            please_sleep().await;
        }
        res
    }
}

impl Generator for PlainGenerator {
    fn generate(&self) -> Vec<i32> {
        futures::executor::block_on(async { self.generate_async().await })
    }
}

fn launch_generate(n: usize) {
    for i in 0..n {
        let _ = tokio::spawn(async move {
            let generator = PlainGenerator;
            eprintln!("Launching generator: {}", i);
            let vec = generator.generate();
            eprintln!("Generator: {} Result: {:?}", i, vec);
        });
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_span_events(FmtSpan::FULL)
        .init();

    launch_generate(1);
    let generator = PlainGenerator;
    eprintln!("Launching main generator");
    let vec = generator.generate();
    std::thread::sleep(Duration::from_secs(1));
    eprintln!("Main Result: {:?}", vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sync_method() {
        let generator = PlainGenerator;
        eprintln!("Launching generator!");
        let vec = generator.generate();
        eprintln!("Generator finished!");
        println!("vec: {:?}", vec);
    }
}
