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

use std::collections::HashMap;

struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}

impl Todo {
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