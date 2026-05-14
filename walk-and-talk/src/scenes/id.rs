use super::{LevelId, LevelRegistry};

macro_rules! register_levels {
    ( $( $id:ident = $num:literal => $module:ident :: $type:ident ),* $(,)? ) => {
        $(
            pub mod $module;
            pub const $id: LevelId = LevelId($num);
        )*

        pub fn registry() -> LevelRegistry {
            LevelRegistry::new()
            $(
                .register_level::<$module::$type>()
            )*
        }
    };
}

register_levels! {
    BLANK  = 0 => blank::Blank,
    LEVEL1 = 1 => level1::Level1,
}
