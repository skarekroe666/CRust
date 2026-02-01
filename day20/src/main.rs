fn main() {
    let my_mood = "SAD";

    let emoji = match my_mood.to_lowercase().as_str() {
        "sad" => "🥲",
        s if s.contains("crazy") => "🤪",
        _ => "😐",
    };

    println!("{my_mood}");
    println!("{emoji}");
}
