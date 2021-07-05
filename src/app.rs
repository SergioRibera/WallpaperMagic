use fltk::enums::CallbackTrigger;
use fltk::{ prelude::*, enums::Event, * };

pub fn launch_app_gui () {
    let app = app::App::default();
    let mut win: window::Window = window::Window::new(100, 100, 400, 300, "WallpaperMagic");
    win.end();
    win.show();

    let mut inp = input::Input::default();
    inp.set_trigger(CallbackTrigger::Changed);
    inp.set_callback(|i| println!("my current value is: {}", i.value()));

    let mut btn = button::Button::new(160, 200, 80, 40, "Click Me!");
    win.add(&btn);
    btn.handle(|b, e| {
        let mut str_event = "Not";
        match e {
            Event::Push => str_event = "Pushed!",
            Event::Focus => str_event = "Hover Me",
            Event::Unfocus => str_event = "Last hover me",
            Event::KeyDown => str_event = "Key Down",
            _ => str_event = "Not Event"
        }
        b.set_label(str_event);
        false
    });

    app.run().unwrap();
}
