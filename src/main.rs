mod ascii;


struct MainMenu {
    title: String
}
fn draw_menu(menu: &MainMenu) {
   println!("{}", menu.title);
}
fn main() {
    let menu = MainMenu {
        title: ascii::TITLE.to_string()
    };
    draw_menu(&menu);
}
