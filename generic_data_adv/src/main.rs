use std::{collections::HashMap, hash::Hash, fmt::Debug};

#[derive(Debug)]
struct HashSetData<KEY, VALUE>
where KEY: Eq + Hash + Debug, VALUE: Debug 
{
    data: HashMap<KEY, Vec<VALUE>>
}

impl <KEY, VALUE> HashSetData<KEY, VALUE> 
where KEY: Eq + Hash + Debug, VALUE: Debug 
{
    fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    fn add(&mut self, key: KEY, value: VALUE) {
        if let Some(entry) = self.data.get_mut(&key) {
            entry.push(value);
        } else {
            self.data.insert(key, vec![value]);
        }
    }
}

fn main() {
    let mut readings = HashSetData::<usize, i32>::new();
    readings.add(1, -2);
    readings.add(1, 3);
    readings.add(1, 5);
    readings.add(2, 1);
    println!("{readings:?}");
}
