mod config;
mod utils;

use std::io::{self, Write};
use crossterm::{execute, terminal, cursor, style::{Print, ResetColor, SetForegroundColor, Color}};

const TEXT: &str = include_str!("./ascii.txt");

#[tokio::main]
async fn main() {
    let config = config::create_config().await;

    print_ascii_art();
    println!("Enter channel name: ");
    let mut channel = String::new();
    io::stdin().read_line(&mut channel).unwrap();
    let channel = channel.trim().to_string();

    let mut connection = config.connect().await.unwrap();

    utils::irc_client(&mut connection, channel.clone()).await;

    utils::send_message(&mut connection, channel).await;
}

fn print_ascii_art() {
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    
    execute!(io::stdout(), cursor::MoveTo(0, 0)).unwrap();

    let mut stdout = io::stdout();
    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print(format!("{} \n", TEXT)),
        ResetColor,
    ).unwrap();
    stdout.flush().unwrap();
}
