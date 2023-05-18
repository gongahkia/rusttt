//TO DO
    // -- win lose condition
    // -- print input to screen
    // -- enemy AI
    // -- add notification screen for enemy AI to have character model drawn to screen
    // -- add cutscenes (https://crates.io/crates/tplay)
    // -- if above not feasible, preload ASCII art
    // -- enemy difficulty scale ([E]z, [M]edium, [A]dvanced)
    // -- add state checker for BoxState

use std::io;
use rand::Rng;
use colored::*;

#[derive(Debug)]
enum Difficulty {
    Easy,
    Medium,
    Advanced
}

#[derive(Debug)]
// player input is cross [X]
// AI input is circle [O]
enum BoxState {
    Empty,
    Cross,
    Circle
}

#[derive(Debug)]
struct Grid {
    box1:BoxState,
    box2:BoxState,
    box3:BoxState,
    box4:BoxState,
    box5:BoxState,
    box6:BoxState,
    box7:BoxState,
    box8:BoxState,
    box9:BoxState
}

fn player_turn(mut playgrid:Grid) -> Grid {

    loop {
        println!("{}", "Let's play some Tic Tac Toe".yellow());
        println!("\n [{}] | [{}] | [{}] \n [{}] | [{}] | [{}] \n [{}] | [{}] | [{}] \n", "1".yellow(), "2".yellow(), "3".yellow(), "4".yellow(), "5".yellow(), "6".yellow(), "7".yellow(), "8".yellow(), "9".yellow());
        println!("{}","Number 1-9:".yellow());
        
        let mut player_input:String = String::new();
        io::stdin().read_line(&mut player_input).expect("Failed to read line");
        let player_input_num:i8 = player_input.trim_end().parse().expect("Failed to parse number");

        match player_input_num {
            1 => {
                match playgrid.box1 {
                    BoxState::Empty => {
                        playgrid.box1 = BoxState::Cross;
                        break;
                    },
                    BoxState::Cross => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                    BoxState::Circle => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                }
            },
            2 => {
                match playgrid.box2 {
                    BoxState::Empty => {
                        playgrid.box2 = BoxState::Cross;
                        break;
                    },
                    BoxState::Cross => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                    BoxState::Circle => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                }
            },
            3 => {
                match playgrid.box3 {
                    BoxState::Empty => {
                        playgrid.box3 = BoxState::Cross;
                        break;
                    },
                    BoxState::Cross => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                    BoxState::Circle => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                }
            },
            4 => {
                match playgrid.box4 {
                    BoxState::Empty => {
                        playgrid.box4 = BoxState::Cross;
                        break;
                    },
                    BoxState::Cross => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                    BoxState::Circle => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                }
            },
            5 => {
                match playgrid.box5 {
                    BoxState::Empty => {
                        playgrid.box5 = BoxState::Cross;
                        break;
                    },
                    BoxState::Cross => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                    BoxState::Circle => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                }
            },
            6 => {
                match playgrid.box6 {
                    BoxState::Empty => {
                        playgrid.box6 = BoxState::Cross;
                        break;
                    },
                    BoxState::Cross => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                    BoxState::Circle => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                }
            },
            7 => {
                match playgrid.box7 {
                    BoxState::Empty => {
                        playgrid.box7 = BoxState::Cross;
                        break;
                    },
                    BoxState::Cross => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                    BoxState::Circle => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                }
            },
            8 => {
                match playgrid.box8 {
                    BoxState::Empty => {
                        playgrid.box8 = BoxState::Cross;
                        break;
                    },
                    BoxState::Cross => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                    BoxState::Circle => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                }
            },
            9 => {
                match playgrid.box9 {
                    BoxState::Empty => {
                        playgrid.box9 = BoxState::Cross;
                        break;
                    },
                    BoxState::Cross => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                    BoxState::Circle => {
                        println!("{}", "Square already taken! Choose an empty square!".red());
                        continue;
                    },
                }
            }
            _ => {
                //error handling
                println!("{}", "Invalid input!".red());
                println!("{}", "Number 1-9:".yellow());
                continue
            }
        }
    }
    playgrid
}

fn beginner_enemy_turn(mut playgrid:Grid) -> Grid {
    loop {
        let beginner_enemy_move:i8 = rand::thread_rng().gen_range(1..10);
        match beginner_enemy_move {
            1 => {
                match playgrid.box1 {
                    BoxState::Empty => {
                        playgrid.box1 = BoxState::Circle;
                        break;
                    },
                    BoxState::Cross => {
                        continue;
                    },
                    BoxState::Circle => {
                        continue;
                    },
                }
            },
            2 => {
                match playgrid.box2 {
                    BoxState::Empty => {
                        playgrid.box2 = BoxState::Circle;
                        break;
                    },
                    BoxState::Cross => {
                        continue;
                    },
                    BoxState::Circle => {
                        continue;
                    },
                }
            },
            3 => {
                match playgrid.box3 {
                    BoxState::Empty => {
                        playgrid.box3 = BoxState::Circle;
                        break;
                    },
                    BoxState::Cross => {
                        continue;
                    },
                    BoxState::Circle => {
                        continue;
                    },
                }
            },
            4 => {
                match playgrid.box4 {
                    BoxState::Empty => {
                        playgrid.box4 = BoxState::Circle;
                        break;
                    },
                    BoxState::Cross => {
                        continue;
                    },
                    BoxState::Circle => {
                        continue;
                    },
                }
            },
            5 => {
                match playgrid.box5 {
                    BoxState::Empty => {
                        playgrid.box5 = BoxState::Circle;
                        break;
                    },
                    BoxState::Cross => {
                        continue;
                    },
                    BoxState::Circle => {
                        continue;
                    },
                }
            },
            6 => {
                match playgrid.box6 {
                    BoxState::Empty => {
                        playgrid.box6 = BoxState::Circle;
                        break;
                    },
                    BoxState::Cross => {
                        continue;
                    },
                    BoxState::Circle => {
                        continue;
                    },
                }
            },
            7 => {
                match playgrid.box7 {
                    BoxState::Empty => {
                        playgrid.box7 = BoxState::Circle;
                        break;
                    },
                    BoxState::Cross => {
                        continue;
                    },
                    BoxState::Circle => {
                        continue;
                    },
                }
            },
            8 => {
                match playgrid.box8 {
                    BoxState::Empty => {
                        playgrid.box8 = BoxState::Circle;
                        break;
                    },
                    BoxState::Cross => {
                        continue;
                    },
                    BoxState::Circle => {
                        continue;
                    },
                }
            },
            9 => {
                match playgrid.box9 {
                    BoxState::Empty => {
                        playgrid.box9 = BoxState::Circle;
                        break;
                    },
                    BoxState::Cross => {
                        continue;
                    },
                    BoxState::Circle => {
                        continue;
                    },
                }
            },
            _ => (),
        }
    }
    playgrid
}

/*fn display_grid(playgrid:Grid) -> &str {
    // fill this out later!
}*/

fn main() {
    let mut playgrid:Grid = Grid{box1:BoxState::Circle, box2:BoxState::Cross, box3:BoxState::Empty, box4:BoxState::Empty, box5:BoxState::Empty, box6:BoxState::Empty, box7:BoxState::Empty, box8:BoxState::Empty, box9:BoxState::Empty};
    println!("{:?}", player_turn(playgrid));
}
