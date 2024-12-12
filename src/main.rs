use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::{fs, io};

//globals
const CSV_PATH: &str = "src/ToDoCommands.csv";
const JSON_PATH: &str = "src/todo.json";


#[derive(Serialize, Deserialize, Debug)]
struct TodoItem{
    name: String,
    is_completed: bool,
    notes: String,
    due_date: String
}

impl TodoItem{
    fn display(&self){
        println!("name: {}", self.name);
        println!("due_date: {}", self.due_date);
        match self.is_completed{
            true => println!("completed: yes"),
            false => println!("completed: no"),
        };
        println!("notes: {}", self.notes);
    }
}

#[derive(Deserialize, Debug)]
struct CustomCommand{
    letter: char,
    name: String,
    description: String,
}

fn main() {
    println!("Welcome to your todo list! \nEnter 'h' if you want to list commands!");

    let csv_data = load_commands_from_csv(CSV_PATH);
    let commands = match csv_data {
        Ok(vec) => vec,
        Err(e) => {
            eprintln!("Error loading csv data: {}", e);
            std::process::exit(1);
        }
    };

    let json_data = load_from_json(JSON_PATH);
    let mut todos = match json_data {
        Ok(vec) => vec,
        Err(e) => {
            eprint!("{:?}", e);
            eprintln!("Error loading todos");
            std::process::exit(1);
        },
    };

    loop {
        println!("\nEnter a command (type 'h' for help):");
        match get_input_char(&commands) {
            Ok(command) => match command {
                'a' => {
                    add_todo(&mut todos);
                }
                'd' => {
                    println!("Enter the name of the task to delete:");
                    let name = get_input_string();
                    delete_todo(&mut todos, &name);
                }
                'e' => {
                    println!("Enter the name of the task to edit:");
                    let name = get_input_string();
                    edit_todo(&mut todos, &name);
                }
                'h' => {
                    print_commands(&commands);
                }
                'l' => {
                    list_todos(&todos);
                }
                'q' => {
                    println!("Exiting the program. Goodbye!");
                    break;
                }
                _ => {
                    println!("Unknown command. Type 'h' to see the available commands.");
                }
            },
            Err(e) => println!("{}", e),
        }
    }

    // Save todos to the JSON file before exiting
    match save_to_json(&todos, JSON_PATH) {
        Ok(_) => println!("Tasks saved successfully."),
        Err(_) => eprintln!("Error saving tasks to JSON."),
    }
}

//used to save the todos and load the todos to the json file

fn save_to_json( todos: &Vec<TodoItem>, filename: &str ) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(todos)?;        //convert vec to a formatted string
    fs::write(filename, json)?;                      //wrtie the string to the file
    Ok(())                                                          //return status
}   

fn load_from_json( filename: &str ) -> Result<Vec<TodoItem>, Box<dyn Error>> {
    let data = fs::read_to_string(filename)?;         //read the file into a string
    let todos: Vec<TodoItem> = serde_json::from_str(&data)?;       //parse the data into the ToDoItem struct, into a vec
    Ok(todos)                                                     //return status
}

//managing commands from csv

fn load_commands_from_csv(filename: &str) -> Result<Vec<CustomCommand>, Box<dyn Error>>{
    let file = File::open(filename)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut commands = Vec::new();

    for result in rdr.deserialize() {
        let record: CustomCommand = result?;  // Deserialize each record into Command struct
        commands.push(record);
    }

    Ok(commands)
}

fn print_commands(commands: &Vec<CustomCommand>){
    for command in commands{
        println!("Command {}: {:<10} - {} ", command.letter, command.name, command.description);
    }
}

//managing input

//used for getting commands
fn get_input_char(commands: &Vec<CustomCommand>) -> Result<char, Box<dyn Error>> {
    let mut x: String = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    // Trimming input and converting to char
    let x = x.trim().chars().next().ok_or("No character entered")?;

    let chars = get_valid_commands_chars(commands);

    // Checking if the input is a valid commmand
    if chars.contains(&x) {
        Ok(x)
    } else {
        Err("Invalid command character".into())
    }
}

//all the valid commands in the csv file
fn get_valid_commands_chars(commands: &Vec<CustomCommand>) -> Vec<char>{
    let mut result: Vec<char> = Vec::new();
    for command in commands{
        result.push(command.letter);
    }
    result
}

// used for getting notes input, name, and date
fn get_input_string() -> String {
    /*
    string = "hello"
    char = 'f'
    string_actual = ['h', 'e', 'l' 'l', 'o']
     */
    let mut x: String = String::new(); // ""
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
    x.trim().to_string()
}

//marking if it's done or not done
fn get_input_bool() -> bool {
    let mut x: String = String::new();
    println!("Enter a 1 for done, and 0 for not done:");
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
    
    //converting input to bool
    match x.trim() {
        "1" => true,
        "0" => false,
        _ => {
            println!("Invalid input! Please put either 1 or 0.");
            x.clear();
            return get_input_bool();
        }
    }
}

// manipualting the todos

fn create_todo_item() -> TodoItem{
    /*
    get name
    get date
    get notes
    mark not completed
     */
    println!("Enter a name: ");
    let name: String = get_input_string();
    println!("Enter a data: ");
    let date = get_input_string();
    println!("Enter some notes: ");
    let notes = get_input_string();
    TodoItem {name: name, is_completed: false, notes: notes, due_date: date}
}

fn edit_todo(todos: &mut Vec<TodoItem>, name: &str){
    let mut index_to_delete: Option<usize> = None;
    for (index,todo) in todos.iter().enumerate(){
        if todo.name == name{
            index_to_delete = Some(index);
            break;
        }
    }

    if let Some(index) = index_to_delete{
        todos[index] = create_todo_item();
        println!("Enter if it's completed or not: ");
        todos[index].is_completed = get_input_bool();
    }
    else{
        println!("The todo does not exist!");
    }

    match save_to_json(&todos, JSON_PATH){
        Ok(_) => (),                                //if it goes right, do nothing
        Err(_) => {
            eprintln!("Error saving json file");   //else exit the program
            std::process::exit(1);
        },
    };
}

fn add_todo(todos: &mut Vec<TodoItem>){
    //The &* makes it an immutable reference
    let item = create_todo_item();
    for todo in &*todos{
        if item.name == todo.name{
            println!("This todo already exists!");
            return;
        }
    }
    //then access the mutable reference
    todos.push(item);
}

fn delete_todo( todos: &mut Vec<TodoItem>, name: &str ) {
    let mut index_to_delete: Option<usize> = None;
    for (index,todo) in todos.iter().enumerate(){
        if todo.name == name{
            index_to_delete = Some(index);
            break;
        }
    }

    if let Some(index) = index_to_delete{
        todos.remove(index);
    }
    else{
        println!("The todo does not exist");
    }
}

fn list_todos(todos: &Vec<TodoItem>){
    for todo in todos{
        todo.display();
        println!("");
    }
}
