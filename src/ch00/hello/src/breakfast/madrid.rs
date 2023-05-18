
pub mod seafood {
    #[derive(Debug)]
    pub struct Meal {
        pub main: String,
        sub: String,
    }

    impl Meal {
        pub fn summer(main: &str) -> Meal {
            Meal {
                main: String::from(main),
                sub: String::from("Frash Fruit"),
            }
        }
    }

    pub enum Beer {
        EstrellaDamm,
        IneditDamm,
        Mahou,
        Ambar,
        Cruzcampo,
    }
}
