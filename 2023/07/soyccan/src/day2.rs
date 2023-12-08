use std::error::Error;
use std::io::stdin;
use std::io::BufRead;

fn main() {
    match run(&mut stdin().lock()) {
        Ok(r) => println!("{} {}", r.0, r.1),
        Err(e) => eprintln!("Oops: {}", e),
    }
}

fn run<R>(input_stream: &mut R) -> Result<(i32, i32), Box<dyn Error>>
where
    R: BufRead,
{
    let mut input = String::new();
    let mut sum_possible_games = 0;
    let mut sum_power = 0;
    while input_stream.read_line(&mut input)? > 0 {
        let (head, tail) = match input.split_once(':') {
            Some((head, tail)) => (head, tail),
            None => return Err("Invalid input".into()),
        };
        let game_id: i32 = head.split_at("Game ".len()).1.parse()?;
        let (possible, power) = game_possible(tail.trim())?;
        if possible {
            sum_possible_games += game_id;
        }
        sum_power += power;
        input.clear();
    }
    Ok((sum_possible_games, sum_power))
}

fn game_possible(sets: &str) -> Result<(bool, i32), Box<dyn Error>> {
    let mut total_red = 0;
    let mut total_green = 0;
    let mut total_blue = 0;
    let mut possible = true;
    for set in sets.split(';') {
        let (red, green, blue) = parse_cubes(set)?;
        if red > 12 || green > 13 || blue > 14 {
            possible = false;
        }
        total_red = total_red.max(red);
        total_green = total_green.max(green);
        total_blue = total_blue.max(blue);
    }
    let power = total_red * total_green * total_blue;
    Ok((possible, power))
}

fn parse_cubes(set: &str) -> Result<(i32, i32, i32), Box<dyn Error>> {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for cubes in set.split(',') {
        let (count, color): (i32, &str) = match cubes.trim().split_once(' ') {
            Some((count, color)) => (count.parse()?, color.trim()),
            None => return Err("Invalid cubes".into()),
        };
        match color {
            "red" => red += count,
            "green" => green += count,
            "blue" => blue += count,
            _ => (),
        }
    }
    Ok((red, green, blue))
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Cursor;

    #[test]
    fn test0() -> Result<(), Box<dyn Error>> {
        let mut cursor = Cursor::new(
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#,
        );
        assert_eq!(run(&mut cursor)?, (8, 2286));
        Ok(())
    }

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        let mut stream = BufReader::new(File::open("testdata/day2.in")?);
        assert_eq!(run(&mut stream)?, (2406, 78375));
        Ok(())
    }
}
