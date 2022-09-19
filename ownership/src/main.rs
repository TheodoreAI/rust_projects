fn main() {
    // The following string variable is mutable using the String type
    // that is stored in the heap so that we don't have to know the size of the string at compile time
    let s1 = String::from("Hello, Mateo");
    takes_ownership(s1);
    // println!("{}", s1); // This line will not compile because s1 was moved to the function takes_ownership
    let x = 5;
    makes_copy(x);

    let _s2 = gives_ownership();
    let s3 = String::from("Hello, Mr. Estrada");
    let s4 = takes_and_gives_back(s3);
    let len = calculate_length(&s4);
    println!("The length of '{}' is {}.", s4, len);
}


fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn gives_ownership() -> String{
    let some_string = String::from("Hello, Mr. Mateo Estrada");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    a_string
}

fn calculate_length(s: &String) -> usize{
    let length = s.len();
    length
}