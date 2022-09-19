fn main() {
    // The following string variable is immutable using the String type
    let s1 = String::from("Hello Mateo, how have you been doing?");
   println!("{}", first_word(&s1));

   println!("{}", first_word_slice(&s1));


	let string_var = String::from("Hello world!");
	let slicing_string = &string_var[..2];
	println!("{}", slicing_string);
}


fn first_word(s: &String)-> usize{
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        
        }
    }
    s.len()
}

fn first_word_slice(s: &String)-> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}