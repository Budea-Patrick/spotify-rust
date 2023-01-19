extern crate image as new_image;

use crate::gui::create_gui;

mod api_response;
mod entities;
mod external_url;
mod token;
mod gui;

fn main() {
    create_gui();
}
