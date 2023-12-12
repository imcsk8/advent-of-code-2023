use regex::Regex;
use std::fs;
use std::collections::HashMap;

fn main() {
    let re = Regex::new(r"\d").unwrap();
    //let check_str = "fourlgzcrldtwoseven9xndl6qv7brtonennvlvzplrt";
    //let check_str = "fourlgzcrldtwoseven9brtonennvlvzplrt";
    let mut total = 0;
    for line in fs::read_to_string("input").unwrap().lines() {
      let converted_line = parse_text_digits(line.to_string());
      //let matches: Vec<_> = re.find_iter(line)
      let matches: Vec<_> = re.find_iter(converted_line.as_str())
        .map(|m| m.as_str()).collect();
      let strnum = format!("{}{}", matches[0], matches.last().unwrap());
      total += strnum.parse::<i32>().unwrap();
      /*println!("Entry: converted: {}, Calibration Value:  {}",
        line, strnum);*/
      println!("Entry: original: {}, converted: {}, Calibration Value:  {}",
        line, converted_line, strnum);
    }
    println!("Total Calibation Value:  {}", total);
}


fn parse_text_digits(line: String) -> String {
    let mut parsed_line = line.clone();
    let numbers = vec![
        (r"one", "1"),
        (r"two", "2"),
        (r"three", "3"),
        (r"four", "4"),
        (r"five", "5"),
        (r"six", "6"),
        (r"seven", "7"),
        (r"eight", "8"),
        (r"nine", "9"),
    ];

    let fuzzy_numbers = vec![
        (r"[o]+n[e]*", "1"),
        (r"[t]*w[o]*", "2"),
        (r"[t]*hre[e]*", "3"),
        (r"[f]*ou[r]*", "4"),
        (r"[f]*iv[e]*", "5"),
        (r"[s]+i[x]*", "6"),
        (r"[s]*eve[n]*", "7"),
        (r"[e]*igh[t]*", "8"),
        (r"[n]*in[e]*", "9"),
    ];

    let mut index = 0;
    for num in &numbers {
        // Pass 1: normal number strings
        let re = Regex::new(num.0).unwrap();
        parsed_line = re.replace_all(parsed_line.as_str(), num.1.to_string())
            .to_string();

        // Pass 2: incomplete number strings
        let re = Regex::new(fuzzy_numbers[index].0).unwrap();
        parsed_line = re.replace_all(parsed_line.as_str(), fuzzy_numbers[index].1.to_string())
            .to_string();
        println!("---- REGEX: *{}* PARSED LINE: {}", num.0, parsed_line);
        index += 1;
    }

    return parsed_line;
}
