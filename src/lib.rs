use std::env;
use std::fs;
use std::error::Error;
use unicode_width::UnicodeWidthStr;
use unicode_segmentation::UnicodeSegmentation;

const SLOTH_ART: &'static str = include_str!("sloth.txt");

pub struct Config {
    tasks_path: String,
}

impl Config {
    pub fn new(_args: &[String]) -> Result<Config, &'static str> {
        let tasks_path = Config::get_tasks_path()?;
        Ok(Config { tasks_path })
    }

    fn get_tasks_path() -> Result<String, &'static str> {
        match env::var("SLOTH_TASKS") {
            Ok(val) => Ok(val),
            Err(_) => match env::var("HOME") {
                Ok(val) => Ok(val + "/.sloth/tasks.txt"),
                Err(_) => Err("Couldn't determine location of tasks file. Neither SLOTH_TASKS nor HOME is set."),
            },
        }
    }
}

fn parse_tasks(contents: &String) -> Vec<String> {
    // Tasks are delimited by a blank line
    let mut tasks = Vec::new();
    let mut curr_task = String::new();
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !curr_task.is_empty() {
                curr_task.pop();
                tasks.push(curr_task.clone());
                curr_task.clear();
            }
        }
        else {
            curr_task = curr_task + line + "\n"
        }
    }
    if !curr_task.is_empty() {
        curr_task.pop();
        tasks.push(curr_task);
    }
    tasks
}

fn pick_task(tasks: &Vec<String>) -> String {
    if tasks.is_empty() {
        return String::from("Chillax");
    }
    let i = rand::random::<usize>() % tasks.len();
    tasks.get(i).unwrap().to_string()
}

fn rewrap(msg: &str, max_width: usize) -> String
{
    let mut rewrapped = String::new();
    let mut curr_line = String::new();
    for line in msg.lines() {
        let mut tokens = line.split_whitespace();
        let mut token = match tokens.next() {
            Some(val) => val,
            None => continue,
        };
        loop {
            if token.width() > max_width && curr_line.is_empty() {
                // Token is longer than max_width; need to cut it
                let mut cut_index = 0;
                for index in token.grapheme_indices(true) {
                    let cut = token.split_at(index.0);
                    if cut.0.width() < max_width {
                        cut_index = index.0;
                    }
                    else {
                        break;
                    }
                }
                let cut = token.split_at(cut_index);
                curr_line.push_str(cut.0);
                token = cut.1;
            }
            else if curr_line.width() + token.width() + 1 > max_width {
                // Can't fit next token; need to wrap
                curr_line = curr_line.trim().to_string();
                rewrapped = rewrapped + &curr_line + "\n";
                curr_line.clear();
            }
            else {
                // Have room, add token
                curr_line = curr_line + token + " ";
                token = match tokens.next() {
                    Some(val) => val,
                    None => break,
                };
            }
        }
        curr_line = curr_line.trim().to_string();
        rewrapped = rewrapped + &curr_line + "\n";
        curr_line.clear();
    }
    rewrapped.pop();
    rewrapped
}

fn get_cols(msg: &str) -> usize
{
    let mut cols = 0;
    for line in msg.lines() {
        let width = line.width();
        if width > cols {
            cols = width;
        }
    }
    cols
}

pub fn slothsay(msg: &str)
{
    let wrapped = rewrap(msg, 40);
    let cols = get_cols(&wrapped);
    let mut fullmsg = String::from(SLOTH_ART);
    fullmsg = fullmsg + " +-" + &"-".repeat(cols) + "-+\n";
    for line in wrapped.lines() {
        fullmsg = fullmsg + " | " + line + &" ".repeat(cols - line.width()) + " |\n";
    }
    fullmsg = fullmsg + " +-" + &"-".repeat(cols) + "-+";
    println!("{}", fullmsg);
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match fs::read_to_string(&config.tasks_path) {
        Ok(contents) => {
            let tasks = parse_tasks(&contents);
            let task = pick_task(&tasks);
            slothsay(&task);
        },
        Err(_) => {
            let msg = String::from("I think the tasks file should be at ") + &config.tasks_path + &", but I couldn't open that file.";
            slothsay(&msg);
        }
    };
    Ok(())
}
