use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

const FILE_PATH: &str = "data.txt";
struct Person {
    name: String,
    age: u32,
}

fn write_person(person: &Vec<Person>) -> Result<(), std::io::Error> {
    let mut file = File::create(FILE_PATH)?;
    for person in person {
        writeln!(file, "Name: {}", person.name)?;
        writeln!(file, "Age: {}", person.age)?;
        writeln!(file)?;
    }
    Ok(())
}

fn read_person() -> Result<Vec<Person>, std::io::Error> {
    let file = File::open(FILE_PATH)?;
    let reader = BufReader::new(file);
    let mut persons = Vec::new();
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        if let Some(name) = line.strip_prefix("Name: ") {
            let mut age = String::new();
            if let Some(Ok(age_line)) = lines.next() {
                if let Some(age_str) = age_line.strip_prefix("Age: ") {
                    age = age_str.to_string();
                }
            }
            if let Ok(age) = age.parse::<u32>() {
                persons.push(Person::new(name.to_string(), age));
            }
        }
    }
    Ok(persons)
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn main() {
    let persons: Vec<Person> = vec![
        Person::new("Alice".to_string(), 25),
        Person::new("Bob".to_string(), 30),
    ];
    if let Err(e) = write_person(&persons) {
        eprintln!("Error writing to file: {}", e);
    }
    let persons = read_person();
    match persons {
        Ok(persons) => {
            for person in persons {
                println!("Name: {}", person.name);
                println!("Age: {}", person.age);
            }
        }
        Err(e) => eprintln!("Error reading from file: {}", e),
    }
}
