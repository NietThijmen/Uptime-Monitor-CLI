use std::env;

mod system_stats;
mod http;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <url>", args[0]);
        return;
    }

    let target_url = &args[1]; // "https://uptime-monitor.test/api/system-stats/{id}"
    let sys = system_stats::sys_usage();
    let res = http::post_system_stats(target_url.to_string(), sys).await;

    println!("Uploaded system stats");
    println!("Status: {}", res.status);
    println!("Success: {}", res.success);
}
