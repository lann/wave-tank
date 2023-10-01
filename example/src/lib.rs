cargo_component_bindings::generate!();

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world(goodbye: bool, name: Option<String>) -> String {
        format!(
            "{}, {}!",
            if goodbye { "Goodbye" } else { "Hello" },
            name.as_deref().unwrap_or("world")
        )
    }
}
