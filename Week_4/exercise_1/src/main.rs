use std::io;

fn last_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    
    for (i,&item) in bytes.iter().enumerate().rev(){
        if item == b' '{
            return &s[i + 1..];
        }
    }
    s
}
fn main() { 
    let mut name = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut name).expect("Failed to read input");

let last = last_word(&name);
println!("The last_word is : {}" , last);

}
