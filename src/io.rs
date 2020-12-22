use std::io::BufRead;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::hash::Hash;

use super::passwords as passwords;
use super::map as map;
use super::passport as passport;
use super::ticket as ticket;
use super::customs as customs;
use super::baggage as baggage;
use super::cpu as cpu;
use super::cipher as cipher;
use super::adaptors as adaptors;
use super::seating as seating;
use super::directions as directions;
use super::buses as buses;
use super::docking as docking;
use super::ticket_scanning as ticket_scanning;
use super::conway as conway;
use super::expressions as expressions;
use super::matching as matching;
use super::tileset as tileset;
use super::allergens as allergens;

pub fn input_as_list(day: i8) -> Vec<i64> {
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    reader.lines().map(
        |s| s.expect("Read failure").parse::<i64>().unwrap()
    ).collect()
}
pub fn input_as_comma_list(day: i8) -> Vec<u64> {
    let filename = format!("data/day-{}.txt", day);
    let data = fs::read_to_string(filename).expect("Issue reading file");
    data.split(",").map(
        |s| s.parse::<u64>().unwrap()
    ).collect()
}
pub fn input_as_password_database(day: i8) -> passwords::Database {
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    let mut database = passwords::Database::new();
    for line in reader.lines() {
        database.add_line(&line.expect("Read failure"));
    }
    database
}
pub fn input_as_map(day: i8) -> map::Map {
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    let mut map = map::Map::new();
    for line in reader.lines() {
        map.add_line(&line.expect("Read failure"));
    }
    map
}
pub fn input_as_passports(day : i8) -> Vec<passport::Passport> {
    let filename = format!("data/day-{}.txt", day);
    let data = fs::read_to_string(filename).expect("Issue reading file");
    data.split("\n\n").map(
        |chunk| passport::Passport::new(chunk)
    ).collect()
}
pub fn input_as_plane(day : i8) -> ticket::Plane {
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    let seats : Vec<ticket::Seat> = reader.lines().map(
        |line| ticket::Seat::from_string(&line.expect("ReadFailure")).unwrap()
    ).collect();
    let mut plane = ticket::Plane::new();
    for seat in seats.iter() {
        plane.add_seat(seat);
    }
    plane
}
pub fn input_as_forms(day : i8) -> Vec<customs::Form> {
    let filename = format!("data/day-{}.txt", day);
    let data = fs::read_to_string(filename).expect("Issue reading file");
    data.split("\n\n").map(
        |chunk| customs::Form::new(chunk)
    ).collect()
}
pub fn input_as_rules(day : i8) -> baggage::Rules {
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    let mut rules = baggage::Rules::new();
    for line in reader.lines() {
        rules.add_line(&line.expect("Read failure"));
    }
    rules
}
pub fn input_as_program(day : i8) -> cpu::Program {
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    cpu::Program::from_lines(
        reader.lines().map(|line| line.expect("Read failure"))
    )
}
pub fn input_as_ciphertext(day : i8) -> cipher::CipherText {
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    cipher::CipherText::from_lines(
        reader.lines().map(|line| line.expect("Read failure"))
    )
}
pub fn input_as_adaptors(day : i8) -> adaptors::Adaptors {
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    adaptors::Adaptors::from_lines(
        reader.lines().map(|line| line.expect("Read failure"))
    )
}
pub fn input_as_seating(day : i8) -> seating::Seating {
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    seating::Seating::from_lines(
        reader.lines().map(|line| line.expect("Read failure"))
    )
}
pub fn input_as_actions(day : i8) -> Vec<directions::Action> {
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    reader.lines().filter_map(
        |line| directions::Action::from_string(&line.expect("Read failure"))
    ).collect()
}
pub fn input_as_timetable(day : i8) -> buses::Timetable {
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    let lines : Vec<String> = reader.lines().map(
        |line| line.expect("Read failure")
    ).collect();
    buses::Timetable::from_lines(
        &lines[0],
        &lines[1]
    )
}
pub fn input_as_docking_program(day : i8) -> docking::Program {
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    docking::Program::from_lines(
        reader.lines().map(|line| line.expect("Read failure"))
    )
}
pub fn input_as_scanning_results(day : i8) -> ticket_scanning::ScanningResult {
    let filename = format!("data/day-{}.txt", day);
    let data = fs::read_to_string(filename).expect("Issue reading file");
    let chunks : Vec<&str> = data.split("\n\n").collect();
    ticket_scanning::ScanningResult::from_lines(
        chunks[0].lines(),
        chunks[1].lines().nth(1).unwrap(),
        chunks[2].lines().skip(1)
    )
}
pub fn input_as_conway<P>(day : i8) -> conway::Conway<P>
  where P : conway::Position + Eq + Hash + Copy
{
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    conway::Conway::from_lines(
        reader.lines().map(|line| line.expect("Read failure"))
    )
}
pub fn input_as_expressions(day : i8) -> Vec<expressions::Expression> {
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    reader.lines().map(|
        line| expressions::Expression::from_string(&line.expect("Read failure")).unwrap()
    ).collect()
}
pub fn input_as_matching(day : i8) -> (matching::RuleSet, Vec<String>) {
    let lines_filename = format!("data/day-{}.txt", day);
    let lines_file = File::open(lines_filename).expect("Issue opening file");
    let lines_reader = BufReader::new(&lines_file);
    let lines : Vec<String> = lines_reader.lines().map(
        |line| line.expect("Read failure")
    ).collect();
    let rules_filename = format!("data/day-{}.rules", day);
    let rules_file = File::open(rules_filename).expect("Issue opening file");
    let rules_reader = BufReader::new(&rules_file);
    let ruleset = matching::RuleSet::from_lines(
        rules_reader.lines().map(|line| line.expect("Read failure"))
    );
    (ruleset, lines)
}
pub fn input_as_tileset(day : i8) -> tileset::TileSet {
    let filename = format!("data/day-{}.txt", day);
    let data = fs::read_to_string(filename).expect("Issue reading file");
    tileset::TileSet::from_string(&data)
}
pub fn input_as_menu(day : i8) -> allergens::Menu
{
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    allergens::Menu::from_lines(
        reader.lines().map(|line| line.expect("Read failure"))
    )
}