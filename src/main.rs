use std::rc::Rc;

#[derive (Copy,Clone)]
struct Ship{
    length: i8,
    life_left: i8,
}

impl Ship{
    fn hit(&mut self){
        self.life_left -= 1;
    }
}

#[derive (Copy,Clone)]
enum Field{
    Empty,
    Hit{ship: Ship},
}

enum Field2{
    Empty,
    Hit{ship: Rc<Ship>},
}

fn main() {
    let mut map = [[Field::Empty;10];10];

    let mut ship7 = Ship{length:7,life_left:7};
    let mut map2 = [[Field2::Empty,Field2::Hit{ship: Rc::new(ship7)},Field2::Empty],[Field2::Empty,Field2::Empty,Field2::Empty]];

    let ship5 = Ship{length:5,life_left:5};
    map[3][2] = Field::Hit{ship: ship5};
    map[3][3] = Field::Hit{ship: ship5};

        for outer in map.iter_mut(){
            for inner in outer.iter_mut(){
                match *inner {
                    Field::Empty => print!(". "),
                    Field::Hit{ref mut ship} => {
                        ship.hit();
                        print!("{} ",ship.life_left);
                    }
                }
            }
            println!("");
        }
        for outer in map2.iter_mut(){
            for inner in outer.iter_mut(){
                match *inner {
                    Field2::Empty => print!(". "),
                    Field2::Hit{ref mut ship} => {
                        //ship.hit();
                        print!("{} ",ship.life_left);
                    }
                }
            }
            println!("");
        }
}
