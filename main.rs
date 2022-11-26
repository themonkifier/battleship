use std::io;
use itertools::izip;
use rand::Rng;
// take guesses, show boards, setup boards

#[derive(Debug, Copy, Clone)]
struct Point {
    x: u8,
    y: u8
}

type Board = [[char; 10]; 10];

fn main() {
    /* initializes boards, counters, and rng */
    let mut b1 = board();
    let mut view = [[' '; 10]; 10];
    let mut c1: u8 = 0;
    let mut b2 = board();
    let mut c2: u8 = 0;
    let mut r = rand::thread_rng();

    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    show(b1);
    show(view);

    let mut p: Option<Point> = takeguess();
    while p.is_none() {
        p = takeguess();
    }
    let p: Point = p.unwrap();

    let mut hit = guess(&mut b2, &mut c2, p);
    changeview(&mut view, b2, p);

    let mut x: u8 = r.gen_range(0..10);
    let mut y: u8 = r.gen_range(0..10);
    guess(&mut b1, &mut c1, Point{x, y});

    /* main gameplay loop */
    loop {
        /* refresh screen every turn */
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        show(b1);
        println!();
        println!("{}", if hit {"Hit!"} else {"Miss..."});
        show(view);
        println!();

        /* takes guess, allows to keep guessing if you mess up */
        let mut p: Option<Point> = takeguess();
        while p.is_none() {
            p = takeguess();
        }
        let p: Point = p.unwrap();

        hit = guess(&mut b2, &mut c2, p);
        changeview(&mut view, b2, p);
        if c2 == 17 {
            println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
            show(b1);
            show(view);
            println!("Player 1 wins!");
            break;
        }

        x = r.gen_range(0..10);
        y = r.gen_range(0..10);
        while b1[y as usize][x as usize] == 'x' || b1[y as usize][x as usize] == '.' {
            x = r.gen_range(0..10);
            y = r.gen_range(0..10);
        }
        guess(&mut b1, &mut c1, Point{x, y});
        if c1 == 17 {
            println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
            println!("Player 2 wins!");
            show(b1);
            show(b2);
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
                    let x = x - 48;
                    let y = y - 97;
                    Some(Point { x, y })
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
    if board[p.y as usize][p.x as usize] == '—' || board[p.y as usize][p.x as usize] == '|' {
        board[p.y as usize][p.x as usize] = 'x';
        *counter += 1;
        true
    } else if board[p.y as usize][p.x as usize] == 'x' {
        true
    } else {
        board[p.y as usize][p.x as usize] = '.';
        false
    }
}

/**
 * Given a board and point, updates the player's view.
 */
fn changeview(view: &mut Board, board: Board, p: Point) {
    if board[p.y as usize][p.x as usize] == 'x' {
        view[p.y as usize][p.x as usize] = 'x';
    } else {
        view[p.y as usize][p.x as usize] = '.';
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
    let mut board = [[' '; 10]; 10];
    let mut r = rand::thread_rng();
    let lengths = [5, 4, 3, 3, 2];

    for length in lengths {
        if rand::random() {
            loop {
                let mut canplace = true;
                let x: u8 = r.gen_range(0..10);
                let y: u8 = r.gen_range(0..(10 - length + 1));
    
                for i in 0..length {
                    if board[(y + i) as usize][x as usize] != ' ' {
                        canplace = false;
                    }
                }
                if canplace {
                    for i in 0..length {
                        board[(y + i) as usize][x as usize] = '|';
                    }
                    break;
                }
            }
        } else {
            loop {
                let mut canplace = true;
                let x: u8 = r.gen_range(0..(10 - length + 1));
                let y: u8 = r.gen_range(0..10);
    
                for j in 0..length {
                    if board[y as usize][(x + j) as usize] != ' ' {
                        canplace = false;
                    }
                }
                if canplace {
                    for j in 0..length {
                        board[y as usize][(x + j) as usize] = '—';
                    }
                    break;
                }
            }
        }
    }
    board
}