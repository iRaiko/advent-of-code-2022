use std::io::{Result, ErrorKind};

fn get_most_calories(elves: &Vec<Vec<u32>>) -> u32
{
    let mut most_calories = 0;
    for elve in elves
    {
        let mut current_elve_calories = 0;
        for calorie in elve
        {
            current_elve_calories += calorie;
        }
        if current_elve_calories > most_calories
        {
            most_calories = current_elve_calories;
        }
    }
    most_calories
}

fn clean_data(input: String) -> Result<Vec<Vec<u32>>>
{
    let mut chars = input.chars().peekable();
    let mut elves = Vec::new();
    while let Some(c) = chars.peek()
    {
        if *c == '\r'
        {
            chars.next();
            chars.next();
            break;
        }
        let mut elve_calories = Vec::new();
        while let Some(c) = chars.peek()
        {
            // If there is a linebreak here it means it is the end of the current elve's inventory and we go to the next elve
            if *c == '\r'
            {
                chars.next();
                chars.next();
                break;
            }
            
            // we can unwrap because chars.peek() confirms there is *some* data
            let mut current: u32 = chars.next().unwrap().to_digit(10).ok_or(
                std::io::Error::new(ErrorKind::Other, "Expected a number"))?;
            while let Some(c) = chars.next()
            {
                // If there is a linebreak here it means it is the end of the current calorie count and we go to the next one
                if c == '\r'
                {
                    chars.next();
                    break;
                }
                current = (current * 10) + c.to_digit(10).ok_or(
                    std::io::Error::new(ErrorKind::Other, "Expected a number"))?;
            }
            elve_calories.push(current);
        }

        elves.push(elve_calories);
    }
    Ok(elves)
}

pub fn day_1_solution() -> Result<u32>
{
    let raw_input_data = std::fs::read_to_string(r"G:\Programming\Github\advent-of-code-2022\data\day_1_data.txt")?;
    let elves = clean_data(raw_input_data)?;
    Ok(get_most_calories(&elves))
}