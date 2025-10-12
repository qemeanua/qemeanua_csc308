fn main() {
    let original_bill = 12000.0;

    if original_bill > 5000.0 && original_bill <= 9999.0 {
       let b = original_bill * 0.1 ;
       let final_bill = original_bill - b;
       println!("Here's your bill");
       println!("Original Bill : N{}", original_bill);
       println!("Discount Applied : 10%");
       println!("Final Bill : N{}", final_bill);
    }

    else if original_bill > 10000.0{
       let c = original_bill * 0.15 ;
       let final_bill = original_bill - c;
       println!("Here's your bill");
       println!("Original Bill : N{}", original_bill);
       println!("Discount Applied : 15%");
       println!("Final Bill : N{}", final_bill);

    }

    else{
        println!("No discount otherwise.");
    }
}
