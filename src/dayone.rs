pub mod dayone {
    use crate::search::search::{get_first_number_from_text, get_last_number_from_text};
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    };
    
    pub fn dayone(path: &str) {
        let inputs = read_input(&path).unwrap();
        let sum = get_sum_from_input(&inputs);
        println!("The total sum is {}", sum);
    }

    fn get_sum_from_input(inputs: &Vec<String>) -> i32 {
        let mut sum = 0;
        for line in inputs {
            sum += get_number_from_line(&line);
        }
        assert!(sum == 53894);
        sum
    }

    fn get_number_from_line(line: &str) -> i32 {
        let first = get_first_number(line);
        let last = get_last_number(line);
        let number_found = (first * 10) + last;

        println!("From line {}, number found {}", line, number_found);

        return number_found;
    }

    fn get_last_number(line: &str) -> i32 {
        let mut last_digit_index = -1;
        let mut last_digit_value = -1;

        for (index, char) in line.char_indices().rev() {
            if char.is_digit(10) {
                last_digit_value = get_number_from_char(char);
                last_digit_index = index as i32;
                break;
            }
        }

        let last_number_result = get_last_number_from_text(&line);
        if last_number_result.is_none() {
            return last_digit_value;
        }

        let last_number_result_unwrapped = last_number_result.unwrap();
        let last_spelled_index = last_number_result_unwrapped.1;

        if last_digit_index > last_spelled_index {
            return last_digit_value;
        }

        return last_number_result_unwrapped.0;
    }

    fn get_first_number(line: &str) -> i32 {
        let mut first_digit_index = i32::MAX;
        let mut first_digit_value = -1;

        for (index, char) in line.char_indices() {
            if char.is_digit(10) {
                first_digit_value = get_number_from_char(char);
                first_digit_index = index as i32;
                break;
            }
        }
        let number_result = get_first_number_from_text(&line);

        if number_result.is_none() {
            return first_digit_value;
        }

        let number_result_unwrapped = number_result.unwrap();
        let first_text_index = number_result_unwrapped.1;

        if first_digit_index < first_text_index {
            return first_digit_value;
        }

        return number_result_unwrapped.0;
    }

    fn get_number_from_char(c: char) -> i32 {
        return c.to_digit(10).unwrap() as i32;
    }

    fn read_input(path: &str) -> Result<Vec<String>, std::io::Error> {
        let path = Path::new(path);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();

        for line in reader.lines() {
            let line = line?;
            lines.push(line);
        }

        Ok(lines)
    }
}
