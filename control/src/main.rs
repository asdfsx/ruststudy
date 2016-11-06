fn main() {
    println!("Hello, world!");

    let x = 5;

    if x == 5 {
        println!("test = 5.");
    } else {
        println!("test != 5.");
    }

    if x == 5 {
        println!("test = 5.");
    } else if x == 56 {
        println!("test = 56.");
    }

    let y = if x == 5 {
        10
    } else {
        5
    };
    println!("y is {}.", y);

    let y = if x == 5 { 10 } else { 5 };
    println!("y is {}.", y); 

    let mut i = 0;
    loop {
        if i == 10 {
            break;
        } 
        println!("i is {}", i);
        i += 1;
    }

    let mut x = 5;
    let mut done = false;
    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 {
            done = true;
        }
    }

    for x in 0..10 {
        println!("{}", x);
    }

    for (i, j) in (5..10).enumerate(){
        println!("i = {} and j = {}", i, j);
    }

    let lines = "hello\nworld".lines();
    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line)
    }

    for x in 0..10{
        if x % 2 == 0 {continue}
        println!("{}", x)
    }

    'outer: for x in 0..10{
        'inner: for y in 0..10{
            if x % 2 == 0 {continue 'outer;}
            if y % 2 == 0 {continue 'inner;}
            println!("x: {}, y: {}", x, y)
        }
    }
}
