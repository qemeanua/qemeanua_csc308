use std::io;
#[derive(Debug)]

struct BankAccount{
    user : String,
    acc_balance : f32,
}

impl BankAccount{
    fn deposit (&mut self, d_amount: f32){
        self.acc_balance += d_amount;
        println!("You deposited N{}", d_amount);

    }

    fn check_balance(&self){
        println!{"{}'s balance is N{}", self.user , self.acc_balance};
    }

    fn withdraw(&mut  self, w_amount : f32){
        if w_amount <= self.acc_balance{
            self.acc_balance -= w_amount;
            println!("N{} was successfully withdrawn", w_amount);
        } 

        else {
            println!("Insufficient funds!");
        }
    }

    
}
fn main() {
    let mut name = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut name).expect("Failed to read name");
    let name = name.trim().to_string();

    let mut account = BankAccount{
        user: name,
        acc_balance : 0.0,
    };

    println!("Enter the amount you want to deposit:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let d_amount: f32 = input.trim().parse().expect("Please enter a valid number");

    account.deposit(d_amount);
    account.check_balance();
    println!("How much do you want to withdraw?:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let w_amount: f32 = input.trim().parse().expect("Please enter a valid number");
    
    account.withdraw(w_amount);
    account.check_balance();
}
