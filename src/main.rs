use std::fmt::Debug;

type Field = [[Option<bool>; 3]; 3];

#[derive(Debug)]
struct Game {
    field: Field,
    turn: bool,
}

impl Game {
    fn new() -> Self {
        Self {
            field: [[None; 3]; 3],
            turn: true,
        }
    }

    fn draw(&self) {
        for y in self.field {
            for x in y {
                if let Some(b) = x {
                    if b {
                        print!("[o]");
                    } else {
                        print!("[x]");
                    }
                } else {
                    print!("[ ]");
                }
            }
            println!();
        }
    }

    fn next_turn(&mut self) {
        self.turn = !self.turn;
    }
}

fn main() {
    let mut game = Game::new();

    game.draw();

    loop {
        game.draw();
        let Ok(pos) = input() else {
            continue;
        };

        let x = pos / 3;
        let y = pos % 3;
        if game.field[x][y].is_some() {
            continue;
        }
        game.field[x][y] = Some(game.turn);
        game.next_turn();
    }
}

fn input() -> Result<usize, std::num::ParseIntError> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>()
}
