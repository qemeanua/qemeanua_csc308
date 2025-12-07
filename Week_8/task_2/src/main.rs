use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {

    let output = Command::new("echo")
        .arg("Hello from child")
        .output()
        .expect("Failed to run echo");

    let text = String::from_utf8_lossy(&output.stdout);

    let mut file = File::create("output.txt").expect("Failed to create file");

    file.write_all(text.as_bytes()).expect("Failed to write to file");
   
}



//Write a Rust program that:
//Runs the command echo “Rust Process Management”
//Captures its stdout
//Writes the output to a file called output.txt
//After running the program, use cat output.txt to verify the output.
