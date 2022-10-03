use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                 vec!["many madrigals".to_string(),
                      "Tenebrae Responsiria".to_string()]);
    table.insert("Caravaggio".to_string(),
                 vec!["The Musicians".to_string(),
                      "The Calling of St.Matthew".to_string()]);
    show(table)
}

fn show(table: Table) {
    for(artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}