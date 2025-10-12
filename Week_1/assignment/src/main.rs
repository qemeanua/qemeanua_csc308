use std::io;


fn main() {
    let base_rate = 20.0;
    let rate1 = 25.0;
    let rate2 = 30.0;

    let mut units_used = String::new();

    println!("Enter the units of electrity used :");
    io::stdin().read_line(&mut units_used).expect("Failed to read input");
    let units_used : f64 = units_used.trim().parse().expect("Please enter a valid number");

    if units_used > 1.0 && units_used <= 99.0{
        let bill = units_used * base_rate;
        println!("Your light bill is N{}",bill);
    }

    else if units_used > 100.0 && units_used <= 199.0{
        let bill = units_used * rate1;
        println!("Your light bill is N{}",bill);

    }

    else if units_used > 200.0{
        let bill = units_used * rate2;
        println!("Your light bill is N{}",bill);
    }

    else{
        println!("No outstanding bills");
    }



    
}
