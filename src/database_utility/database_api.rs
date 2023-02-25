use std::{fs, io::{Write, self}, str::FromStr, fmt::Debug, error::{self, Error}};
use rusqlite::{Connection, Result};
use thiserror::Error;

const ERROR_STATEMENT: &str = "Something went wrong. enter \"help\" to see available commands";
const HELP_STATEMENT: [&str; 6] = ["MessageCounts - Displays the number of messages sent by each person",
    "Participants - lists the participants of the conversation by name",
    "StrictSearch - returns messages matching a search criteria, format: StrictSearch {\"your query\"}",
    "CurrentDatabases - searches the currect directory for databases that have already been created",
    "OpenDatabase - opens an extant database. Format: OpenDatabase {database name here}",
    "help - Displays options"];

#[derive(Debug, Error)]
enum ParseCommandError {
    #[error("No command matching user input")]
    NoMatchingCommand,
}

enum Command {
    MessageCounts, 
    Participants, 
    StrictSearch, 
    Help, 
    CurrentDatabases, 
    OpenDatabase,
}
impl FromStr for Command {
    type Err = ParseCommandError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "MessageCounts" | "messagecounts" => Ok(Self::MessageCounts), 
            "Participants" | "participants" => Ok(Self::Participants), 
            "StrictSearch" | "strictsearch" => Ok(Self::StrictSearch), 
            "Help" | "help" => Ok(Self::Help), 
            "CurrentDatabases" | "currentdatabases"=> Ok(Self::CurrentDatabases), 
            "OpenDatabase" | "opendatabases"=> Ok(Self::OpenDatabase),
            _ => Err(ParseCommandError::NoMatchingCommand)
        }
    }
}

struct CommandArgumentPair {
    command: Command,
    argument: Option<String>,
}

pub fn command_line_control_loop() {
    let mut user_input: Vec<String>;
    let mut args: Vec<String>;
    let mut order: CommandArgumentPair;
    let mut db: Option<&str> = None;
    loop {
        print!(">");
        io::stdout().flush().expect("IO issue, couldn't flush output Stream");
        order = match get_user_input(){
            Ok(com) => com,
            Err(_) => continue,
        };
    }
}

fn input_split(input: String) -> Vec<String>{
    input.split(" ").map(|x| x.to_string()).collect::<Vec<String>>()
}

fn get_user_input() -> Result<CommandArgumentPair, Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input_split(input.trim().to_string());
    let command = Command::from_str(input[0].as_str())?;

    Ok(CommandArgumentPair { command: Command::Help, argument: Some("test".to_string()) })
}   


pub fn get_participants(database: &str) -> Result<Option<Vec<String>>, rusqlite::Error>{
    let conn = Connection::open(database).expect("Failed to open database");
    let mut stmt = conn.prepare("SELECT sender FROM messages GROUP BY sender")?;

    let rows = stmt.query_map([], |row| row.get(0))?;
    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }
    match names.len() {
        0 => Ok(None),
        _ => Ok(Some(names)),
    }
}

pub fn message_counts(database: &str) -> Result<Option<Vec<(String, i32)>>, rusqlite::Error> {
    let conn = Connection::open(database).expect("Failed to open database");
    let mut stmt = conn.prepare("SELECT sender, count(*) FROM messages GROUP BY sender")?;
    let mut name_and_message_count: Vec<(String, i32)> = Vec::new();

    let rows = stmt.query_map([], |row| {
        Ok((row.get::<usize, String>(0).unwrap(), row.get::<usize, i32>(1).unwrap()))
    })?;

    for stats in rows {
        match stats {
            Ok(tpl) => name_and_message_count.push(tpl),
            Err(error) => println!("Error: {}", error),
        }
    }

    match name_and_message_count.len() {
        0 => Ok(None),
        _ => {
            let total = name_and_message_count.iter().map(|x| x.1).sum();
            name_and_message_count.push(("Total".to_string(), total));
            Ok(Some(name_and_message_count))
        }
    }
}

pub fn strict_search(database: &str, query: &str) -> Result<Option<Vec<String>>, rusqlite::Error> {
    let conn = Connection::open(database).expect("Failed to open database");
    let mut stmt = conn.prepare("SELECT content FROM messages WHERE content LIKE '%:query%'")?;
    let mut query_results: Vec<String> = Vec::new();
    let rows = stmt.query_map(&[(":query", query)], |row| {
        row.get(0)
    })?;

    for result in rows {
        match result {
            Ok(res) => query_results.push(res),
            Err(error) => println!("Error {:?}", error),
        }
    }
    match query_results.len(){
        0 => Ok(None),
        _ => Ok(Some(query_results))
    }
}


// This is nightmare code, turns out dealing with directories is harder than one would expect. 
fn current_databases() -> Option<Vec<String>>{
    let entries = match fs::read_dir("") {
        Ok(directory) => directory.filter_map(|x| {
            match x {
                Ok(entry) => Some(entry),
                Err(err) => panic!("Error with particular DirEntry: {}", err),
            }
        }).filter(|x| x.path().extension().is_some())
        .filter(|x| x.path().extension().unwrap().to_str().unwrap() == "db")
        .map(|x| x.file_name().to_str().unwrap().to_owned()).collect::<Vec<String>>(),
        Err(err) => {
            panic!("Error reading directory: {}", err)},
    };

    Some(entries)
}

fn print_help() {
    for command in HELP_STATEMENT {
        println!("{}", command);
    }
}

fn print_results<T:std::iter::Iterator>(query_results: Option<T>) where <T as Iterator>::Item:Debug{
    match query_results {
        Some(results) => for result in results {println!("{:?}", result);},
        None => println!("{:?}", "No results found in current directory"),
    }
}