use std::borrow::Borrow;

#[derive(Debug)]
struct MyBox<T>(T);

impl Borrow<str> for MyBox<&str> {
    fn borrow(&self) -> &str {
        &self.0
    }
}

struct FakeHashMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> FakeHashMap<K, V> {
    fn new() -> Self {
        Self {
            keys: vec![],
            values: vec![],
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.keys.push(key);
        self.values.push(value);
    }

    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: std::cmp::Eq + ?Sized,
    {
        let mut found = None;

        for (index, item) in self.keys.iter().enumerate() {
            if item.borrow() == key {
                found = Some(index);
                break;
            }
        }

        if let Some(index) = found {
            return Some(&self.values[index]);
        }
        return None;
    }
}

fn main() {
    let mut fhm: FakeHashMap<MyBox<&str>, MyBox<&str>> = FakeHashMap::new();
    fhm.insert(MyBox("key"), MyBox("value"));
    println!("{:?}", fhm.get("key"));
}
