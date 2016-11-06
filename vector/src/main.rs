fn main() {
    println!("Hello, world!");
    let v = vec![0; 10];
    let v = vec![1, 2, 3, 4, 5];

    let i: usize = 1;
    let j: i32 = 1;

    println!("Item 1 is {}.", v[i]);
    //println!("Item 1 is {}.", v[j]);

    //println!("Item 7 is {}.", v[7]);

    match v.get(7) {
        Some(x) => println!("Item 7 is {}.", x),
        None => println!("Item 7 is None.")
    }

    let mut v = vec![1,2,3,4,5];
    for i in &v {
        println!("A refer to {}.", i);
    }

    for i in &v {
        println!("A refer to {}.Again!", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}.", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}.Again!", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}.", i);
    }

    //不能重复这样的遍历
    //for i in v {
    //    println!("Take ownership of the vector and its element {}.", i);
    //}
}
