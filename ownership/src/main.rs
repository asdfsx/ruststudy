fn main() {
    println!("Hello, world!");

    let v = vec![1,2,3];
    let v2 = v;

    //下边的语句失败，因为没有所有权，所有权移动到了v2上
    //println!("v[0] is {}.",v[0]);

    let v = vec![1,2,3];
    take(v);

    //下边的语句失败，因为没有所有权，所有权移动到take函数里了
    //println!("v[1] is {}.", v[1]);

    let v = vec![1,2,3];
    let mut v2 = v;
    v2.truncate(2);
    //下面的语句失败，因为没有所有权，所有权移动到了v2
    //println!("v[2] is {}.", v[2]);

    let v = 1;
    let v2 = v;
    //基本类型实现了Copy函数，所以可以继续使用v
    println!("v is {}.", v);

    let a = 5;
    let _y = double(a);
    println!("{}", a);   

    let v = vec![1,2,3];
    let v = foo(v);
    println!("v[1] is {}", v[1]);
}

fn foo(v: Vec<i32>) -> Vec<i32>{
    v
}

fn double(x :i32) -> i32{
    x * 2
}

fn take(v: Vec<i32>){
    
}
