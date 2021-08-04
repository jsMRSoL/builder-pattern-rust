enum Item {
    VegBurger,
    ChickenBurger,
    Pepsi,
    Coke,
}

enum Packing {
    Bottle,
    Wrapper,
}

impl Packing {
    fn pack(&self) -> String {
	match self {
	    Packing::Bottle => {return "Bottle".into()},
	    Packing::Wrapper => {return "Wrapper".into()}
	}
    }
}

impl Item {
    fn get_packing(&self) -> Packing {
	match &self {
	    Item::ChickenBurger | Item::VegBurger => {return Packing::Wrapper},
	    Item::Pepsi | Item::Coke => {return Packing::Bottle},
	}
    }
    fn get_name(&self) -> String {
	match self {
	    Item::ChickenBurger => {return "Chicken Burger".into()},
	    Item::VegBurger => {return "Veggie Burger".into()},
	    Item::Pepsi => {return "Pepsi".into()},
	    Item::Coke => {return "Coke".into()},
	}
    }
    fn get_price(&self) -> f32 {
	match self {
	    Item::ChickenBurger => {return 50.5},
	    Item::VegBurger => {return 25.0}
	    Item::Pepsi => {return 35.0},
	    Item::Coke => {return 30.0},
	}
    }
}

struct Meal {
    list: Vec<Item>,
}

impl Meal {
    fn new() -> Self {
	Self {
	    list: Vec::new(),
	}
    }
    fn add_item(&mut self, item: Item) {
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
	meal.add_item(Item::VegBurger);
	meal.add_item(Item::Coke);
	meal
    }
    fn prepare_meat_meal() -> Meal {
	let mut meal = Meal::new();
	meal.add_item(Item::ChickenBurger);
	meal.add_item(Item::Pepsi);
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
