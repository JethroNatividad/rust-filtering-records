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
// Process: filter in every fields
// outputs: search results

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
    println!("Hello, world!");
}
