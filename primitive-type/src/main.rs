fn main() {
    println!("Hello, world!");

    let x = true;
    let y: bool = false;

    let x = 'c';
    let y: char = 'd';
    //let x = '';

    let a = [1,2,3];
    let mut b = [1,2,3];
    let c = [0;20];
    println!("c has {} elements", c.len());
    println!("The second element is: {}", c[1]);

    let a = [1,2,3,4,5];
    let complete = &a[..];
    let middle = &a[1..4];

    let x = (1, "asdf");
    let x:(i32, &str) = x;
    //let x:(i32, str) = x;

    let mut x = (1, 2);
    let y = (2, 3);
    println!("x == y {}",   x == y);

    let (x, y, z,) = (1, 2, 3,);
    println!("x is {}.", x);

    let tuple = (1, 2, 3);
    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;
    println!("y is {}.", y);


    fn foo(x:i32) -> i32{x}
    let x: fn(i32) -> i32 = foo;
}
