use crate::inputs::InputBindings;
use bevy::prelude::*;

pub fn load_bindings(mut commands: Commands) {
    let bindings = try_load_save()
        .or_else(|| try_load_default())
        .unwrap_or_else(|| {
            warn!("No bindings files found, using hardcoded defaults");
            InputBindings::default()
        });

    commands.insert_resource(bindings);
}

fn try_load_save() -> Option<InputBindings> {
    let contents = std::fs::read_to_string(save_path()).ok()?;
    match ron::from_str::<InputBindings>(&contents) {
        Ok(b) => Some(b),
        Err(e) => {
            warn!("Save file bindings corrupt or outdated, falling back: {e}");
            None
        }
    }
}

fn try_load_default() -> Option<InputBindings> {
    let contents = std::fs::read_to_string("assets/bindings.ron").ok()?;
    match ron::from_str::<InputBindings>(&contents) {
        Ok(b) => Some(b),
        Err(e) => {
            error!("Default bindings.ron failed to parse: {e}");
            None
        }
    }
}

pub fn save_bindings(bindings: Res<InputBindings>) {
    let path = save_path();
    std::fs::create_dir_all(path.parent().unwrap()).ok();

    match ron::ser::to_string_pretty(&*bindings, ron::ser::PrettyConfig::default()) {
        Ok(serialized) => {
            if let Err(e) = std::fs::write(&path, serialized) {
                error!("Failed to write bindings save: {e}");
            }
        }
        Err(e) => error!("Failed to serialize bindings: {e}"),
    }
}

fn save_path() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("walk-and-talk")
        .join("saves") // wherever your save system roots itself
        .join("bindings.ron")
}
