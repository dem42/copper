extern crate copper;
use copper::display_manager;

fn test_engine() {
    display_manager::create_display();

    while !display_manager::is_close_requested() {
        display_manager::update_display();
    }

    display_manager::close_display();
}

fn main() {
    test_engine();
}
