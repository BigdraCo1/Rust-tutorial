fn main() {
    println!("Hello from Rust, Trevor Sullivan!");
    types(); 
    type_conversion();
    mutable_variable();
    strings();
    tuples_val();
    arrays();
    slices();
}

// Unit type
fn types() {
    // declare a variable x as unit type
    let x: () = ();
    println!("{:?}", x);
}

fn type_conversion() {
    let x: f32 = 42.0;
    let y: i32 = x as i32 - 5; // To do arithmetic operations, it must be converted to the same type
    println!("{:?}", y);
}

// variable is immutable by default
fn mutable_variable() {
    let mut x: bool = true; // Add mut keyword to make the variable mutable
    println!("{:?}", x);
    x = false;
    println!("{:?}", x);
}

fn strings() {
    let a: char = '🔥';
    println!("If you know {} emoji. You're cooked", a);
    let x: &str = "Hello, world!";
    println!("{}", x);
    let y: String = x.to_string();
    println!("{}", y);
}

fn tuples_val() {
    let x: (i32, f64, u8) = (500, 6.4, 1); 
    let (a, b, c) = x;
    println!("{}, {}, {}", a, b, c);
    println!("{}, {}, {}", x.0, x.1, x.2);
}

fn arrays() {
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // [type; size]
    let first: i32 = a[0];
    let second: i32 = a[1];
    println!("{}, {}", first, second);
}

fn slices() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..=3]; // Add = to include the last element
    let slice2: &[i32] = &a[..3]; 
    println!("{:?}", slice2);
    println!("{:?}", slice);
}