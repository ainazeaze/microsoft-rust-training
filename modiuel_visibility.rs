mod kitchen {
    fn secret_recipe() -> &'static str {
        "42 spices"
    }
    pub fn menu() -> &'static str {
        "Today's special"
    }

    pub mod staff {
        pub fn cook() -> String {
            format!("Cooking with {}", super::secret_recipe())
        }
    }
}

fn main() {
    println!("{}", kitchen::menu()); // Line A compiles : menu() is pub
    println!("{}", kitchen::secret_recipe()); // Line B does not compile : secret_kitchen() is private to kitchen
    println!("{}", kitchen::staff::cook()); // Line C compiles: staff::cook() is pub and cook () can access secret_recipe () via super::
}
