use std::io;

fn main() {
    let mut name = String::new();
    let mut s_score = String::new();

    println!("Enter the student's name:");
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim();

    println!("Enter score:");
    io::stdin().read_line(&mut s_score).expect("Failed to read input");
    let score: f32 = s_score.trim().parse().expect("Please enter a valid number");

    if score >= 50.0{
        println!("{} passed the course!", name);

    } else {
        println!("{} failed the course, do better next time ", name);
    }
}
