#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4];
    
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2];
    let second: &u32 = &v[1]; 
    // Can panic! (index outoff bound,
    // v.get() is better when we don't know the vector lenght)
    println!("{}", second);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("aa")
    }
    if let Some(3) = v.get(2) {
        println!("The third is 3");
    }
    
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for r in row {
        println!("{:?}", r);
    }
}