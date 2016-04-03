extern crate rand;

use std::io;
use rand::Rng;
use std::rc::Rc;
use std::cell::Cell;
use std::option::Option;

const MAP_WIDTH: usize = 5;
const MAP_HEIGHT: usize = 5;
const MAX_ITERATIONS: u32 = 10; //max amount of tries to put a ship on the map
const MAX_RESTARTS: u32 = 10;   //max amount of tries to create a valid map

type Map = ([[Field; MAP_WIDTH]; MAP_HEIGHT]); //yeah, Map is much nicer looking

struct Ship {
    id: u8,
    length: u8,
    life_left: Cell<u8>,
}

struct Field {
    visited: bool,
    ship: Option<Rc<Ship>>, // there must be a better way
}

impl Ship {
    fn new(id: u8, size: u8) -> Ship {
        Ship {
            id: id,
            length: size,
            life_left: Cell::new(size),
        }
    }
    fn hit(&self) -> u8 {
        self.life_left.set(self.life_left.get() - 1);
        println!("LIFE LEFT: {}", self.life_left.get());
        self.life_left.get()
    }
}

fn place_on_map(map: &mut Map, ship: &Rc<Ship>) -> bool {
    use std::iter::repeat;

    let len = ship.length as usize;
    let mut rng = rand::thread_rng();

    let shape: Vec<(usize, usize)> = if rng.gen() {
        let x = rng.gen_range(0, MAP_WIDTH - len);
        let y = rng.gen_range(0, MAP_HEIGHT);
        (x..).zip(repeat(y)).take(len).collect()
    } else {
        let x = rng.gen_range(0, MAP_WIDTH);
        let y = rng.gen_range(0, MAP_HEIGHT - len);
        repeat(x).zip(y..).take(len).collect()
    };

    if shape.iter().all(|&(x,y)| map[x][y].ship.is_none())  {
        for &(x, y) in &shape {
            map[x][y].ship = Some(ship.clone());
        }
        true
    }
    else {
        false
    }
}

// Without this initializing the map in main would be a PitA
impl Default for Field {
    fn default() -> Self {
        Field {
            visited: false,
            ship: None,
        }
    }
}

impl Field {
    fn check(&mut self) {
        if self.visited {
            println!("Already shot here, try again.");
        } else {
            self.visited = true;

            if let Some(ref x) = self.ship {
                match x.hit() {
                    0 => println!("Hit and sunk!"),
                    _ => println!("Hit!"),
                }
            } else {
                println!("Miss!");
            }
        }
    }
}

fn print_map(map: &Map) {
    for outer in map.iter() {
        for inner in outer.iter() {
            if inner.visited {
                match inner.ship {
                    None => print!("_ "),
                    Some(ref s) if s.life_left.get() > 0 => print!("{} ", s.id),
                    _ => print!("* "),
                }
            } else {
                print!(". ");
            }
        }
        println!("");
    }
}

fn clear_map(map: &mut Map) {
    for outer in map.iter_mut() {
        for inner in outer.iter_mut() {
            inner.visited = false;
            inner.ship = None;
        }
    }
}

fn parse_coordinates(v: &[&str]) -> Result<(u8, u8), String> {

    if v.len() == 2 {
        let x = v[0].parse::<usize>();
        let y = v[1].parse::<usize>();

        match (x, y) {
            (Err(_), _) => Err("The first coordinate is not a integer.".to_owned()),
            (_, Err(_)) => Err("The second coordinate is not a integer.".to_owned()),
            (Ok(x @ 1...MAP_WIDTH), Ok(y @ 1...MAP_HEIGHT)) => Ok((x as u8, y as u8)),
            _ => Err(format!{"Please use coordinates in the range 1-{}",MAP_WIDTH}.to_owned()),
            // TODO: the above error will not be true if MAP_WIDTH!=MAP_HEIGHT.
        }
    } else {
        Err("Please provide both coordinates.".to_owned())
    }
}

#[test]
fn parsing_test() {
    let input = ["1", "2"];
    assert_eq!(parse_coordinates(&input), Ok((1, 2)));

    let input = ["1", "5"];
    assert_eq!(parse_coordinates(&input), Ok((1, 5)));

    let input = ["-1", "5"];
    assert!(parse_coordinates(&input).is_err());

    let input = ["d", "6"];
    assert!(parse_coordinates(&input).is_err());

    let input = ["1"];
    assert!(parse_coordinates(&input).is_err());
}

fn main() {
    let mut map: [[Field; MAP_WIDTH]; MAP_HEIGHT] = Default::default();
    // thank Amon-Ra for default()

    // ship creation
    let mut ship_array: Vec<Rc<Ship>> = [4, 3, 3, 3, 2]
        .iter().cloned()
        .enumerate()
        .map(|(current_id, len)| {
            Rc::new(Ship::new(current_id as u8, len))
        })
        .collect();

    // map setup
    let mut restarts = 0;
    'main_setup: loop {
        if restarts >= MAX_RESTARTS {
            // wasn't able to put all the ships. Reduce create_list?
            panic!("Couldn't place ships after {} restarts. Aborting game",
                   restarts);
        }
        clear_map(&mut map);
        let mut iterations;

        for s in &ship_array {
            iterations = 0;
            while !place_on_map(&mut map, s) {
                iterations += 1;
                if iterations >= MAX_ITERATIONS {
                    // could not find a free space for the ship
                    println!("Couldn't put ship{} after {} tries. Starting over.",
                             s.length,
                             iterations);
                    restarts += 1;
                    continue 'main_setup; //Nah, that's not a GOTO, nope.
                }
            }
        }
        break;
    }

    //let's explain the rules.
    println!("There are ships hidden in the map below.");
    println!("If you hit one, it will display it's ID.");
    println!("If you destroy a ship, it will display '*' instead.");
    println!("A miss is marked as '_'.");
    println!("Provide input as coordinates divaded by space, ex. '2 3'");
    println!("The game will end when all ships are destoyed. Good luck!");
    // main game loop
    loop {
        ship_array.iter()//Remove any "dead" ships
            .position(|n| n.life_left.get() == 0)
            .map(|e|ship_array.remove(e))
            .is_some(); //AKA BLACK MAGIC

        let left = ship_array.len();
        if left > 0 {
            println!("Ships left: {}", left);
        } else {
            println!("You have won the game!");
            break;
        }

        print_map(&map);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let v = input.trim().split_whitespace().take(2).collect::<Vec<&str>>();
        // If I could do even more parsing on the line above, maybe parse_coordinates could be cut?
        match parse_coordinates(&v) {
            Ok((y, x)) => map[(x - 1) as usize][(y - 1) as usize].check(),
            Err(s) => println!("{}", s),
        }
    }
}
