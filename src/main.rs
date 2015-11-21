#[derive (Copy,Clone)]
struct Ship{
    length: i8,
    life_left: i8,
}

#[derive (Copy,Clone)]
enum Field{
    Empty,
    Hit{ship :Ship},
}

fn main() {
    let mut map = [Field::Empty;10];
    map[3] = Field::Hit{ship: Ship{length:5,life_left:5}};

    for elem in &map{
        match *elem {
            Field::Empty => print!(". "),
            Field::Hit{ship} => print!("{} ",ship.life_left)
        }
    }
}
