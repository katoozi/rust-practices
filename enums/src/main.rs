#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

enum PlayerAction {
    Move { direction: Direction, speed: u8 },
    Wait,
    Attack(Direction),
}

fn main() {
    let simulated_player_action = PlayerAction::Move {
        direction: Direction::N,
        speed: 10,
    };

    match simulated_player_action {
        PlayerAction::Wait => println!("player wants to wait"),
        PlayerAction::Move { direction, speed } => {
            println!(
                "player wants to move in {:?} with {} speed",
                direction, speed
            );
        }
        PlayerAction::Attack(direction) => {
            println!("player want to attack direction {:?}", direction);
        }
    }
}
