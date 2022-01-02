use gdnative::prelude::*;

mod lantern;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<lantern::Lantern>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
