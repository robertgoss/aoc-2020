
mod io {
    use std::io::Read;
    use std::io::BufRead;
    use std::fs::File;
    use std::io::BufReader;

    fn input_as_list(day: i8) -> Vec<i64> {
        let filename = format!("data\\day-{}.txt", day);
        let file = File::open(filename).expect("Issue opening file");
        let reader = BufReader::new(&file);
        reader.lines().map(
            |s| s.expect("Read failure").parse::<i64>().unwrap()
        ).collect()
    }
}

mod challenge {
    use super::io as io;

    fn challenge_1() {
        let data = io::input_as_list(1);
        println!("{}", &data);
    }

    pub fn challenge(num : u8) {
        match num {
            1 => challenge_1(),
            _ => () 
        }
    }
}



fn main() {
    challenge::challenge(1);
}