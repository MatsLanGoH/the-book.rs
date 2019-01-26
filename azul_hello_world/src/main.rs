extern crate azul;

use azul::{
    prelude::*,
    widgets::{
        label::Label,
        button::Button,
    },
};

struct CounterApplication {
    counter: usize,
}

impl Layout for CounterApplication {
    fn layout(&self, _info: WindowInfo<Self>) -> Dom<Self> {
        // `.dom()` is convention to transform a given widget into a DOM
        let label = Label::new(format!("{}", self.counter)).dom();
        let button = Button::with_label("Update counter").dom()
            .with_callback(On::MouseUp, Callback(update_counter));

        Dom::new(NodeType::Div)
            .with_child(label)
            .with_child(button)
    }
}
fn update_counter(app_state: &mut AppState<CounterApplication>,
                  _event: WindowEvent<CounterApplication>) -> UpdateScreen {
    app_state.data.modify(|state| state.counter += 1);
    Redraw
}

fn main() {
    let app = App::new(
        CounterApplication { 
            counter: 0,
        },
        AppConfig::default()
    );
    let window = Window::new(
        WindowCreateOptions::default(),
        css::native()
    ).unwrap();
    app.run(window).unwrap();
}
