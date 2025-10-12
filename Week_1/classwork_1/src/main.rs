fn main() {
    let celsius_temp = 36.5;
    let fahrenheit_temp = 100.0;

    let newfahrenheit_temp = ( celsius_temp * (9.0/5.0)) + 32.0;
    let newcelsius_temp = ( fahrenheit_temp - 32.0) * 5.0/9.0;

    println!("Smart Weather Temperature Converter");
    println!("The converted tempereratures are {}F and {}C", newfahrenheit_temp , newcelsius_temp);
}
