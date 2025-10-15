use rand::Rng;
use term_size;
use clearscreen;
use std::{thread, time};

fn get_terminal_size() -> (usize, usize) {
    if let Some((w, h)) = term_size::dimensions() {
        (w, h)
    } else {
        (80, 24)
    }
}

fn draw_ship(screen: &mut Vec<Vec<char>>, width: usize, height: usize) {
    let ship: [&'static str; 4] = [
        "     ___",
        " ___/   \\___",
        "/   '---'   \\",
        "'--_______--'",
    ];
    let ship_height = ship.len();
    let ship_width = ship.iter().map(|line| line.len()).max().unwrap_or(0);

    let start_row = (height as isize - ship_height as isize) / 2;
    let start_col = (width as isize - ship_width as isize) / 2;

    for (i, line) in ship.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            let row_i: isize = start_row + i as isize;
            let col_j: isize = start_col + j as isize;

            if row_i >= 0 && col_j >= 0 {
                let row = row_i as usize;
                let col = col_j as usize;
                if row < height && col < width {
                    screen[row][col] = ch;
                }
            }
        }
    }
}

fn get_star() -> char {
    let mut rng = rand::rng();
    if rng.random_range(0..25) == 7 {
        '*'
    } else {
        ' '
    }
}

fn init_screen(screen: &mut Vec<Vec<char>>, width: usize, height: usize) {
    for k in 0..height {
        for l in 0..width {
            screen[k][l] = get_star()
        }
    }

    draw_ship(screen, width, height);
    
    for row in screen.iter() {
        // collect chars into a String and print
        let s: String = row.iter().collect();
        println!("{}", s);
    }
}

fn update_screen(screen: &mut Vec<Vec<char>>,width: usize, height: usize) {
    clearscreen::clear().expect("failed to clear screen");
    for row in screen.iter_mut() {
        row.remove(0);
        row.push(get_star())
    }
    draw_ship(screen, width, height);

    for row in screen.iter() {
        // collect chars into a String and print
        let s: String = row.iter().collect();
        println!("{}", s);
    }
}

fn make_taskbar(width:usize,distance:u64,food:u64,aliens:u64) {
    
}

fn main() {
    let (w, h) = get_terminal_size();
    let width = w;
    let height = h;
    let mut distance :u64 = 0;
    let mut food :u64 = 0; 
    let mut aliens :u64 = 0;
    let mut speed :u64 = 1;
    
    if width == 0 || height == 0 {
        eprintln!("Terminal size returned zero dimension: {:?}x{:?}", width, height);
        return;
    }

    let mut screen = vec![vec![' '; width]; height];

    init_screen(&mut screen, width, height);

    for gametime in (0..20) {
        thread::sleep(time::Duration::from_millis(500));
        update_screen(&mut screen, width, height);
    }
}