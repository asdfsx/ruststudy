fn main() {
    println!("Hello, world!");
    let x = 5;
    let (a, b) = (1, 2);
    let c : i32 = 5;
    let d = 5; // d: i32
    // d = 10;
    let mut e = 5;
    e = 10;
    let f : i32;
    // print!("The value of f is: {}", f);
    
    let g : i32 = 17;
    {
        let h : i32 = 3;
        println!("The value of g is {} and value of h is {}.", g, h);
    }
    // println!("The value of g is {} and value of h is {}.", g, h);

    let i : i32 = 8;  
    {
        println!("The value of i is {}.", i);
        let i = 24;
        println!("The value of i is {}.", i);
    } 
    println!("The value of i is {}.", i);
    let i = 42;
    println!("The value of i is {}.", i);

    let mut j : i32 = 1;
    j = 5;
    let j = j;
    // j = 10;

    let y = 4;
    let y = "Test";
    
}
