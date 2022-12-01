fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?}, {:?}", action, item);

    // instantiate a struct, binding it as mutable.
    let mut todo = Todo {
        map:HashMap::new(),
    };

    if action == "add" {
        todo.insert(item);
        // match the Result returned from the save function and print a message on screen for both cases.
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }
}

use std::io::Read;
use std::collections::HashMap;
use std::str::FromStr;

struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}

impl Todo {
    // return Todo struct or io:Error
    fn new() -> Result<Todo, std::io::Error> {
        // see https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
        // create(true) will create the file if it's not already present
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();

        // reads all the bytes in the file
        f.read_to_string(&mut content)?;

        // binding a map variable map: HashMap
        // will split our lines
        // collect::<Vec<&str>>()  as de scribed in the documentation
        // map(|v|(v[0], v[1])) transform it into a tuple for convenience
        // map(|(k,v)| (String::from(k), bool::from_str(v).unwrap()) convert the two elements of the tuple into a String
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        Ok(Todo {map})
    }

    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }
    // -> annotates the returned type from the function. returning a Result.
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        // iterate over the map, and format each string, separating key and value with a tab character and each line with a new line.
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            // push the formatted string into a content variable.
            content.push_str(&record)
        }
        // write content inside a file called db.txt.
        std::fs::write("db.txt", content)
    }
}