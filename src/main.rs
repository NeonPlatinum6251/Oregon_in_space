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

fn draw_ship(screen: &mut Vec<char>, width: usize, height: usize) {
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
                    screen[col+row*width] = ch;
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

fn init_screen(screen: &mut Vec<char>, width: usize, height: usize) {
    for k in 0..height {
        for l in 0..width {
            screen[l+k*width] = get_star()
        }
    }

    draw_ship(screen, width, height);
    
    for (i, c) in screen.iter().enumerate() {
        if i%width == 0 {
            println!("");
        }
        print!("{}", c);
    }
}

fn update_screen(screen: &mut Vec<char>,width: usize, height: usize,distance:u64,food:u64,aliens:u64) {
    clearscreen::clear().expect("failed to clear screen");
    for i in 0..height {
        screen.remove(i*width);
        screen.insert((i+1)*width -1, get_star());
    }
    draw_ship(screen, width, height);
    
    for (i, c) in screen.iter().enumerate() {
        if i%width == 0 {
            println!("");
        }
        print!("{}", c);
    }
    make_taskbar(width, distance, food, aliens);
}

fn make_taskbar(width:usize,distance:u64,food:u64,aliens:u64) {
    let topandbottom: String = "_".repeat(width-2);
    println!("{}\n| food: {}\n|distance: {}\n|aliens: {}\n {}",topandbottom,food,distance,aliens,topandbottom)
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

    let mut screen = vec![' '; width*height];

    init_screen(&mut screen, width, height);
    make_taskbar(width, distance, food, aliens);
}