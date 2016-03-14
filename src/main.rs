extern crate rand;

use std::io;
use rand::Rng;
use std::rc::Rc;
use std::cell::RefCell;
use std::option::Option;

const MAP_WIDTH: usize = 5;
const MAP_HEIGHT: usize = 5;
const MAX_ITERATIONS:u32 = 10; //max amount of tries to put a ship on the map
const MAX_RESTARTS:u32 = 10;   //max amount of tries to create a valid map

type MAP = ([[Field;MAP_WIDTH];MAP_HEIGHT]); //yeah, MAP is much nicer looking

struct Ship{
    id: u8,
    length: u8,
    life_left: u8,
}

struct Field{
    visited: bool,
    ship: Option<Rc<RefCell<Ship>>>, //there must be a better way
}

impl Ship{
    fn new(id: u8,size: u8)->Ship{
        let s = Ship{id:id,length:size,life_left:size};
        s
    }
    fn hit(&mut self)->u8{
        self.life_left -= 1;
        println!("LIFE LEFT: {}",self.life_left );
        self.life_left
    }
}

fn place_on_map(map: &mut MAP,ship_rc_refcell: &Rc<RefCell<Ship>>)->bool{

    let mut rng = rand::thread_rng();

    let (x,y);
    let ship_rc = ship_rc_refcell.borrow();

//I would LOVE to fold this into a single codepath somehow:

    if rng.gen() { //make it vertical
        x = rng.gen_range(0, MAP_WIDTH-ship_rc.length as usize);
        y = rng.gen_range(0, MAP_HEIGHT);

        //check if all fields are empty
        for check_x in x..x+ship_rc.length as usize{
            if let Some(_) = map[check_x][y].ship{
                return false;
            }
        }
        //place the ship
        for check_x in x..x+ship_rc.length as usize{
            map[check_x][y].ship = Some(ship_rc_refcell.clone());
        }

    }
    else {//make it horizontal
        x = rng.gen_range(0, MAP_WIDTH);
        y = rng.gen_range(0, MAP_HEIGHT-ship_rc.length as usize);

    //check if all fields are empty
        for check_y in y..y+ship_rc.length as usize{
            if let Some(_) = map[x][check_y].ship{
                return false;
            }
        }
        //place the ship
        for check_y in y..y+ship_rc.length as usize{
            map[x][check_y].ship = Some(ship_rc_refcell.clone());
        }
    }
    true
}
//Without this initializing the map in main would be a PitA
impl Default for Field{
    fn default() -> Self {
        Field { visited: false, ship: None, }
    }
}

impl Field {
    fn check(&mut self){
        if self.visited == false{
            self.visited = true;

            if let Some(ref mut x) = self.ship{
                match x.borrow_mut().hit(){
                    0 => println!("Hit and sunk!"),
                    _ => println!("Hit!"),
                }
            }
            else{
                println!("Miss!");
            }
        }
        else{
            println!("Already shot here, try again.");
        }
    }
}

fn print_map(map: &MAP){
    for outer in map.iter(){
        for inner in outer.iter(){
            if inner.visited == false{
                print!(". ");
            }
            else{
                match inner.ship {
                    None =>print!("_ "),
                    Some(ref s) if s.borrow().life_left > 0 => print!("{} ",s.borrow().id),
                    _ => print!("* "),
                }
            }

        }
        println!("");
    }
}

fn clear_map(map: &mut MAP){
    for outer in map.iter_mut(){
        for inner in outer.iter_mut(){
                inner.visited = false;
                inner.ship = None;
        }
    }
}

fn parse_coordinates(v: Vec<&str>)->Result<(u8,u8),String>{

    if v.len() != 2{
        Err("Please provide both coordinates.".to_owned())
    }
    else{

        let x = v[0].parse::<usize>();
        let y = v[1].parse::<usize>();

        match (x,y){
            (Err(_),_) => Err("The first coordinate is not a integer.".to_owned()),
            (_,Err(_)) => Err("The second coordinate is not a integer.".to_owned()),
            (Ok(x @ 1...MAP_WIDTH),Ok(y @ 1...MAP_HEIGHT)) => Ok((x as u8,y as u8)),
            _ => Err(format!{"Please use coordinates in the range 1-{}",MAP_WIDTH}.to_owned()),
            //TODO: the above error will not be true if MAP_WIDTH!=MAP_HEIGHT.
        }
    }
}

#[test]
fn parsing_test(){
    let mut input = vec!["1","2"];
    assert_eq!(parse_coordinates(input),Ok((1,2)));

    input = vec!["1","5"];
    assert_eq!(parse_coordinates(input),Ok((1,5)));

    input = vec!["-1","5"];
    assert!(parse_coordinates(input).is_err());

    input = vec!["d","6"];
    assert!(parse_coordinates(input).is_err());

    input = vec!["1"];
    assert!(parse_coordinates(input).is_err());
}

fn main() {
    let mut map: [[Field;MAP_WIDTH];MAP_HEIGHT] = Default::default(); //thank Amon-Ra for default()

//ship creation
    let create_list = vec![4,3,3,3,2];
    let mut ship_array:Vec<Rc<RefCell<Ship>>> = Vec::with_capacity(create_list.len());//such opimaz
    let mut current_id = 0;

    for size in create_list{
        ship_array.push(Rc::new(RefCell::new(Ship::new(current_id, size))));//a mouth full, isnt it
        current_id += 1;
    }

//map setup
    let mut restarts = 0;
    'main_setup: loop{
        if restarts >= MAX_RESTARTS { //wasn't able to put all the ships. Reduce create_list?
            panic!("Couldn't place ships after {} restarts. Aborting game",restarts);
        }
        clear_map(&mut map);
        let mut iterations;

        for s in &ship_array{
            iterations = 0;
            while !place_on_map(&mut map, s){
                iterations += 1;
                if iterations >= MAX_ITERATIONS {// could not find a free space for the ship
                    println!("Couldn't put ship{} after {} tries. Starting over."
                        ,s.borrow().length,iterations);
                    restarts += 1;
                    continue 'main_setup; //Nah, that's not a GOTO, nope.
                }
            }
        }
        break;
    }

//main game loop
    loop{
        ship_array.iter()
            .position(|n| n.borrow().life_left <= 0)
            .map(|e|ship_array.remove(e))
            .is_some(); //AKA BLACK MAGIC

        let left = ship_array.len();
        if left > 0{
            println!("Ships left: {}",left );
        }
        else{
            println!("You have won the game!");
            break;
        }

        print_map(&map);

        let mut input = String::new();
        io::stdin().read_line(&mut input)
        .ok()
        .expect("failed to read line");

        let v = input.trim().split_whitespace().take(2).collect::<Vec<&str>>();
        //If I could do even more parsing on the line above, maybe parse_coordinates could be cut?
        match parse_coordinates(v){
            Ok((y,x)) => map[(x-1) as usize][(y-1) as usize].check(),
            Err(s) => println!("{}",s ),
        }
    }
}
