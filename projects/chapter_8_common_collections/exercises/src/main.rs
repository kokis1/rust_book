use std::collections::HashMap;
use std::io;

fn task_1(integer_vec: Option<Vec<i32>>) {
// task: find the median and mode of a list of integers

// optionally input a vector to the function
    let mut integer_vec = match integer_vec {
        None => vec!(10, 34, 1, 5, 78, 1, 5, 7, 10, 10),
        Some(vec) => vec,
    };

    integer_vec.sort(); // sorts into ascending order

    // uses matching to get the median based off the parity of the vector's length
    let median = match integer_vec.len() % 2 {
        0 => {
            let index = integer_vec.len() / 2;
            integer_vec[index]
        }
        _ => {
            let index = (integer_vec.len() + 1) / 2;
            integer_vec[index]
        }
    };

    let mut frequency_map = HashMap::new();

    // either incrementing the frequency or inserting a count of 0
    for int in &integer_vec {
        let count: &mut i32 = frequency_map.entry(int).or_insert(0);
        *count += 1;
    }

    let mut most_keyed = integer_vec.get(0).unwrap();
    for key_ref in frequency_map.keys() {
        let freq1: &i32 = frequency_map.get(key_ref).unwrap();
        let freq2: &i32 = frequency_map.get(&most_keyed).unwrap();
        if freq1 > freq2 {
            most_keyed = *key_ref;
        }
    }

    println!("The numbers are {integer_vec:?}.");
    println!("The frequency map of the numbers is {frequency_map:?}.");
    println!("The median is {median} and the mode is {most_keyed}");
}

fn task_2(s: Option<String>) {
    // converting strings to pig latin
    let s = match s {
        None => String::from("This sentence is a String!!"),
        Some(stringy_string) => stringy_string,
    };

    let vowel_list: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];

    // making an empty string to add the updated words to
    let mut pig_latin_str = String::new();

    for word in s.split_whitespace() {
        for c in word.to_lowercase().chars() {
            let mut new_word = String::from(word);

            if !c.is_alphabetic() {continue;}

            let suffix = if vowel_list.contains(&c) {
                String::from("-hay ")
            } else {
                new_word.remove(0);
                format!("-{c}ay ")
            };
            // if the word starts with a constonant push to the end of word and add -ay
            new_word.push_str(&suffix);
            pig_latin_str.push_str(&new_word);
            break;
        }
    }
    println!("{s} in pig latin is {pig_latin_str}");
}

enum Instruction {
    Quit,
    AddEmployee{department: String, name: String},
    List{ by_department: bool},
    Unrecognised,
}
impl Instruction {
    fn parse_input(input: String) -> Self {

        let mut input: Vec<&str> = input.split_whitespace().collect();
        input.reverse();

        match input.pop().unwrap() {
            "quit" => return Instruction::Quit,
            "add" => {
                if input.len() != 2 {return Instruction::Unrecognised;}
                return Instruction::AddEmployee {
                    department: String::from(input.pop().unwrap()),
                    name: String::from(input.pop().unwrap()),
                }
            }
            "list" => {
                if input.len() != 1 {return Instruction::Unrecognised;}
                match input.pop().unwrap() {
                    "department" => return Instruction::List {by_department: true},
                    "all" => return Instruction::List {by_department: false},
                    _ => {
                        println!("Unrecognised Instruction!");
                        return Instruction::Unrecognised;
                    }
                }
            },
            _ => {
                println!("Unrecognised Instruction!");
                return Instruction::Unrecognised;
            },
        }

    }
}

fn task_3() {
    // make a text interface where a user can input employees by department
    // then allow them to retrieve a list back sorted alphabetically by department
    let mut employee_hashmap: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!(">>");

        // takes user input from stdin
        let mut user_input = String::new();
        let _ = io::stdin().read_line(&mut user_input);
        let instruction = Instruction::parse_input(user_input);

        match instruction { // performs an instruction based on what the input was
            Instruction::Quit => {
                println!("Exiting the database!");
                return;
            },
            Instruction::AddEmployee{department, name} => {
                println!("Adding {name} to the {department} department");
                match employee_hashmap.get_mut(&department){
                    Some(i) => i.push(name),
                    None => {employee_hashmap.insert(department.clone(), vec!(name));},
                };
            },
            Instruction::List{by_department: true} => {
                for key in employee_hashmap.keys() {
                    println!("{key}:");
                    let mut employee_vector: Vec<String> = employee_hashmap.get(key).cloned().unwrap();
                    employee_vector.sort(); // sorts the employees alphabetically
                    println!("{employee_vector:?}");
                }
            },
            Instruction::List{by_department: false} => {
                let mut employee_vector: Vec<String> = Vec::from_iter(employee_hashmap.values().cloned()).concat();
                employee_vector.sort();
                println!("{employee_vector:#?}");
            },
            _ => continue,
        }
    }
}

fn main() {
    task_1(None);
    task_2(Some(String::from("hello my name is Reuben")));
    task_3();
    
}