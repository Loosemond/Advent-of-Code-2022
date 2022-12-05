use std::fs;

// A and X for Rock 1
// B and Y for Paper 2
// C and Z for Scissors 3
// Player Wins C X, A Y , B Z
// Player Draw if the same,
// Player Loses B X, C Y, A Z

// 1 - Check if is the same
// 2 - Check if it is a winning condition

fn score_player_p1(play: &str) -> u32 {
    match play {
        "C X" => 6 + 1,
        "A Y" => 6 + 2,
        "B Z" => 6 + 3,
        "A X" => 3 + 1,
        "B Y" => 3 + 2,
        "C Z" => 3 + 3,
        "B X" => 1,
        "C Y" => 2,
        "A Z" => 3,
        &_ => 0,
    }
}

// X loose
// Y draw
// Z Win
// A and X for Rock 1
// B and Y for Paper 2
// C and Z for Scissors 3
fn score_player_p2(play: &str) -> u32 {
    match play {
        "C Z" => 6 + 1,
        "A Z" => 6 + 2,
        "B Z" => 6 + 3,
        "A Y" => 3 + 1,
        "B Y" => 3 + 2,
        "C Y" => 3 + 3,
        "B X" => 0 + 1,
        "C X" => 0 + 2,
        "A X" => 0 + 3,
        &_ => 0,
    }
}

fn main() {
    println!("Day 2");
    let file_path = "./input-file";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut total: u32 = contents
        .rsplit("\n")
        .map(|play| score_player_p1(play))
        .sum();
    println!("Part 1 Final score: {}", total);
    total = contents
        .rsplit("\n")
        .map(|play| score_player_p2(play))
        .sum();
    println!("Part 2 Final score: {}", total);
}
