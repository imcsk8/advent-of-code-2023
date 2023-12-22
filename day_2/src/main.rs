use std::fs;
use log::debug;

static MAX_RED: u32 = 12;
static MAX_GREEN: u32 = 13;
static MAX_BLUE: u32 = 14;


#[derive(Debug, Clone, Copy)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
    total: u32,
    valid: bool,
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    sets: Vec<Set>,
    few_set: Set,
    valid: bool,
}

impl Game {
    pub fn new(entry: String) -> Self {
        let mut ret_game = Game {
            id: 0,
            sets: Vec::new(),

            few_set: Set {
                red: 0,
                green: 0,
                blue: 0,
                valid: true,
                total: 0,
            },

            valid: true,
        };

        let mut current_set = Set {
            red: 0,
            green: 0,
            blue: 0,
            valid: true,
            total: 0,
        };

        let game: Vec<&str> = entry.split(":").collect();
        ret_game.id = game[0].replace("Game ", "").parse::<u32>().unwrap();
        let sets: Vec<&str> = game[1].split(";").collect();
        for set in &sets {
            let cube_sets: Vec<&str> = set.split(",").collect();
            for cube_set in &cube_sets {
                let s = Set::new(cube_set.to_string());
                if s.valid {
                    ret_game.sets.push(s);
                } else {
                    ret_game.valid = false;
                }

                if s.red > current_set.red {
                    current_set.red = s.red;
                }

                if s.green > current_set.green {
                    current_set.green = s.green;
                }

                if s.blue > current_set.blue {
                    current_set.blue = s.blue;
                }
            }
        }
        ret_game.few_set = current_set;
        ret_game
    }
}

impl Set {
    pub fn new(cube_set: String) -> Self {
        let mut ret_set = Set {
            red: 0,
            green: 0,
            blue: 0,
            valid: true,
            total: 0,
        };
        let cube_entry: Vec<&str> = cube_set.split(" ").collect();
        if cube_entry[2].contains("red") {
            ret_set.red = cube_entry[1].parse::<u32>().unwrap();
            if ret_set.red > MAX_RED {
               ret_set.valid = false;
            }
            ret_set.total += ret_set.red;
        }

        if cube_entry[2].contains("green") {
            ret_set.green = cube_entry[1].parse::<u32>().unwrap();
            if ret_set.green > MAX_GREEN {
                ret_set.valid = false;
            }
            ret_set.total += ret_set.green;
        }

        if cube_entry[2].contains("blue") {
            ret_set.blue = cube_entry[1].parse::<u32>().unwrap();
            if ret_set.blue > MAX_BLUE {
                ret_set.valid = false;
            }
            ret_set.total += ret_set.blue;
        }
        ret_set
    }

    pub fn power(&self) -> u64 {
        (self.red * self.green * self.blue).into()
    }
}

fn main() {
    get_games();
}

fn get_games() {
    let mut total: u32 = 0;
    let mut step2_total: u64 = 0;
    for line in fs::read_to_string("input").unwrap().lines() {
        let game = Game::new(line.to_string());
        if game.valid {
            debug!("ID: {}, VALID: {}!", game.id, game.valid);
            total += game.id;
        }
        let power = game.few_set.power();
        println!("FEWEST SET FOR GAME: {} -> {:?}: power: {}", game.id, game.few_set, power);
        step2_total += power;
    }
    println!("TOTAL STEP 1: {}", total);
    println!("TOTAL STEP 2: {}", step2_total);
}
