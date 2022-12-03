// rock a
// paper b
// scissors c

// rock x = 1
// paper y = 2
// scissors z = 3

// win 6
// draw 3
// loss 0

// paper > rock > scissors > paper

use std::{fs::File, io::{BufReader, BufRead, Result}};

pub fn day_2_solution_part_one() -> Result<usize>
{
    let file = File::open(r"G:\Programming\Github\advent-of-code-2022\data\day_2_data.txt")?;
    let lines = BufReader::new(file).lines();
    let mut total_score = 0;
    for i in lines
    {
        let i = i?;
        let mut split = i.split(' ');
        let opponent = split.next().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "No value where there should be one"))?;
        let me = split.next().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "No value where there should be one"))?;
        total_score += calculate_score_part_one(me, opponent);
    }
    Ok(total_score)
}

pub fn day_2_solution_part_two() -> Result<usize>
{
    let file = File::open(r"G:\Programming\Github\advent-of-code-2022\data\day_2_data.txt")?;
    let lines = BufReader::new(file).lines();
    let mut total_score = 0;
    for i in lines
    {
        let i = i?;
        let mut split = i.split(' ');
        let opponent = split.next().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "No value where there should be one"))?;
        let me = split.next().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "No value where there should be one"))?;
        total_score += calculate_score_part_two(me, opponent);
    }
    Ok(total_score)
}

pub fn day_2_both_solutions() -> Result<(usize, usize)>
{
    let file = File::open(r"G:\Programming\Github\advent-of-code-2022\data\day_2_data.txt")?;
    let lines = BufReader::new(file).lines();
    let mut total_score_part_one = 0;
    let mut total_score_part_two = 0;
    for i in lines
    {
        let i = i?;
        let mut split = i.split(' ');
        let opponent = split.next().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "No value where there should be one"))?;
        let me = split.next().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "No value where there should be one"))?;
        total_score_part_one += calculate_score_part_one(me, opponent);
        total_score_part_two += calculate_score_part_two(me, opponent);
    }
    Ok((total_score_part_one, total_score_part_two))
}

pub fn calculate_score_part_two(player: &str, enemy: &str) -> usize
{
    // part two
    // X = lose
    // Y = draw
    // Z = Win
    match enemy
    {
        // Rock
        "A" => match player
        {
            // Lose + Scissors 
            "X" => 0 + 3,
            // Draw + Rock
            "Y" => 3 + 1,
            // Win + Paper
            "Z" => 6 + 2,
            _ => 0,
        },
        // Paper
        "B" => match player
        {
            // Lose + Rock
            "X" => 0 + 1,
            // Draw + Paper
            "Y" => 3 + 2,
            // Win + Scissors
            "Z" => 6 + 3,
            _ => 0,
        },
        // Scissors
        "C" => match player
        {
            // Lose + Paper
            "X" => 0 + 2,
            // Draw + Scissors
            "Y" => 3 + 3,
            // Win + Rock
            "Z" => 6 + 1,
            _ => 0,
        },
        _ => 0,
    }
}

fn calculate_score_part_one(player: &str, enemy: &str) -> usize
{
    match player
    {
        // Rock
        "X" => 1 + match enemy
        {
            // Draw
            "A" => 3,
            // Lose
            "B" => 0,
            // Win
            "C" => 6,
            _ => 0,
        },
        // Paper
        "Y" => 2 + match enemy
        {
            // Win
            "A" => 6,
            // Draw
            "B" => 3,
            // Lose
            "C" => 0,
            _ => 0,
        },
        // Scissors
        "Z" => 3 + match enemy
        {
            // Lose
            "A" => 0,
            // Win
            "B" => 6,
            // Draw
            "C" => 3,
            _ => 0,
        },
        _ => 0,
    }
}