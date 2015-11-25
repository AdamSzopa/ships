use std::option::Option;
use std::io;

#[derive (Copy,Clone)]
struct Ship{
    id: u8,
    length: i8,
    life_left: i8,
}

impl Ship{
    fn hit(&mut self){
        self.life_left -= 1;
    }
}

#[derive (Copy,Clone)]
struct Field{
    visited: bool,
    ship: Option<i8>,
}

impl Field {
    fn check(&mut self){
        self.visited = true;
    }
}

fn print_map(map: [[Field;10];10]){
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

fn parse_coordinates(v: Vec<&str>)->Result<(u8,u8),String>{

    if v.len() != 2{
        Err("Please provide both coordinates.".to_owned())
    }
    else{

        let x = v[0].parse::<i8>();
        let y = v[1].parse::<i8>();

        match (x,y){
            (Err(_),_) => Err("The first coordinate is not a integer.".to_owned()),
            (_,Err(_)) => Err("The second coordinate is not a integer.".to_owned()),
            (Ok(x @ 1...10),Ok(y @ 1...10)) => Ok((x as u8,y as u8)),
            _ => Err("Please use coordinates in the range 1-10.".to_owned()),
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
    let mut map = [[Field{visited:false,ship:None};10];10];

    let ship_array = [Ship{id:1,length:5,life_left:5}];

    let ship5 = Ship{id:1,length:5,life_left:5};

    map[3][2] = Field{ship:Some(ship5.length),..map[3][2]};
    map[3][3] = Field{ship:Some(ship5.length),..map[3][2]};


    loop{
        print_map(map);

        let mut input = String::new();
        io::stdin().read_line(&mut input)
        .ok()
        .expect("failed to read line");
println!("{}",input );
        let v = input.trim().split_whitespace().take(2).collect::<Vec<&str>>();

        match parse_coordinates(v){
            Ok((x,y)) => map[(x-1) as usize][(y-1) as usize].check(),
            Err(s) => println!("{}",s ),
        }
    }






}
