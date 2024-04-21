mod arguments;
mod dynamor;
mod menu;
mod types;

fn main() {
    arguments::parse_args();
    menu::show_menu();
}
