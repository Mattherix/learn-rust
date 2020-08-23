use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let name = read_name_from_file().expect("Can't read file");
    println!("{}", name);

    let name = read_username_from_file().expect("Can't read file");
    println!("{}", name);

    let name = fs::read_to_string("hello.txt").expect("Can't read file");
    println!("{}", name);
}

fn read_name_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
//fn main() {
//    let f = File::open("hello.txt");
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error)
//             }
//         },
//     };
//     // let f = File::open("hello2.txt").unwrap();
//     let f = File::open("hello2.txt").expect("Failed to open hello2.txt");
// }