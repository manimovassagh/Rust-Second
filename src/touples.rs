pub fn run() {
    let persons: (&str, &str, &mut i32) = ("Mehdi", "Rahmaniyan", &mut 33);
    let _family_collection: (&str, &str, &mut i32) = ("Rahmanian", "Movassagh", &mut 33);
    println!(
        "My name is {} and my family is {} and i am {} years old",
        persons.0, persons.1, persons.2
    );
    println!("Tuple is Printing");
   
}
