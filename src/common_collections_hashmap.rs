use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let red_team: (String, u32) = ("Red".to_string(), 30);
    scores.insert(String::from("Pink"), 20);
    scores.insert(red_team.0, red_team.1);

    let pink_score = scores.get(&String::from("Pink"));
    if pink_score.is_some() {
        println!("Pink score is {}", pink_score.unwrap());
    } else {
        println!("Pink have no score :(");
    }
    scores.entry(String::from("Dark")).or_insert(100);
    for (key, value) in &scores {
        println!("{} -> {}", key, value);
    } 
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}