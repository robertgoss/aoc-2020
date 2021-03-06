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
mod expressions;
mod matching;
mod tileset;
mod allergens;
mod crabs;
mod cups;
mod floor;
mod door;

extern crate pest;
#[macro_use]
extern crate pest_derive;

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
    use super::cups as cups;
    use super::floor as floor;

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
    fn challenge_35() {
        let data = io::input_as_expressions(18);
        let num : i64 = data.iter().map(
            |expr| expr.compute()
        ).sum();
        println!("{}", num);
    }
    fn challenge_36() {
        let data = io::input_as_expressions(18);
        let num : i64 = data.iter().map(
            |expr| expr.compute_precedent()
        ).sum();
        println!("{}", num);
    }
    fn challenge_37() {
        let (rules, data) = io::input_as_matching(19);
        let num : usize = data.iter().filter(
            |line| rules.is_match(&line)
        ).count();
        println!("{}", num);
    }
    fn challenge_38() {
        let (mut rules, data) = io::input_as_matching(19);
        rules.add_new_rules();
        let num : usize = data.iter().filter(
            |line| rules.is_match(&line)
        ).count();
        println!("{}", num);
    }
    fn challenge_39() {
        let tileset = io::input_as_tileset(20);
        let solution = tileset.solve(12);
        let num : u64 = solution.corner_tiles().iter().product();
        println!("{}", num);
    }
    fn challenge_40() {
        let tileset = io::input_as_tileset(20);
        let solution = tileset.solve(12);
        let picture = solution.picture();
        let total = picture.total();
        let monster_num = picture.search_monster().len();
        let num = total - monster_num;
        println!("{}", num);
    }
    fn challenge_41() {
        let menu = io::input_as_menu(21);
        let num = menu.count_ingredients_no_allergens();
        println!("{}", num);
    }
    fn challenge_42() {
        let menu = io::input_as_menu(21);
        let ingredients = menu.ordered_ingredients_allergens();
        println!("{}", ingredients.join(","));
    }
    fn challenge_43() {
        let mut data = io::input_as_game(22);
        data.play();
        println!("{}",data.score());
    }
    fn challenge_44() {
        let mut data = io::input_as_game(22);
        data.play_recursive();
        println!("{}",data.score());
    }
    fn challenge_45() {
        let mut data = cups::Cups::from_cycle_ints(
            vec!(3,6,2,9,8,1,7,5,4)
        );
        data.simulate(100);
        println!("{}",data.labels(1));
    }
    fn challenge_46() {
        let init = vec!(3,6,2,9,8,1,7,5,4);
        let vec :Vec<usize> = init.into_iter().chain(10..=1000000).collect();
        let mut data = cups::Cups::from_cycle_ints(vec);
        data.simulate(10000000);
        let num = data.offset_labels(1, 1) * data.offset_labels(1, 2);
        println!("{}",num);
    }
    fn challenge_47() {
        let data = io::input_as_paths(24);
        let mut floor = floor::Floor::new();
        floor.apply_paths(&data);
        let num = floor.len();
        println!("{}",num);
    }
    fn challenge_48() {
        let data = io::input_as_paths(24);
        let mut floor = floor::Floor::new();
        floor.apply_paths(&data);
        floor.simulate(100);
        let num = floor.len();
        println!("{}",num);
    }
    fn challenge_49() {
        let data = io::input_as_handshake(25);
        let num = data.encryption_key();
        println!("{}",num);
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
            35 => challenge_35(),
            36 => challenge_36(),
            37 => challenge_37(),
            38 => challenge_38(),
            39 => challenge_39(),
            40 => challenge_40(),
            41 => challenge_41(),
            42 => challenge_42(),
            43 => challenge_43(),
            44 => challenge_44(),
            45 => challenge_45(),
            46 => challenge_46(),
            47 => challenge_47(),
            48 => challenge_48(),
            49 => challenge_49(),
            _ => () 
        }
    }
}



fn main() {
    challenge::challenge(49);
}