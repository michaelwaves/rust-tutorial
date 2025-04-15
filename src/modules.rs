mod kitchen {
    #[derive(Debug)]
    pub enum Protein {
        Chicken,
        Tofu,
        Pork,
    }
    pub struct Dish {
        pub protein: String,
        pub carb: String,
    }

    impl Dish {
        pub fn breakfast(protein: &str) -> Dish {
            Dish {
                protein: String::from(protein),
                carb: String::from("rice"),
            }
        }
    }
}

pub fn eat_dish() {
    let mut meal = kitchen::Dish::breakfast("Chicken");
    meal.protein = String::from("Tofu");
    println!("I'd like {} please", meal.protein);
    println!("The default carb is always {}", meal.carb);

    let protein_1 = kitchen::Protein::Tofu;
    let protein_2 = kitchen::Protein::Chicken;

    println!(
        "The options for protein are {:?} and {:?} and {:?}",
        protein_1,
        protein_2,
        kitchen::Protein::Pork
    )
    //meal.carb = String::from("bread");
}
