fn main() {
    let mut name = String::from("Firstname");
    let returned_name = {
         add_surname_to_firstname(&mut name);
         &name
    };

    println!("{returned_name}");
}

fn add_surname_to_firstname(name: &mut String)
{
    name.push_str("Lastname");
}
