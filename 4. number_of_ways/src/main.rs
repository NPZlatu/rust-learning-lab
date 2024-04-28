/*
Given a set of stairs and a frog positioned at the bottom step (0th), the frog aims to ascend to the top step ((N-1)th) of the staircase. 
The frog can climb either one or two steps at a time. Additionally, a height array of length N is provided, denoting the height of each step. 
How many distinct ways can the frog ascend from one step to another, considering the height array?
*/

use std::collections::HashMap;

fn main() {
    let mut cache: HashMap<u8, u8> = HashMap::new();
    println!("{}", num_of_ways(6, &mut cache));
}

fn num_of_ways(height: u8, cache: &mut HashMap<u8, u8>) -> u8 {
    if height < 3 {
        height
    } else {
        if let Some(value) = cache.get(&height) {
            *value
        } else {
            let value1 = num_of_ways(height - 1, cache);
            let value2 = num_of_ways(height - 2, cache);
            let value = value1 + value2;
            cache.insert(height, value);
            value
        }
    }
}
