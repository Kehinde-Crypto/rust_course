enum Direction{
    North,
    East,
    West,
    South,
}
fn main(){
    let move_direction = Direction::West;
    match move_direction{
        Direction::North => println!("Heading North!"),
        Direction::South => println!("Heading South!"),
        Direction::East => println!("Heading East!"),
        Direction::West => println!("Heading West!"),
    }
}