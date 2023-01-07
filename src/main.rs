use std::collections::HashMap;
use std::io;

fn main() {
    let mut employees_map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Invalid name");

        let input: Vec<&str> = input.split_whitespace().collect();

        let command: Option<&&str> = input.get(0);

        match command {
            Some(&"Add") => {
                let employee_name: &str = input[1];
                let employee_department: &str = input[3];

                let department: Option<&mut Vec<String>> = employees_map.get_mut(employee_department);

                match department {
                    Some(department_array) => department_array.push(String::from(employee_name)),
                    None => { employees_map.insert(String::from(employee_department), vec!(String::from(employee_name))); }
                }

                println!("Employee added to {}", employee_department);

                continue
            }
            Some(&"List") => {
                let department = input[1];

                let department_array = employees_map.get_mut(department);

                match department_array {
                    Some(x) => {
                        x.sort();
                        println!("Employees in this department: {:?}", x)
                    },
                    None => {
                        println!("None in {}", department);
                    }
                }

                continue
            }
            _ => continue
        }
    }
}
