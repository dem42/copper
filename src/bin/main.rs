extern crate copper;
use copper::display_manager::Display;

fn test_engine() {
    let mut display = Display::create();
    
    while !display.is_close_requested() {
        display.update_display();
    }
}

fn main() {
    test_engine();    
}
