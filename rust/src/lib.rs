use godot::prelude::*;

mod node;
mod resource;

struct GdExtSample;

#[gdextension]
unsafe impl ExtensionLibrary for GdExtSample {}
