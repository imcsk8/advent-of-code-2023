use regex::Regex;
use std::fs;

fn main() {
    let re = Regex::new(r"\d").unwrap();
    //let check_str = "fourlgzcrldtwoseven9xndl6qv7brtonennvlvzplrt";
    //let check_str = "fourlgzcrldtwoseven9brtonennvlvzplrt";
    let mut total = 0;
    for line in fs::read_to_string("input").unwrap().lines() {
      let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
      let strnum = format!("{}{}", matches[0], matches.last().unwrap());
      total += strnum.parse::<i32>().unwrap();
      println!("Entry: {}, Calibration Value:  {}", line, strnum);
    }
    println!("Total Calibation Value:  {}", total);
}


