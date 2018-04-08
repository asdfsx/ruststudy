use std::env;

fn main() {
    //let mut tomlfile = String::new();
    //
    //for argument in env::args() {
    //    let tmp = &*argument;
    //    if tmp.contains("toml=") {   
    //        tomlfile = (&tmp[5 .. tmp.len()]).to_string();
    //    }
    //}

    let mut tomlfile = String::new();
    for argument in env::args() {
        if argument.contains("toml=") {
            //tomlfile = (&argument[5..]).to_string();
            tomlfile.push_str(&argument[5 ..]);
        }
    }
    println!("{}", tomlfile);
}

