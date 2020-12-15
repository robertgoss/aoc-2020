#[derive(Copy, Clone, Debug)]
enum Cardinal {
    North,
    East,
    South,
    West
}
impl Cardinal {
    fn next(self : &Self) -> Self {
        match self {
            Cardinal::North => Cardinal::East,
            Cardinal::East => Cardinal::South,
            Cardinal::South => Cardinal::West,
            Cardinal::West => Cardinal::North
        }
    } 
    fn prev(self : &Self) -> Self {
        match self {
            Cardinal::North => Cardinal::West,
            Cardinal::East => Cardinal::North,
            Cardinal::South => Cardinal::East,
            Cardinal::West => Cardinal::South
        }
    } 
    pub fn rotate(self : &Self, amount : i32) -> Self {
        if amount == 0 {
            *self
        } else if amount > 0 {
            self.next().rotate(amount-1)
        } else {
            self.prev().rotate(amount+1)
        }
    }
}
#[derive(Copy, Clone)]
enum ActionType {
    Direction(Cardinal),
    Left,
    Right,
    Forward
}
impl ActionType {
    pub fn from_char(ch : char) -> Option<ActionType> {
        match ch {
            'L' => Some(ActionType::Left),
            'R' => Some(ActionType::Right),
            'F' => Some(ActionType::Forward),
            'N' => Some(ActionType::Direction(Cardinal::North)),
            'S' => Some(ActionType::Direction(Cardinal::South)),
            'E' => Some(ActionType::Direction(Cardinal::East)),
            'W' => Some(ActionType::Direction(Cardinal::West)),
            _ => None
        }
    }
}
pub struct Action {
    atype : ActionType,
    val : i32
}
impl Action {
    pub fn from_string(string : &str) -> Option<Action> {
        ActionType::from_char(
            string.chars().next().unwrap()
        ).and_then(
            |atype| string[1..].parse::<i32>().ok().map(
                |val| Action{ atype : atype, val : val}
            )
        )
    }
}
pub struct Ship {
    position : (i32, i32),
    waypoint : (i32, i32),
    direction : Cardinal
}
impl Ship {
    pub fn new() -> Ship {
        Ship { position : (0,0), direction : Cardinal::East, waypoint : (10, 1) }
    }
    pub fn simulate<I>(self : &mut Self, actions : I)
      where I : Iterator<Item = Action>
    {
        for action in actions {
            self.simulate_once(&action);
        }
    }
    pub fn simulate_waypoint<I>(self : &mut Self, actions : I)
      where I : Iterator<Item = Action>
    {
        for action in actions {
            self.simulate_waypoint_once(&action);
        }
    }
    fn simulate_once(self : &mut Self, action : &Action)
    {
        match action.atype {
            ActionType::Direction(dir) => self.move_direction(dir, action.val),
            ActionType::Forward => self.move_direction(self.direction, action.val),
            ActionType::Left => self.turn(-action.val),
            ActionType::Right => self.turn(action.val)
        } 
    }
    fn simulate_waypoint_once(self : &mut Self, action : &Action)
    {
        match action.atype {
            ActionType::Direction(dir) => self.move_waypoint(dir, action.val),
            ActionType::Forward => self.move_to_waypoint(action.val),
            ActionType::Left => self.turn_waypoint(-action.val / 90),
            ActionType::Right => self.turn_waypoint(action.val / 90)
        } 
    }
    fn turn(self : &mut Self, degrees : i32) {
        let amount : i32 = degrees / 90;
        self.direction = self.direction.rotate(amount);
    }
    fn move_direction(self : &mut Self, direction : Cardinal, distance : i32) {
        match direction {
            Cardinal::North => self.position.1 += distance,
            Cardinal::South => self.position.1 -= distance,
            Cardinal::East => self.position.0 += distance,
            Cardinal::West => self.position.0 -= distance
        };
    }
    fn move_waypoint(self : &mut Self, direction : Cardinal, distance : i32) {
        match direction {
            Cardinal::North => self.waypoint.1 += distance,
            Cardinal::South => self.waypoint.1 -= distance,
            Cardinal::East => self.waypoint.0 += distance,
            Cardinal::West => self.waypoint.0 -= distance
        };
    }
    fn move_to_waypoint(self : &mut Self, distance : i32) {
        self.position.0 = self.position.0 + (self.waypoint.0 * distance);
        self.position.1 = self.position.1 + (self.waypoint.1 * distance);
    }
    fn turn_waypoint_prev(self : &mut Self) {
        self.waypoint = (-self.waypoint.1, self.waypoint.0)
    }
    fn turn_waypoint_next(self : &mut Self) {
        self.waypoint = (self.waypoint.1, -self.waypoint.0)
    }
    fn turn_waypoint(self : &mut Self, amount : i32) {
        if amount > 0 {
            self.turn_waypoint_next();
            self.turn_waypoint(amount-1);
        } else if amount < 0 {
            self.turn_waypoint_prev();
            self.turn_waypoint(amount+1);
        }
    }
    pub fn distance(self : &Self) -> u32 {
        self.position.0.unsigned_abs() + self.position.1.unsigned_abs()
    }
}