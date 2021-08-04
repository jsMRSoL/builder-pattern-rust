// create interfaces for Item and Packing
trait Item {
    fn get_packing(&self) -> Box<dyn Packing>;
    fn get_name(&self) -> String;
    fn get_price(&self) -> f32;
}

trait Packing {
    fn pack(&self) -> String;
}


// create concrete classes implementing the Packing interface
struct Wrapper;
impl Packing for Wrapper {
    fn pack(&self) -> String {
	"Wrapper".into()
    }
}

struct Bottle;
impl Packing for Bottle {
    fn pack(&self) -> String {
	"Bottle".into()
    }
}


// Create concrete classes for food items
struct VegBurger;
impl VegBurger {
}
impl Item for VegBurger {
    fn get_packing(&self) -> Box<dyn Packing> {
	Box::new(Wrapper)
    }
    fn get_name(&self) -> String {
	"Veg Burger".into()
    }
    fn get_price(&self) -> f32 {
	25.0
    }
}

struct ChickenBurger;
impl ChickenBurger {
}
impl Item for ChickenBurger {
    fn get_packing(&self) -> Box<dyn Packing> {
	Box::new(Wrapper)
    }
    fn get_name(&self) -> String {
	"Chicken Burger".into()
    }
    fn get_price(&self) -> f32 {
	50.5
    }
}

struct Pepsi;
impl Pepsi {
}
impl Item for Pepsi {
    fn get_packing(&self) -> Box<dyn Packing> {
	Box::new(Bottle)
    }
    fn get_name(&self) -> String {
	"Pepsi".into()
    }
    fn get_price(&self) -> f32 {
	35.0
    }
}

struct Coke;
impl Coke {
}
impl Item for Coke {
    fn get_packing(&self) -> Box<dyn Packing> {
	Box::new(Bottle)
    }
    fn get_name(&self) -> String {
	"Coke".into()
    }
    fn get_price(&self) -> f32 {
	30.0
    }
}

struct Meal {
    list: Vec<Box<dyn Item>>,
}

impl Meal {
    fn new() -> Self {
	Self {
	    list: Vec::new(),
	}
    }
    fn add_item(&mut self, item: Box<dyn Item>) {
	self.list.push(item);
    }
    fn get_cost(&self) -> f32 {
	let mut cost: f32 = 0.0;
	for item in self.list.iter() {
	    cost += item.get_price();
	}
	cost
    }
    fn show_items(&self) {
	for item in self.list.iter() {
	    println!("Item: {}", item.get_name());
	    println!("Packing: {}", item.get_packing().pack());
	    println!("Price: {}", item.get_price());
	}
	
    }
}

struct MealBuilder;
impl MealBuilder {
    fn prepare_veg_meal() -> Meal {
	let mut meal = Meal::new();
	meal.add_item(Box::new(VegBurger));
	meal.add_item(Box::new(Coke));
	meal
    }
    fn prepare_meat_meal() -> Meal {
	let mut meal = Meal::new();
	meal.add_item(Box::new(ChickenBurger));
	meal.add_item(Box::new(Pepsi));
	meal
    }
}

fn main() {

    let veg_meal = MealBuilder::prepare_veg_meal();
    println!("Vegetarian Meal");
    veg_meal.show_items();
    println!("Total cost: {}", veg_meal.get_cost());

    let meat_meal = MealBuilder::prepare_meat_meal();
    println!("Meaty Meal");
    meat_meal.show_items();
    println!("Total cost: {}", meat_meal.get_cost());
    
}
