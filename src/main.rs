use std::io;
use itertools::izip;
use std::fmt; // display trait for tiles
use rand::Rng; // take guesses, show boards, setup boards

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum TileKind {
    Miss,
    VerticalShip,
    HorizontalShip,
    HitShip,
    SinkShip,
    Water,
}

impl TileKind {
    fn is_hit(&self) -> bool {
        self == &TileKind::HitShip ||
        self == &TileKind::SinkShip
    }

    fn is_ship(&self) -> bool {
        self == &TileKind::VerticalShip ||
        self == &TileKind::HorizontalShip
    }
}

impl fmt::Display for TileKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
        match self {
            Self::Miss => '.',
            Self::VerticalShip => '|',
            Self::HorizontalShip => '=',
            Self::HitShip => 'x',
            Self::SinkShip => 'X',
            Self::Water => ' ',
        })
    }
}

const TOTAL_SHIP_TILES: u8 = 17;

type Board = [[TileKind; 10]; 10];

fn main() {
    /* initializes boards, counters, and rng */
    let mut your_board = board();
    let mut view = [[TileKind::Water; 10]; 10];
    let mut your_successful_guesses: u8 = 0;
    let mut computers_board = board();
    let mut computers_successful_guesses: u8 = 0;
    let mut r = rand::thread_rng();

    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    show(your_board);
    show(view);

    let mut p: Option<Point> = takeguess();
    while p.is_none() {
        p = takeguess();
    }
    let p: Point = p.unwrap();

    let mut hit = guess(&mut computers_board, &mut computers_successful_guesses, p);
    changeview(&mut view, computers_board, p);

    let mut x: usize = r.gen_range(0..10);
    let mut y: usize = r.gen_range(0..10);
    guess(&mut your_board, &mut your_successful_guesses, Point{x, y});

    /* main gameplay loop */
    loop {
        /* refresh screen every turn */
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        show(your_board);
        println!("\n{}", if hit {"Hit"} else {"Miss..."});
        show(view);
        println!();

        /* takes guess, allows to keep guessing if you mess up */
        let mut p: Option<Point> = takeguess();
        while p.is_none() {
            p = takeguess();
        }
        let p: Point = p.unwrap();

        hit = guess(&mut computers_board, &mut computers_successful_guesses, p);
        changeview(&mut view, computers_board, p);
        if computers_successful_guesses == TOTAL_SHIP_TILES {
            println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
            show(your_board);
            show(view);
            println!("Player 1 wins!");
            break;
        }

        x = r.gen_range(0..10);
        y = r.gen_range(0..10);
        while your_board[y][x].is_hit() || your_board[y][x] == TileKind::Miss {
            x = r.gen_range(0..10);
            y = r.gen_range(0..10);
        }
        guess(&mut your_board, &mut your_successful_guesses, Point{x, y});
        if your_successful_guesses == TOTAL_SHIP_TILES {
            println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
            println!("Player 2 wins!");
            show(your_board);
            show(computers_board);
            break;
        }
    }
}

/**
 * Takes a guess from the standard input, ranging from a0-j9,
   and returns it.
 */
fn takeguess() -> Option<Point> {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: &str = &guess;
    
    match (guess.as_bytes()[0], guess.as_bytes()[1]) {
        (b'a'..=b'j', b'0'..=b'9') => {
            match (guess.as_bytes()[0], guess.as_bytes()[1]) {
                (y, x) => {
                    Some(Point{
                        x: (x - b'0') as usize,
                        y: (y - b'a') as usize,
                    })
                }
            }
        }
        _ => None,
    }
}

/**
 * Given a board and guess, processes the guess.
 * Returns true and increments the counter on a successful guess,
   false otherwise.
 */
fn guess(board: &mut Board, counter: &mut u8, p: Point) -> bool {
    if board[p.y][p.x].is_ship() {
        board[p.y][p.x] = TileKind::HitShip;
        *counter += 1;
        true
    } else if board[p.y][p.x] == TileKind::HitShip {
        true
    } else {
        board[p.y][p.x] = TileKind::Miss;
        false
    }
}

/**
 * Given a board and point, updates the player's view.
 */
fn changeview(view: &mut Board, board: Board, p: Point) {
    if board[p.y][p.x].is_hit() {
        view[p.y][p.x] = board[p.y][p.x];
    } else {
        view[p.y][p.x] = TileKind::Miss;
    }
}

/**
 * Given a board, prints it to the standard output.
 */
fn show(board: Board) {
    print!(" ");
    for i in 0..10 {
        print!(" {}", i);
    }
    for (row, char) in izip!(board, 'a'..'k') {
        print!("\n{} ", char);
        for c in row {
            print!("{} ", c);
        }
    }
    println!();
}

/**
 * Returns a new randomly generated board.
 */
fn board() -> Board {
    let mut board = [[TileKind::Water; 10]; 10];
    let mut r = rand::thread_rng();
    let ship_lengths = [5, 4, 3, 3, 2];

    for ship_length in ship_lengths {
        if rand::random() {
            loop {
                let mut canplace = true;
                let x: usize = r.gen_range(0..10);
                let y: usize = r.gen_range(0..(10 - ship_length + 1));
    
                for i in 0..ship_length {
                    if board[y + i][x] != TileKind::Water {
                        canplace = false;
                    }
                }
                if canplace {
                    for i in 0..ship_length {
                        board[y + i][x] = TileKind::VerticalShip;
                    }
                    break;
                }
            }
        } else {
            loop {
                let mut canplace = true;
                let x: usize = r.gen_range(0..(10 - ship_length + 1));
                let y: usize = r.gen_range(0..10);
    
                for j in 0..ship_length {
                    if board[y][x + j] != TileKind::Water {
                        canplace = false;
                    }
                }
                if canplace {
                    for j in 0..ship_length {
                        board[y][x + j] = TileKind::HorizontalShip;
                    }
                    break;
                }
            }
        }
    }
    board
}
