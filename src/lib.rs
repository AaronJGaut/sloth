use std::env;
use std::fs;
use std::error::Error;

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

struct Dim {
    rows: usize,
    cols: usize,
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
            if token.chars().count() > max_width && curr_line.is_empty() {
                // Token is longer than max_width; need to cut it
                let mut cut_index = 0;
                for index in token.char_indices() {
                    let index = index.0;
                    if index >= max_width {
                        cut_index = index;
                        break;
                    }
                }
                let cut = token.split_at(cut_index);
                curr_line.push_str(cut.0);
                token = cut.1;
            }
            else if curr_line.chars().count() + token.chars().count() + 1 > max_width {
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

fn get_dims(msg: &str) -> Dim
{
    let rows = msg.lines().count();
    let mut cols = 0;
    for line in msg.lines() {
        // I should probably be counting grapheme clusters
        if line.chars().count() > cols {
            cols = line.chars().count();
        }
    }
    Dim { rows, cols }
}

fn slothsay(msg: &str)
{
    let wrapped = rewrap(msg, 40);
    let dim = get_dims(&wrapped);
    let mut fullmsg = String::from(SLOTH_ART);
    fullmsg = fullmsg + " +-" + &"-".repeat(dim.cols) + "-+\n";
    for line in wrapped.lines() {
        fullmsg = fullmsg + " | " + line + &" ".repeat(dim.cols - line.chars().count()) + " |\n";
    }
    fullmsg = fullmsg + " +-" + &"-".repeat(dim.cols) + "-+";
    println!("{}", fullmsg);
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.tasks_path)?;
    let tasks = parse_tasks(&contents);
    let task = pick_task(&tasks);
    slothsay(&task);

    Ok(())
}
