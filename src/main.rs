use std::{
    collections::HashMap,
    io::{self, Write},
};

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        clear_screen();

        println!("Employee Department Management System");
        println!("================ ... ================\n");

        println!("Type 'Add <name> to <department>' to add an employee to a department");
        println!("Type 'List <department>' to list all employees in a department");
        println!("Type 'All' to list all employees in all departments");
        println!("Type 'Exit' to exit the program");
        println!("================ ... ================");

        let mut option = String::new();
        print!("Enter option: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        println!("================ ... ================\n");

        if let Some(status) = handle_option(option, &mut company) {
            if status == 1 {
                break;
            }
        }

        println!("\nPress Enter to continue...");
        io::stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line");
    }
}

fn clear_screen() {
    print!("\u{1B}[H\u{1B}[2J");
}

fn handle_option(option: String, company: &mut HashMap<String, Vec<String>>) -> Option<u8> {
    match Command::from_input(option) {
        Some(Command::Add { name, department }) => {
            add_employee_to_department(name, department, company);
            None
        }
        Some(Command::List(department)) => {
            println!("{department}");
            None
        }
        Some(Command::All) => {
            println!("All");
            None
        }
        Some(Command::Exit) => {
            println!("Exiting...");
            Some(1)
        }
        None => None,
    }
}

enum Command {
    Add { name: String, department: String },
    List(String),
    All,
    Exit,
}

impl Command {
    fn from_input(option: String) -> Option<Self> {
        let parts: Vec<&str> = option.trim().split_whitespace().collect();

        match parts.as_slice() {
            ["Add", name, "to", department] => Some(Command::Add {
                name: name.to_string(),
                department: department.to_string(),
            }),
            ["List", department] => Some(Command::List(department.to_string())),
            ["All"] => Some(Command::All),
            ["Exit"] => Some(Command::Exit),
            _ => {
                println!("Invalid command!");
                None
            }
        }
    }
}

fn add_employee_to_department(
    employee_name: String,
    department: String,
    company: &mut HashMap<String, Vec<String>>,
) {
    // company.entry(department).or_insert(Vec::new()).push(employee_name);
}
