pub struct Handshake {
    door_pub : u64,
    card_pub : u64
}

fn transform(loopnum : u64, val : u64) -> u64 {
    let mut transformed = val;
    for _ in 0..loopnum {
        transformed = (transformed * val) % 20201227;
    }
    transformed
}

impl Handshake {
    pub fn from_strings(door : &str, card : &str) -> Handshake {
        Handshake {
            door_pub : door.parse::<u64>().unwrap(),
            card_pub : card.parse::<u64>().unwrap()
        }
    }

    fn door_loop(self : &Self) -> u64 {
        let mut public = 1;
        for i in 0.. {
            if i % 4048 == 0 {
                println!("Loop try {}", i);
            }
            public = (public * 7) % 20201227;
            if public == self.door_pub {
                println!("Door loop: {}", i);
                return i;
            }
        }
        unreachable!();
    }

    pub fn encryption_key(self : &Self) -> u64 {
        transform(self.door_loop(), self.card_pub)
    }
}