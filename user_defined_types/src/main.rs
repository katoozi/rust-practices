struct Color(u8, u8, u8);

struct Player {
    name: String,
    iq: u8,
    friends: u8,
    score: u16,
}

impl Player {
    fn new() -> Player {
        Player {
            name: "test".to_owned(),
            iq: 10,
            friends: 20,
            score: 100,
        }
    }
}

fn main() {
    let value = Color(1, 2, 4);
    println!("{}", value.0);
    println!("{}", value.1);
    println!("{}", value.2);

    let Color(reg, green, blue) = value;

    println!("red: {}, green: {}, blue: {}", reg, green, blue);

    let Color(r, _, b) = value;

    println!("red: {}, blue: {}", r, b);

    let mut player = Player::new();
    player.name = "mohammad".to_string();

    bump_player(player, 10);
}

fn bump_player(mut player: Player, score: u16) {
    player.score += score;
    println!("Name: {}", player.name);
    println!("IQ: {}", player.iq);
    println!("Score: {}", player.score);
    println!("Friends: {}", player.friends);
}
