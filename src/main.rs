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
        print!("\x1b[2J");
        print!("\x1b[H");
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

    fn turn(&mut self, pos: usize) -> bool {
        let x = pos / 3;
        let y = pos % 3;
        if self.field[x][y].is_none() {
            self.field[x][y] = Some(self.turn);
            return true;
        }
        false
    }

    fn check_win(&self) -> bool {
        // 引数のlistの要素が全て自分のターンのもの(まるかばつか)判定する
        let check = |list: [Option<bool>; 3]| -> bool {
            let list: Vec<bool> = list.iter().filter_map(|b| *b).collect();
            if list.len() != 3 {
                return false;
            }
            if list.iter().all(|b| *b == self.turn) {
                return true;
            }
            return false;
        };
        // 横の判定
        for y in self.field {
            if check(y) {
                return true;
            }
        }

        // 縦の判定
        for y in 0..3 {
            // 全ての行の一番目の要素を入れる配列
            let mut list: [Option<bool>; 3] = [None; 3];
            for x in 0..3 {
                list[x] = self.field[x][y];
            }
            if check(list) {
                return true;
            }
        }

        // 斜めの判定
        let mut list = [None; 3];
        for i in 0..3 {
            list[i] = self.field[i][i];
        }
        if check(list) {
            return true;
        }
        let mut list = [None; 3];
        for (i, (x, y)) in (0..3).zip((0..3).rev()).enumerate() {
            list[i] = self.field[y][x];
        }
        if check(list) {
            return true;
        }

        false
    }

    // 終了処理
    fn quit(&self) {
        self.draw();
        println!(
            "winner: {}",
            if self.turn {
                "プレイヤー1"
            } else {
                "プレイヤー2"
            }
        )
    }
}

fn main() {
    let mut game = Game::new();

    game.draw();

    loop {
        // 番号の入力を受け取る、もし数字以外ならループを始めからにする
        let Ok(pos) = input() else {
            println!("数字を入力してください");
            continue;
        };
        // 入力された数値が8以上だったらやり直し
        if pos > 8 {
            println!("数字は8までです");
            continue;
        }

        // すでにマスにまるかばつがある状態ならやり直し
        if !game.turn(pos) {
            println!("すでに埋まっています");
            continue;
        }

        game.draw();
        if game.check_win() {
            break;
        }
        game.next_turn();
    }
    game.quit();
}

fn input() -> Result<usize, std::num::ParseIntError> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>()
}

#[test]
fn check_win_test() {
    let field = [
        [
            [Some(true), None, None],
            [None, Some(true), None],
            [None, None, Some(true)],
        ],
        [
            [None, None, Some(true)],
            [None, Some(true), None],
            [Some(true), None, None],
        ],
        [
            [Some(true), None, None],
            [Some(true), None, None],
            [Some(true), None, None],
        ],
        [
            [None, Some(true), None],
            [None, Some(true), None],
            [None, Some(true), None],
        ],
        [
            [None, None, Some(true)],
            [None, None, Some(true)],
            [None, None, Some(true)],
        ],
        [
            [Some(true), Some(true), Some(true)],
            [None, None, None],
            [None, None, None],
        ],
        [
            [None, None, None],
            [Some(true), Some(true), Some(true)],
            [None, None, None],
        ],
        [
            [None, None, None],
            [None, None, None],
            [Some(true), Some(true), Some(true)],
        ],
    ];
    assert_eq!(
        field
            .into_iter()
            .all(|field| Game { field, turn: true }.check_win()),
        true
    );
}
