use cli_table::{print_stdout, Table, WithTitle};
use std::collections::HashMap;
use std::io;
use std::io::Write;

// Create a program that takes in a search string and filters this dataset and print by tabular format.
// Dataset:
// First Name Last Name Position Separation date
// John Johnson Manager 2016-12-31
// Tou Xiong Software Engineer 2016-10-05
// Michaela Michaelson District Manager 2015-12-19
// Jake Jacobson Programmer
// Jacquelyn Jackson DBA
// Sally Weber Web Developer 2015-12-18

// Inputs: search string
// Process: filter in firstname or lastname
// outputs: search results

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

#[derive(Table)]
struct Employee {
    #[table(title = "First Name")]
    first_name: String,
    #[table(title = "Last Name")]
    last_name: String,
    #[table(title = "Position")]
    position: String,
    #[table(title = "Separation Date")]
    separation_date: String,
}

fn main() {
    let dataset: Vec<HashMap<&str, &str>> = vec![
        HashMap::from([
            ("first_name", "John"),
            ("last_name", "Johnson"),
            ("position", "Manager"),
            ("separation_date", "2016-12-31"),
        ]),
        HashMap::from([
            ("first_name", "Tou"),
            ("last_name", "Xiong"),
            ("position", "Software Engineer"),
            ("separation_date", "2016-10-05"),
        ]),
        HashMap::from([
            ("first_name", "Michaela"),
            ("last_name", "Michaelson"),
            ("position", "District Manager"),
            ("separation_date", "2015-12-19"),
        ]),
        HashMap::from([
            ("first_name", "Jake"),
            ("last_name", "Jacobson"),
            ("position", "Programmer"),
            ("separation_date", ""),
        ]),
        HashMap::from([
            ("first_name", "Jacquelyn"),
            ("last_name", "Jackson"),
            ("position", "DBA"),
            ("separation_date", ""),
        ]),
        HashMap::from([
            ("first_name", "Sally"),
            ("last_name", "Weber"),
            ("position", "Web Developer"),
            ("separation_date", "2015-12-18"),
        ]),
    ];

    // Get search_query, "Enter a search string:"
    let search_query: String = get_input("Enter a search string: ");
    // Search in firstname or lastname
    // store result in vec of hashmap
    let result: Vec<HashMap<&str, &str>> = dataset
        .into_iter()
        .filter(|employee| {
            employee.get("first_name").unwrap().contains(&search_query)
                || employee.get("&last_name").unwrap().contains(&search_query)
        })
        .collect();

    // convert to vec of struct table
    let employees_table: Vec<Employee> = result
        .into_iter()
        .map(|mut employee| Employee {
            first_name: employee.remove("first_name").unwrap().to_string(),
            last_name: employee.remove("last_name").unwrap().to_string(),
            position: employee.remove("position").unwrap().to_string(),
            separation_date: employee.remove("separation_date").unwrap().to_string(),
        })
        .collect();
    // show table
    println!("Hello, world!");
}
