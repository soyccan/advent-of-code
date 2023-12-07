use std::error::Error;
use std::io::stdin;
use std::io::BufRead;

static NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    match run(&mut stdin().lock(), true) {
        Ok(r) => println!("{}", r),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run<R>(input_stream: &mut R, parse_word: bool) -> Result<u32, Box<dyn Error>>
where
    R: BufRead,
{
    let mut input = String::new();
    let mut sum = 0;

    while input_stream.read_line(&mut input)? > 0 {
        let mut first_digit: Option<(usize, u32)> = None;
        let mut last_digit: Option<(usize, u32)> = None;

        for (i, c) in input.chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some((i, d));
                }
                last_digit = Some((i, d));
            }
        }
        if parse_word {
            for (digit, &word) in NUMBER_WORDS.iter().enumerate() {
                let digit = (digit + 1) as u32;
                if let Some(index) = input.find(word) {
                    if first_digit.map_or(true, |(i, _)| index < i) {
                        first_digit = Some((index, digit));
                    }
                }
                if let Some(index) = input.rfind(word) {
                    if last_digit.map_or(true, |(i, _)| index > i) {
                        last_digit = Some((index, digit));
                    }
                }
            }
        }
        sum += first_digit.unwrap().1 * 10 + last_digit.unwrap().1;

        input.clear();
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::error::Error;
    use std::fs::File;
    use std::io::SeekFrom;
    use std::io::{BufReader, Seek};

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        let mut stream = BufReader::new(File::open("testdata/day1.in")?);
        assert_eq!(run(&mut stream, false)?, 54951);
        stream.seek(SeekFrom::Start(0))?;
        assert_eq!(run(&mut stream, true)?, 55218);
        Ok(())
    }
}
