fn main() {
    let score = act_score_from_user();
    println!("The correspoding class is {}", to_class(score));
}

fn to_class(act_score: i32) -> String {
    match act_score {
        1..=15 => String::from("MA 100E"),
        16..=21 => String::from("MA 100"),
        22..=24 => String::from("MA 112"),
        25..=27 => String::from("MA 113"),
        28..=36 => String::from("MA 125"),
        _ => String::from("Invalid input"),
    }
}

fn act_score_from_user() -> i32 {
    println!("Enter your ACT score: ");
    let mut act_score = String::new();
    std::io::stdin()
        .read_line(&mut act_score)
        .expect("Failed to read line");
    let act_score: i32 = act_score.trim().parse().expect("Please type a number!");
    act_score
}
