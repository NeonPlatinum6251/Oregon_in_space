use rand::Rng;
use term_size;
use std::thread;
use std::time::Duration;
use crossterm::event::{poll,read,Event,KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::io::{stdout, Write};
use crossterm::{
    execute,
    cursor::MoveTo,
    terminal::{Clear, ClearType},
};

/*TODO:  
Add Random events 
Aliens should multiply -> 
this should be what makes the game harder as more food is required */
struct Game {
    distance:u64,
    food:u64,
    aliens:u64,
    money:u64,
}
enum RandEvent {
    Sickness(bool),
    NoEvent(bool),
    MeteorShower(bool),
}

impl Game {
    fn tick(&mut self) {
        self.distance += 1;
        if self.food <= self.aliens {
            self.aliens -= 1;
            self.food = 0  
        } else {
            self.food -= self.aliens;
        }

        let mut rng = rand::rng();
        let birth_chance = self.aliens/2;
        for n in 1..birth_chance {
            if rng.random_range(0..100) == 2 {
                self.aliens +=1
            }
        }
    }
    //below are the key functions 
    fn add_money(&mut self) {
        self.money += 1
    }

    fn buy_food(&mut self) {
        if self.money > 5 {
            self.money -= 5;
            self.food += 100;
        }

    }
}

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
}

fn update_star_field(screen: &mut Vec<char>, width: usize, height: usize) {
    for row in 0..height {
        for col in 0..(width - 1) {
            let idx = row * width + col;
            if screen[idx] == '*' || screen[idx] == ' ' {
                screen[idx] = screen[idx + 1];
            }
        }
        screen[row * width + (width - 1)] = get_star();
    }
}

fn render(screen: &Vec<char>, width: usize, height: usize, game: &Game) {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0), Clear(ClearType::All)).unwrap();

    for row in 0..height - 5 { 
        for col in 0..width {
            print!("{}", screen[row * width + col]);
        }
        print!("\r\n");
    }
    let taskbar_start = height.saturating_sub(5);
    execute!(stdout, MoveTo(0, taskbar_start as u16)).unwrap();
    let line = "_".repeat(width.saturating_sub(2));
    println!("┌{}", line);
    execute!(stdout,MoveTo(0,(taskbar_start as u16)+1)).unwrap();
    println!("| food: {}  | distance: {}  | aliens: {}  | money: {}", game.food, game.distance, game.aliens,game.money);
    execute!(stdout,MoveTo(0,(taskbar_start as u16)+2)).unwrap();
    println!("└	{}", line);

    stdout.flush().unwrap();
}


fn handle_input() -> Option<KeyCode> {
    if poll(Duration::from_millis(0)).unwrap() {
        if let Event::Key(event) = read().unwrap() {
            return Some(event.code);
        }
    }
    None
}

fn main() {
    enable_raw_mode().unwrap();
    let (w, h) = get_terminal_size();
    let width = w;
    let height = h;
    let mut the_game:Game = Game {
            distance: 0,
            food: 1000,
            aliens:10,
            money:0,
        };
    
    if width == 0 || height == 0 {
        eprintln!("Terminal size returned zero dimension: {:?}x{:?}", width, height);
        return;
    }

     let mut screen: Vec<char> = vec![' '; width * height];
    init_screen(&mut screen, width, height);
    draw_ship(&mut screen, width, height);

    while the_game.aliens != 0 {
        if let Some(key) = handle_input() {
            match key {
                KeyCode::Char('q') => break,
                KeyCode::Char('f') => {
                    if the_game.food > 0 { the_game.food -= 1; }
                }
                KeyCode::Char('h') => the_game.add_money(),
                KeyCode::Char('b') => the_game.buy_food(),
                _ => {}
            }
        }

        update_star_field(&mut screen, width, height);
        draw_ship(&mut screen, width, height);
        render(&screen, width, height, &the_game);
        the_game.tick();

        thread::sleep(Duration::from_millis(300));
    }
    disable_raw_mode().unwrap();
}