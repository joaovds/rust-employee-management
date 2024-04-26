use std::{
    collections::HashMap,
    io::{self, Write},
};

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        clear_screen();

        println!(
            "{}",
            Colors::Cyan.print("Employee Department Management System")
        );
        println!(
            "{}",
            Colors::Cyan.print("=====================================")
        );

        println!("{}", Colors::Blue.print("Type \x1B[3m'Add <name> to <department>'\x1B[23m to add an employee to a department"));
        println!(
            "{}",
            Colors::Blue.print(
                "Type \x1B[3m'List <department>'\x1B[23m to list all employees in a department"
            )
        );
        println!(
            "{}",
            Colors::Blue
                .print("Type \x1B[3m'All'\x1B[23m to list all employees in all departments")
        );
        println!(
            "{}",
            Colors::Blue.print("Type \x1B[3m'Exit'\x1B[23m to exit the program")
        );
        println!(
            "{}",
            Colors::Cyan.print("================ ... ================")
        );

        let mut option = String::new();
        print!("{}", Colors::Cyan.print("Enter option: "));
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        println!(
            "{}",
            Colors::Cyan.print("================ ... ================\n")
        );

        if let Some(status) = handle_option(option, &mut company) {
            if status == 1 {
                break;
            }
        }

        println!("{}", Colors::Green.print("\nPress Enter to continue..."));
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
            add_employee_to_department(&name, &department, company);
            None
        }
        Some(Command::List(department)) => {
            list_employees_by_department(&department, company);
            None
        }
        Some(Command::All) => {
            list_all_departments_with_employees(company);
            None
        }
        Some(Command::Exit) => {
            println!("{}", Colors::Red.print("Exiting...\n"));
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
                println!("{}", Colors::Red.print("Invalid command!"));
                None
            }
        }
    }
}

fn add_employee_to_department(
    employee_name: &String,
    department: &String,
    company: &mut HashMap<String, Vec<String>>,
) {
    company
        .entry(department.to_string())
        .or_insert(Vec::new())
        .push(employee_name.to_string());
    println!(
        "{}",
        Colors::Green.print(&format!(
            "Added \x1B[1m{}\x1B[22m to \x1B[1m{}\x1B[22m department successfully!!!",
            employee_name, department
        ))
    );
}

fn list_all_departments_with_employees(company: &mut HashMap<String, Vec<String>>) {
    let mut company: Vec<_> = company.into_iter().collect();
    company.sort();

    for (department, employees) in company.iter() {
        println!(
            "{}",
            Colors::Magenta.print(&format!(
                "Employees in the \x1B[1m{}\x1B[22m department:",
                department
            ))
        );
        sort_and_print_employees(employees.to_vec());
        println!();
    }
}

fn list_employees_by_department(department: &String, company: &mut HashMap<String, Vec<String>>) {
    if let Some(department_employees) = company.get(department) {
        println!(
            "{}",
            Colors::Magenta.print(&format!(
                "Employees in the \x1B[1m{}\x1B[22m department:",
                department
            ))
        );
        sort_and_print_employees(department_employees.to_vec());
    } else {
        println!(
            "{}",
            Colors::Red.print(&format!(
                "No employees found in the \x1B[1m{}\x1B[22m department",
                department
            ))
        );
    }
}

fn sort_and_print_employees(employees: Vec<String>) {
    let mut names: Vec<String> = employees.iter().map(|name| name.to_string()).collect();
    names.sort();

    for name in names {
        println!("{}", Colors::Yellow.print(&format!("{}", name)));
    }
}

enum Colors {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
}

impl Colors {
    fn print(&self, text: &str) -> String {
        match self {
            Colors::Red => format!("\x1b[31m{}\x1b[0m", text),
            Colors::Green => format!("\x1b[32m{}\x1b[0m", text),
            Colors::Yellow => format!("\x1b[33m{}\x1b[0m", text),
            Colors::Blue => format!("\x1b[34m{}\x1b[0m", text),
            Colors::Magenta => format!("\x1b[35m{}\x1b[0m", text),
            Colors::Cyan => format!("\x1b[36m{}\x1b[0m", text),
        }
    }
}
