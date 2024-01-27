use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct MyResource {
    #[export(range = (1.0, 100.0))]
    #[init(default = 9.0)]
    pub my_property: f32,

    base: Base<Resource>,
}
