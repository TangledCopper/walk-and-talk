pub mod button_light;
pub mod floor;
pub mod floor_button;
pub mod npc;
pub mod player;
pub mod prelude;

use super::prelude::*;
use bevy::prelude::*;
use button_light::ButtonLightPlugin;
use floor_button::{ButtonTriggered, FloorButtonPlugin};
use npc::NPCPlugin;
use player::{ActiveInteractible, PendingInteraction, PlayerPlugin};

pub struct PrefabsPlugin;
impl Plugin for PrefabsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ButtonTriggered>()
            .init_resource::<ActiveInteractible>()
            .init_resource::<PendingInteraction>()
            .add_plugins(NPCPlugin::default())
            .add_plugins(FloorButtonPlugin::default())
            .add_plugins(ButtonLightPlugin::default())
            .add_plugins(PlayerPlugin::default());
    }
}

#[macro_export]
macro_rules! prefab_plugin {
    ( $plugin:ident, $marker:ty, [ $( $system:expr ),* $(,)? ] ) => {
        #[derive(Default)]
        pub struct $plugin;

        impl Plugin for $plugin {
            fn build(&self, app: &mut App) {
                app.add_systems(Update, (
                    $(
                        $system.run_if(any_with_component::<$marker>),
                    )*
                ));
            }
        }
    };
}
