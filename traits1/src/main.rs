use std::any::Any;

trait Animal: std::fmt::Debug + Any {
    fn make_noise(&self) {
        println!("Who knows what noise I make?");
    }

    fn as_any(&self) -> &dyn Any;
}

trait Tame {}

#[derive(Debug)]
struct Cat { noise: String }

#[derive(Debug)]
struct Tortoise;

impl Animal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Tame for Cat {}

impl Animal for Cat {
    fn make_noise(&self) {
        println!("{}", self.noise);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn pet<A: Animal + Tame>(animal: A) {
    animal.make_noise();
}

fn main() {
    let cat = Cat{ noise: "Mrow".to_string() };
    let tortoise = Tortoise{};

    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(cat));
    animals.push(Box::new(tortoise));
    animals.iter().for_each(|a| println!("{a:?}"));
    for animal in animals.iter() {
        if let Some(cat) = animal.as_any().downcast_ref::<Cat>() {
            println!("Now we have the cat as a Cat");
        }
    }
}
