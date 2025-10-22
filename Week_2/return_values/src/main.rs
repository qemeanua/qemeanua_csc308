fn main() {
   let result :i32 = square(4);
   println!("Square is {}", result);
}

fn square(num : i32)-> i32{
    num * num
}