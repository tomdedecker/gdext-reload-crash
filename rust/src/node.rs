use godot::prelude::*;

use crate::resource::MyResource;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct MyNode {
    #[export]
    resource: Gd<MyResource>,

    base: Base<Node>,
}

#[godot_api]
impl INode for MyNode {
    fn ready(&mut self) {
        godot_print!("ready()");
        // godot_print!("ready() - 2");
    }
}
