
use bevy::{prelude::*};

use crate::ants::{item};

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(item::setup_item);
    }
}
