fn main() {
    println!("Hello, world!");
    print_number(23);
    print_sum(3, 4);
    println!("add_one: {}", add_one(24));
    //diverges();
    let x:i32 = diverges();
    
    let f: fn(i32) -> i32 = plus_one;
    let f = plus_one;

    let six = f(5);
}


fn print_number(x : i32){
    println!("x is {}", x);
}

//fn print_sum(x, y){
fn print_sum(x:i32, y:i32){
    println!("sum is {}", x + y);
}

fn add_one(x: i32) -> i32{
    // x + 1; // 注意这里多出来的分号
    x + 1
}

fn diverges () -> !{
    panic!("This function never returns!");
}

fn plus_one(i: i32) -> i32{
    i + 1
}
