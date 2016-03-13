extern crate rand;

use std::option::Option;
use std::io;
use rand::Rng;

const MAP_WIDTH: usize = 5;
const MAP_HEIGHT: usize = 5;
const MAX_ITERATIONS:u32 = 10;
const MAX_RESTARTS:u32 = 10;

type MAP = ([[Field;MAP_WIDTH];MAP_HEIGHT]);

#[derive (Copy,Clone)]
struct Ship{
    id: u8,
    length: u8,
    life_left: u8,
}

impl Ship{
    fn emplace(&self,map: &mut MAP)->bool{

        let mut rng = rand::thread_rng();

        let (x,y);
        //let candidate_area;

        if rng.gen() { //make it vertical
            x = rng.gen_range(0, MAP_WIDTH-self.length as usize);
            y = rng.gen_range(0, MAP_HEIGHT);
            println!("vertical - x: {}, y: {}",x,y );
            //candidate_area = &mut map[x..x+self.length as usize][y];

            //check if all fields are empty
            for check_x in x..x+self.length as usize{
                match map[check_x][y].ship {
                    Some(_) => return false,
                    None => {},
                }
            }

            for check_x in x..x+self.length as usize{
                map[check_x][y].ship = Some(self.id);
                println!("placing ship {} at {} {}",self.id,check_x,y);
            }

        }
        else {//make it horizontal
            x = rng.gen_range(0, MAP_WIDTH);
            y = rng.gen_range(0, MAP_HEIGHT-self.length as usize);
            println!("horizontal - x: {}, y: {}",x,y );
        //    candidate_area = &mut map[x][y..y+self.length as usize];

        //check if all fields are empty
            for check_y in y..y+self.length as usize{
                match map[x][check_y].ship {
                    Some(_) => return false,
                    None => {},
                }
            }

            for check_y in y..y+self.length as usize{
                map[x][check_y].ship = Some(self.id);
                println!("placing ship {} at {} {}",self.id,x,check_y);
            }
        }
        true
    }

    fn new(id: u8,size: u8)->Ship{
        let s = Ship{id:id,length:size,life_left:size};
        s
    }
/*    fn hit(&mut self){
        self.life_left -= 1;
    }*/
}


#[derive (Copy,Clone,Debug)]
struct Field{
    visited: bool,
    ship: Option<u8>,
}

impl Field {
    fn check(&mut self){
        if self.visited == false{
            self.visited = true;
            /*if let Some(s) = self.ship{
                s.life_left -= 1;
                if s.life_left == 0{
                    println!("Hit ads sunk!");
                }
                else{
                    println!("Hit!");
                }
            }
            else{
                println!("Miss!");
            }*/
        }
    }
}

fn print_map(map: MAP){
    for outer in map.iter(){
        for inner in outer.iter(){
            if inner.visited == false{
                print!(". ");
            }
            else{
                match inner.ship {
                    None =>print!("_ "),
                    Some(s) => print!("{} ",s),
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
            _ => Err(format!{"Please use coordinates in the range 1-{}",MAP_WIDTH}.to_owned()), //TODO: make this error msg change depending on map size.
        }
    }
}

#[test]
fn parsing_test(){
    let mut input = vec!["1","2"];
    assert_eq!(parse_coordinates(input),Ok((1,2)));

    input = vec!["1","10"];
    assert_eq!(parse_coordinates(input),Ok((1,10)));

    input = vec!["-1","5"];
    assert!(parse_coordinates(input).is_err());

    input = vec!["d","6"];
    assert!(parse_coordinates(input).is_err());

    input = vec!["1"];
    assert!(parse_coordinates(input).is_err());
}

fn main() {
    let mut map = [[Field{visited:false,ship:None};MAP_WIDTH];MAP_HEIGHT];

//ship creation
    let create_list = vec![4,3,3,3,2];
    let mut ship_array:Vec<Ship> = Vec::new();
    let mut current_id = 0;

    for size in create_list{
        ship_array.push(Ship::new(current_id, size));
        current_id += 1;
    }

//map setup
    let mut restarts = 0;
    'main_setup: loop{
        if restarts >= MAX_RESTARTS {
            panic!("Couldn't place ships after {} restarts. Aborting game",restarts);
        }
        clear_map(&mut map);
        let mut iterations;

        for s in &ship_array{
            iterations = 0;
            while !s.emplace(&mut map){
                iterations += 1;
                if iterations >= MAX_ITERATIONS {
                    println!("Couldn't put ship{} after {} tries. Starting over.",s.length,iterations);
                    restarts += 1;
                    continue 'main_setup;
                }
            }
        }
        break;
    }

//main game loop
    loop{

        let left = ship_array.len();
        if left > 0{
            println!("Ships left: {}",left );
        }
        else{
            println!("You have won the game!");
            break;
        }

        print_map(map);

        let mut input = String::new();
        io::stdin().read_line(&mut input)
        .ok()
        .expect("failed to read line");

        let v = input.trim().split_whitespace().take(2).collect::<Vec<&str>>();

        match parse_coordinates(v){
            Ok((x,y)) => map[(x-1) as usize][(y-1) as usize].check(),
            Err(s) => println!("{}",s ),
        }
        //println!("{:?}", map);
    }
}
