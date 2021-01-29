struct Color(u8, u8, u8);

struct Dummy;

struct Player {
    name: String,
    iq: u8,
    friends: u8,
    score: u16,
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

    let player = Player {
        name: "Bob".to_string(),
        iq: 10,
        friends: 10,
        score: 2,
    };

    bump_player(player, 10);
}

fn bump_player(mut player: Player, score: u16) {
    player.score += score;
    println!("Name: {}", player.name);
    println!("IQ: {}", player.iq);
    println!("Score: {}", player.score);
    println!("Friends: {}", player.friends);
}
