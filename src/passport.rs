use std::iter::FromIterator;
use std::collections::HashMap;

static REQUIRED_FIELD_NAMES : [&str; 7] = ["byr","iyr","eyr","hgt","hcl","ecl","pid"];
static VALID_EYE_COLOURS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn is_hex(string : &str) -> bool {
    string.chars().all(
        |c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c)
    )

}
pub struct Passport {
    elements : HashMap<String, String>
}

impl Passport {
    pub fn new(data : &str) -> Passport {
        let data = data.to_string();
        let elements = HashMap::from_iter(
            data.split_whitespace().filter_map(
                |part| part.split_once(':')
            ).map(
                |(key, val)| (key.to_string(), val.to_string())
            )
        );
        Passport { elements : elements }
    }
    pub fn required_fields_exist(self : &Self) -> bool {
        REQUIRED_FIELD_NAMES.iter().all(
            |field_name| self.elements.contains_key(&field_name.to_string())
        )
    }
    pub fn required_fields_valid(self : &Self) -> bool {
        self.byr_valid() &&
        self.iyr_valid() &&
        self.eyr_valid() &&
        self.hgt_valid() &&
        self.hcl_valid() &&
        self.ecl_valid() &&
        self.pid_valid()
    }
    pub fn byr_valid(self : &Self) -> bool {
        self.elements.get("byr").and_then(
            |byr| byr.parse::<usize>().ok()
        ).map(
            |byr| 1920 <= byr && byr <= 2002
        ).unwrap_or(false)
    }
    fn iyr_valid(self : &Self) -> bool {
        self.elements.get("iyr").and_then(
            |byr| byr.parse::<usize>().ok()
        ).map(
            |byr| 2010 <= byr && byr <= 2020
        ).unwrap_or(false)
    }
    fn eyr_valid(self : &Self) -> bool {
        self.elements.get("eyr").and_then(
            |byr| byr.parse::<usize>().ok()
        ).map(
            |byr| 2020 <= byr && byr <= 2030
        ).unwrap_or(false)
    }
    fn hgt_valid(self : &Self) -> bool {
        self.hgt_valid_cm() || self.hgt_valid_in()
    }
    fn hgt_valid_cm(self : &Self) -> bool {
        self.elements.get("hgt").and_then(
            |hgt| hgt.strip_suffix("cm")
        ).and_then(
            |hgt| hgt.parse::<usize>().ok()
        ).map(
            |hgt| (150..=193).contains(&hgt)
        ).unwrap_or(false)
    }
    fn hgt_valid_in(self : &Self) -> bool {
        self.elements.get("hgt").and_then(
            |hgt| hgt.strip_suffix("in")
        ).and_then(
            |hgt| hgt.parse::<usize>().ok()
        ).map(
            |hgt| (59..=76).contains(&hgt)
        ).unwrap_or(false)
    }
    fn hcl_valid(self : &Self) -> bool {
        self.elements.get("hcl").and_then(
            |hcl| hcl.strip_prefix('#')
        ).map(
            |hex| is_hex(hex)
        ).unwrap_or(false)
    }
    fn ecl_valid(self : &Self) -> bool {
        self.elements.get("ecl").map(
            |ecl| VALID_EYE_COLOURS.iter().filter(|&col| col == ecl).count() == 1
        ).unwrap_or(false)
    }
    fn pid_valid(self : &Self) -> bool {
        self.elements.get("pid").map(
            |pid| pid.parse::<usize>().is_ok() && pid.len() == 9
        ).unwrap_or(false)
    }
}