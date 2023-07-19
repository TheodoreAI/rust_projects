
fn main() {
    another_function(5, 9);
    let x = five();
    println!("The return value of x is: {}", x);

    let y = plus_one(5);
    println!("The value of plus_one is: {}", y);
    const SIZE : u32 = 10;
    println!("The array is: {}", SIZE);
}

fn five() -> u32 {
    5
}

fn plus_one(x: i32) -> i32{
    x + 1
}

fn another_function(x: u32, y: u32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}


    
