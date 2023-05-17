pub mod alcohol {

    pub mod whiskey {
        pub mod tennessee {
            #[derive(Debug)]
            pub struct JackDanial {}
        }
        pub mod bourbon {
            #[derive(Debug)]
            pub struct JimBeam {}
        }
    }

    pub(super) mod sake {
        pub mod junmai {
            #[derive(Debug)]
            pub struct Kubota {}

            #[derive(Debug)]

            pub struct Hakaisan {}
        }

        pub mod honjozo {

        }
    }

    use crate::alcohol::sake::junmai::{Kubota, Hakaisan};

    pub fn build_sake() -> Kubota {
        Kubota {}
    }

    pub fn build_hakaisan() -> Hakaisan {
        Hakaisan {}
    }
}

