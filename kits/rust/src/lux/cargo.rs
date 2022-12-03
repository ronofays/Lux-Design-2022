
// Cargo object

pub mod cargo {
    pub struct UnitCargo {
        pub ice: u32,
        pub ore: u32,
        pub water: u32,
        pub metal: u32,
    }

    impl UnitCargo {
        pub fn new() -> UnitCargo {
            UnitCargo {
                ice: 0,
                ore: 0,
                water: 0,
                metal: 0,
            }
        }
    }


}