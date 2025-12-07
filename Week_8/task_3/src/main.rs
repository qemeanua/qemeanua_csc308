use std::process::{Command};
use std::thread::sleep;
use std::time::Duration;

fn main(){
    let mut child = Command::new("ping")
        .arg("google.com")
        .spawn()
        .expect("Failed to spawn ping process");

    println!("Ping process started with PID: {}", child.id());

    println!("Ping runs for 5 seconds...");
    sleep(Duration::from_secs(5));

    let _terminate = child.kill();

    let status = child.wait().unwrap();
    println!("Exited with: {}", status);
}