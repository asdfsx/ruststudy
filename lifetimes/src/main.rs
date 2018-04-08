fn main() {
    println!("Hello, world!");

    let y = &5;
    let f = Foo{x:y};
    println!("{}", f.x);
    println!("x is {}", f.x());

    println!("x_on_y: {}.", x_on_y(f.x,f.x));
    println!("x_on_y2: {}.", x_on_y2(f.x, &23));

    let x: &'static str = "Hello, world.";

    static FOO: i32 = 5;
    let x: &'static i32 = &FOO;


    let a = 5;
    let x : &i32;
    {
        let y = &a;
        let f = Foo{x:y};
        x = &f.x;
    }
    println!("f.x in x {}.", x);
}

fn x_on_y2<'a, 'b>(x:&'a i32, y:&'b i32) -> &'a i32{
    x
}

fn x_on_y<'a>(x:&'a i32, y:&'a i32) -> &'a i32{
    x
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32{self.x}
}

struct Foo<'a>{
    x: &'a i32,
}

fn foo(x: i32){
}

fn bar<'a>(x: &'a i32){
}
