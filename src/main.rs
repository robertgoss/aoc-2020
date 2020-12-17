#![feature(str_split_once)]
#![feature(iter_map_while)]
#![feature(unsigned_abs)]

mod memory_game;
mod expenses;
mod passwords;
mod map;
mod passport;
mod ticket;

mod customs;
mod baggage;
mod cpu;
mod cipher;
mod adaptors;
mod seating;
mod directions;
mod buses;
mod docking;
mod ticket_scanning;
mod conway;

mod io;

mod challenge {
    use super::io as io;
    use super::expenses as expenses;
    use super::passwords as passwords;
    use super::seating as seating;
    use super::directions as directions;
    use super::docking as docking;
    use super::memory_game as memory_game;
    use super::conway as conway;

    fn challenge_1() {
        let data = io::input_as_list(1);
        let (a, b) = expenses::find_2_summands(&data, 2020).unwrap();
        println!("{} {} {}", a , b, a * b);
    }

    fn challenge_2() {
        let data = io::input_as_list(1);
        let (a, b, c) = expenses::find_3_summands(&data, 2020).unwrap();
        println!("{} {} {} {}", a, b, c, a * b * c);
    }

    fn challenge_3() {
        let data = io::input_as_password_database(2);
        let num = data.count_valid(passwords::RuleSet::Sled);
        println!("{}", num);
    }
    fn challenge_4() {
        let data = io::input_as_password_database(2);
        let num = data.count_valid(passwords::RuleSet::Toboggan);
        println!("{}", num);
    }
    fn challenge_5() {
        let data = io::input_as_map(3);
        let num = data.count_trees_path(1, 3);
        println!("{}", num);
    }
    fn challenge_6() {
        let data = io::input_as_map(3);
        let a = data.count_trees_path(1, 1);
        let b = data.count_trees_path(1, 3);
        let c = data.count_trees_path(1, 5);
        let d = data.count_trees_path(1, 7);
        let e = data.count_trees_path(2, 1);
        println!("{} {} {} {} {} {}", a, b, c, d, e, a*b*c*d*e);
    }
    fn challenge_7() {
        let data = io::input_as_passports(4);
        let num = data.iter().filter(
            |passport| passport.required_fields_exist()
        ).count();
        println!("{}", num);
    }
    fn challenge_8() {
        let data = io::input_as_passports(4);
        let num = data.iter().filter(
            |passport| passport.required_fields_valid()
        ).count();
        println!("{}", num);
    }
    fn challenge_9() {
        let data = io::input_as_plane(5);
        let num = data.max();
        println!("{}", num);
    }
    fn challenge_10() {
        let data = io::input_as_plane(5);
        let num = data.find_missing()[0];
        println!("{}", num);
    }
    fn challenge_11() {
        let data = io::input_as_forms(6);
        let num : usize = data.iter().map(
            |form| form.num_any()
        ).sum();
        println!("{}", num);
    }
    fn challenge_12() {
        let data = io::input_as_forms(6);
        let num : usize = data.iter().map(
            |form| form.num_all()
        ).sum();
        println!("{}", num);
    }
    fn challenge_13() {
        let data = io::input_as_rules(7);
        let num = data.num_dependencies("shiny gold").unwrap();
        println!("{}", num);
    }
    fn challenge_14() {
        let data = io::input_as_rules(7);
        let num = data.full_num_contained("shiny gold").unwrap();
        println!("{}", num);
    }
    fn challenge_15() {
        let data = io::input_as_program(8);
        let num = data.run();
        println!("{}", num);
    }
    fn challenge_16() {
        let data = io::input_as_program(8);
        let num = data.fix().unwrap();
        println!("{}", num);
    }
    fn challenge_17() {
        let data = io::input_as_ciphertext(9);
        let num = data.first_prop(25).unwrap();
        println!("{}", num);
    }
    fn challenge_18() {
        let data = io::input_as_ciphertext(9);
        let num = data.weakness(25).unwrap();
        println!("{}", num);
    }
    fn challenge_19() {
        let data = io::input_as_adaptors(10);
        let num = data.joltage_differences();
        println!("{}", num);
    }
    fn challenge_20() {
        let data = io::input_as_adaptors(10);
        let num = data.number_arrangements();
        println!("{}", num);
    }
    fn challenge_21() {
        let mut data = io::input_as_seating(11);
        data.simulate(&seating::SeatingRules::Adjacent);
        let num = data.number_occupied();
        println!("{}", num);
    }
    fn challenge_22() {
        let mut data = io::input_as_seating(11);
        data.simulate(&seating::SeatingRules::Visible);
        let num = data.number_occupied();
        println!("{}", num);
    }
    fn challenge_23() {
        let data = io::input_as_actions(12);
        let mut ship = directions::Ship::new();
        ship.simulate(data.into_iter());
        let num = ship.distance();
        println!("{}", num);
    }
    fn challenge_24() {
        let data = io::input_as_actions(12);
        let mut ship = directions::Ship::new();
        ship.simulate_waypoint(data.into_iter());
        let num = ship.distance();
        println!("{}", num);
    }
    fn challenge_25() {
        let data = io::input_as_timetable(13);
        let (offset, bus) = data.first_bus().unwrap();
        let num = offset * bus.id().unwrap();
        println!("{}", num);
    }
    fn challenge_26() {
        let data = io::input_as_timetable(13);
        let num = data.first_congunction();
        println!("{}", num);
    }
    fn challenge_27() {
        let data = io::input_as_docking_program(14);
        let mut cpu = docking::Computer::new();
        cpu.run(&data);
        let num = cpu.sum_variables();
        println!("{}", num);
    }
    fn challenge_28() {
        let data = io::input_as_docking_program(14);
        let mut cpu = docking::Computer::new();
        cpu.run_decode(&data);
        let num = cpu.sum_variables();
        println!("{}", num);
    }
    fn challenge_29() {
        let mut data = memory_game::Game::new(
            io::input_as_comma_list(15)
        );
        let num = data.nth(2020-1).unwrap();
        println!("{}", num);
    }
    fn challenge_30() {
        let mut data = memory_game::Game::new(
            io::input_as_comma_list(15)
        );
        let num = data.nth(30000000-1).unwrap();
        println!("{}", num);
    }
    fn challenge_31() {
        let data = io::input_as_scanning_results(16);
        let num = data.scanning_error_rate();
        println!("{}", num);
    }
    fn challenge_32() {
        let mut data = io::input_as_scanning_results(16);
        data.discard_invalid();
        let num : u64 = data.departures().iter().map(
            |name| data.own_field(name).unwrap()
        ).product();
        println!("{}", num);
    }
    fn challenge_33() {
        let mut data = io::input_as_conway::<conway::Position3D>(17);
        data.simulate_n(6);
        let num = data.num_cubes();
        println!("{}", num);
    }
    fn challenge_34() {
        let mut data = io::input_as_conway::<conway::Position4D>(17);
        data.simulate_n(6);
        let num = data.num_cubes();
        println!("{}", num);
    }

    pub fn challenge(num : u8) {
        match num {
            1 => challenge_1(),
            2 => challenge_2(),
            3 => challenge_3(),
            4 => challenge_4(),
            5 => challenge_5(),
            6 => challenge_6(),
            7 => challenge_7(),
            8 => challenge_8(),
            9 => challenge_9(),
            10 => challenge_10(),
            11 => challenge_11(),
            12 => challenge_12(),
            13 => challenge_13(),
            14 => challenge_14(),
            15 => challenge_15(),
            16 => challenge_16(),
            17 => challenge_17(),
            18 => challenge_18(),
            19 => challenge_19(),
            20 => challenge_20(),
            21 => challenge_21(),
            22 => challenge_22(),
            23 => challenge_23(),
            24 => challenge_24(),
            25 => challenge_25(),
            26 => challenge_26(),
            27 => challenge_27(),
            28 => challenge_28(),
            29 => challenge_29(),
            30 => challenge_30(),
            31 => challenge_31(),
            32 => challenge_32(),
            33 => challenge_33(),
            34 => challenge_34(),
            _ => () 
        }
    }
}



fn main() {
    challenge::challenge(34);
}