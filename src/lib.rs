#[derive(Debug)]
pub struct Human {
    name: String,
    age: u8,
}

impl Human {
    fn new() -> Self {
        Self {
            name: random_name(),
            age: 0,
        }
    }
}

fn random_name() -> String {
    "Aleph".into()
}

#[derive(Debug)]
pub struct World {
    age: u64,
    humans: Vec<Human>,
}

impl World {
    pub fn new() -> Self {
        println!("world: loading humans...");
        Self {
            age: 0,
            humans: vec![Human::new(), Human::new()],
        }
    }

    pub fn stats(&self) -> String {
        format!("age: {}, #humans: {}", self.age, (&self.humans).into_iter().count())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
