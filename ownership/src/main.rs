fn main() {
    // The following string variable is mutable using the String type
    // that is stored in the heap so that we don't have to know the size of the string at compile time
    let mut string_variable = String::from("Hello, Mateo");
    println!("{}", string_variable);

    string_variable.push_str(" Estrada");

    println!("{}", string_variable);
}
