fn main() {
    println!("Hello, world!");

    let v1 = vec![1,2,3];
    let v2 = vec![1,2,3];

    let (v1, v2, number) = foo(v1, v2);

    let answer = borrow_foo(&v1, &v2);

    for i in &v1 {
        println!("after borrow_foo: {}", i);
    }


    let answer = foo1(&v1, &v2);
    println!("sum v1, v2, use borrow: {}", answer);

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("x is :{}.", x);

    //y的作用域必须小于x
    //let mut x = 5;
    //let y = &mut x;
    //*y += 1;
    //println!("x is :{}.", x);

    //x的声明必须在y之前
    //let y: &i32;
    //let x = 5;
    //y = &x;
    //println!("{}", y)

    //y的作用域大于x，所以失败
    //let y :&i32;
    //{
    //    let x = 5;
    //    y = &x;
    //}
    //println!("{}", y)

    //v在进入的时候已经做了mutable的借用，所以循环内就不能再次当作mutable来使用了
    //在同一个作用域内，只能有一个mutable的借用
    //let mut v = vec![1,2,3];
    //for i in &mut v {
    //    println!("{}",i);
    //    v.push(34);
    //}

    //v在循环的时候已经做了immutable的借用，所以循环内的v不能修改
    //let mut v = vec![1,2,3];
    //for i in & v {
    //    println!("{}",i);
    //    v.push(34);
    //}   
}

fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    (v1, v2, 42)
}

fn borrow_foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32{
    42
}

fn sum_vec(v: &Vec<i32>) -> i32 {
    return v.iter().fold(0, |a, &b| a+b);
}

fn foo1(v1: &Vec<i32>, v2: &Vec<i32>) -> i32{
    let s1 = sum_vec(v1);
    let s2 = sum_vec(v2);

    s1 + s2
}

//fn foo_push(v1:&Vec<i32>) {
//    v1.push(5)
//}

fn foo_push2(v1: &mut Vec<i32>){
    v1.push(5)
}


