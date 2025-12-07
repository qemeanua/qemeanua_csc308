use std::process::Command;

fn main() {
    let child_1 = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to spawn");

    let child_2 = Command::new("ls")
        .arg("-la")
        .spawn()
        .expect("Failed to execute command");

    let child_3 = Command::new("echo")
        .arg("Hello from child")
        .spawn()
        .expect("Failed to spawn echo");

    println!("Child 1 (sleep) PID: {}", child_1.id());
    println!("Child 2 (ls) PID: {}", child_2.id());
    println!("Child 3 (task3) PID: {}", child_3.id());

}





//Write a Rust program that spawns three child processes:
//One that runs sleep 5
//One that runs ls –la
//One that prints “Hello from child”
//Run the program
//Use ps, pstree or top to identify all three child processes.
//Write down their PIDs, PPIDs, and states (Sleeping, Running, etc.).
