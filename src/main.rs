//TO DO
    // -- win lose condition
    // -- print input to screen
    // -- enemy AI
    // -- add notification screen for enemy AI to have character model drawn to screen
    // -- add cutscenes (https://crates.io/crates/tplay)
    // -- if above not feasible, preload ASCII art
    // -- enemy difficulty scale ([E]z, [M]edium, [A]dvanced)

use std::io;
use colored::*;

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
                playgrid.box1 = BoxState::Cross;
                break;
            },
            2 => {
                playgrid.box2 = BoxState::Cross;
                break;
            },
            3 => {
                playgrid.box3 = BoxState::Cross;
                break;
            },
            4 => {
                playgrid.box4 = BoxState::Cross;
                break;
            },
            5 => {
                playgrid.box5 = BoxState::Cross;
                break;
            },
            6 => {
                playgrid.box6 = BoxState::Cross;
                break;
            },
            7 => {
                playgrid.box7 = BoxState::Cross;
                break;
            },
            8 => {
                playgrid.box8 = BoxState::Cross;
                break;
            },
            9 => {
                playgrid.box9 = BoxState::Cross;
                break;
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

fn main() {
    let mut playgrid:Grid = Grid{box1:BoxState::Empty, box2:BoxState::Empty, box3:BoxState::Empty, box4:BoxState::Empty, box5:BoxState::Empty, box6:BoxState::Empty, box7:BoxState::Empty, box8:BoxState::Empty, box9:BoxState::Empty};
    println!("{:?}", player_turn(playgrid));
}
