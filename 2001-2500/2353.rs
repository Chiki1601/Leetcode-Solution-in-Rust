use std::collections::HashMap;

struct FoodRatings {
    food_to_cuisine: HashMap<String, String>,
    food_to_rating: HashMap<String, i32>,
    cuisine_foods: HashMap<String, Vec<String>>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_to_cuisine = HashMap::new();
        let mut food_to_rating = HashMap::new();
        let mut cuisine_foods: HashMap<String, Vec<String>> = HashMap::new();

        for i in 0..foods.len() {
            food_to_cuisine.insert(foods[i].clone(), cuisines[i].clone());
            food_to_rating.insert(foods[i].clone(), ratings[i]);
            cuisine_foods.entry(cuisines[i].clone()).or_default().push(foods[i].clone());
        }

        Self {
            food_to_cuisine,
            food_to_rating,
            cuisine_foods,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        self.food_to_rating.insert(food, new_rating);
    }

    fn highest_rated(&self, cuisine: String) -> String {
        let mut best_food = String::new();
        let mut best_rating = -1;

        if let Some(foods) = self.cuisine_foods.get(&cuisine) {
            for food in foods {
                let rating = self.food_to_rating.get(food).unwrap();
                if *rating > best_rating
                    || (*rating == best_rating && (best_food.is_empty() || food < &best_food))
                {
                    best_rating = *rating;
                    best_food = food.clone();
                }
            }
        }

        best_food
    }
}
