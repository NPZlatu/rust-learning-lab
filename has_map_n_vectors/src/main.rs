
/**
 * Using a hash map and vectors, create a text interface to allow a user to add employee names
 * to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
 * Then let the user retrieve a list of all people in a department or all people in the company
 * by department, sorted alphabetically.
 */

// Import necessary modules

use std::io;
use std::collections::HashMap;

// Define the Department enum
#[derive(Debug, PartialEq, Eq, Hash, Copy)]
enum Department {
    Engineering,
    Sales,
}

// Define the Action enum
#[derive(Debug)]
enum Action {
    AddEmployeeToDepartment,
    RetrieveEmployeeFromDepartment,
    RetrieveAllEmployees,
}

// Function to display the menu options and read user input
fn input_action() -> Action {
    println!("Choose an Option");
    println!("1. Add Employee to Department");
    println!("2. Retrieve Employee by Department");
    println!("3. Retrieve All Employees");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    match choice.trim().parse() {
        Ok(1) => Action::AddEmployeeToDepartment,
        Ok(2) => Action::RetrieveEmployeeFromDepartment,
        Ok(3) => Action::RetrieveAllEmployees,
        _ => panic!("Invalid Choice, Please enter a number between 1 and 3"),
    }
}

// Function to add an employee to a department
fn add_employee_to_department(office: &mut HashMap<Department, Vec<String>>) {
    let department = input_department();
    let employee_name = input_employee_name();

    office.entry(department).or_insert_with(Vec::new).push(employee_name);

    display_employees_in_alphabetic_order(office, Some(department));
}

// Function to retrieve employees by department
fn retrieve_employee_by_department(office: &HashMap<Department, Vec<String>>) {
    let department = input_department();
    if let Some(_) = office.get(&department) {
        display_employees_in_alphabetic_order(office, Some(department));
    } else {
        println!("No employees found in {:?}", department);
    }
}

// Function to retrieve all employees
fn retrieve_all_employees(office: &HashMap<Department, Vec<String>>) {
    display_employees_in_alphabetic_order(office, None);
}

// Function to input department from the user
fn input_department() -> Department {
    println!("Choose department");
    println!("1. Engineering");
    println!("2. Sales");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    match choice.trim().parse() {
        Ok(1) => Department::Engineering,
        Ok(2) => Department::Sales,
        _ => panic!("Invalid input. Please enter 1 for Engineering or 2 for Sales."),
    }
}

// Function to input employee name from the user
fn input_employee_name() -> String {
    println!("Enter Employee Name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_string()
}

// Function to display employees in alphabetical order
fn display_employees_in_alphabetic_order(
    office: &HashMap<Department, Vec<String>>,
    department: Option<Department>
) {
    let mut all_employees: Vec<&String> = Vec::new();

    match department {
        Some(d) => {
            if let Some(employees) = office.get(&d) {
                all_employees.extend(employees);
            } else {
                println!("No employees found in {:?}", d);
                return;
            }
        }
        None => {
            for employees in office.values() {
                all_employees.extend(employees);
            }
        }
    }

    all_employees.sort();
    println!("Employees: {:#?}", all_employees);
}

// Function to initialize the office with predefined values
fn initialize_office_with_predefined_values(office: &mut HashMap<Department, Vec<String>>) {
    let employees_engineering = vec!["Niraj Paudel".to_string(), "Anusha Wagle".to_string()];
    let employees_sales = vec![
        "Suraj Paudel".to_string(),
        "Pratikshya Thapa".to_string(),
        "Raya Paudel".to_string()
    ];

    office.insert(Department::Engineering, employees_engineering);
    office.insert(Department::Sales, employees_sales);
}

// Main function to run the program
fn main() {
    let mut office: HashMap<Department, Vec<String>> = HashMap::new();
    initialize_office_with_predefined_values(&mut office);

    let action: Action = input_action();

    match action {
        Action::AddEmployeeToDepartment => add_employee_to_department(&mut office),
        Action::RetrieveEmployeeFromDepartment => retrieve_employee_by_department(&office),
        Action::RetrieveAllEmployees => retrieve_all_employees(&office),
    }
}
