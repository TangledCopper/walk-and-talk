use super::{MenuId, MenuRegistry};

macro_rules! register_menus {
    ( $( $id:ident = $num:literal => $module:ident :: $type:ident ),* $(,)? ) => {
        $(
            pub mod $module;
            pub const $id: MenuId = MenuId($num);
        )*

        pub fn registry() -> MenuRegistry {
            MenuRegistry::new()
            $(
                .register_menu::<$module::$type>()
            )*
        }
    };
}

register_menus! {
    MAIN_MENU = 0 => main_menu::MainMenu,
    NO_MENU   = 1 => no_menu::NoMenu,
    INGAME_UI = 2 => ingame_ui::IngameUI,
    PAUSE = 3 => pause::Pause,
    DIALOGUE = 4 => dialogue::DialogueUI,
    CHOICE_MENU = 5 => choice::ChoiceUI
}
