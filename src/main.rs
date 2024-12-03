use std::{collections::HashMap, thread::sleep, time::Duration};

fn main() {
    let sn = Duration::from_millis(1000);
    loop {
        sleep(sn);
        let mut map = HashMap::new();
        map.insert("Bob", 25);
        map.insert("Alice", 30);
        map.insert("Alice", 30);
        map.insert("Charlie", 30);

        let target_value = 30;


            // Iterate to find the key
        let key = map.iter().find_map(|(k, &v)| if v == target_value { Some(k) } else { None }).filter(|&&k| {k == "Alice"});        
        match key {
            Some(k) => println!("Found key: {}", k),
            None => println!("Value not found"),
        }
    }
}

