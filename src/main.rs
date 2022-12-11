mod entities;
mod api_response;
mod external_url;
mod token;

use std::borrow::{Borrow, BorrowMut};
use std::env;
use std::fmt::Debug;
use std::path::Path;
use fltk::{app, button, frame, image, input, menu, window};
use fltk::button::Button;
use fltk::enums::Color;
use fltk::frame::Frame;
use fltk::prelude::{GroupExt, ImageExt, InputExt, MenuExt, WidgetExt, WindowExt};
use new_image::ColorType;
use reqwest::Client;
use tokio::runtime::Runtime;
use crate::entities::{get_album_response, get_artist_response};
use crate::token::extract_token;

extern crate image as new_image;
extern crate core;

// mod token;
// mod entities;
// mod api_response;
// mod external_url;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 2 {
//         panic!("Command line argument error");
//     }
//
//     let artist_name = &args[1];
//     let albums = get_album_response(artist_name, extract_token());
//     for album in albums {
//         println!("{}", album);
//         album.print_covers();
//     }
//
//     let artists = get_artist_response(artist_name, Client::new(), extract_token().await).await;
//     for artist in artists {
//         println!("{}", artist);
//     }
//
// }

 fn main() {
    let application = app::App::default().with_scheme(app::Scheme::Base);
    let mut window = window::Window::default()
        .with_size(800, 800)
        .center_screen()
        .with_label("TEST!");

    let mut search_bar = input::Input::default()
        .with_size(600, 20)
        .with_label("Search");
    search_bar.set_pos(50, 10);

    let mut dropdown_menu = menu::Choice::default()
        .with_size(50, 20)
        .right_of(&search_bar, 10);
    dropdown_menu.add_choice("Album|Artist|Song");

    let mut button = button::Button::default()
        .with_label("Search")
        .with_size(50, 20)
        .right_of(&dropdown_menu, 10);
    button.hide();

    dropdown_menu.set_callback({
        let mut button_clone = button.clone();
        move |this_dropdown_menu| {
            if this_dropdown_menu.changed() && this_dropdown_menu.choice().unwrap() != "" {
                button_clone.show();
            }
        }
    });

    button.set_callback({
        let mut dropdown_menu = dropdown_menu.clone();
        let mut search_bar = search_bar.clone();
        move |this_button| {
            let choice = dropdown_menu.choice().unwrap();
            let search_query = search_bar.value();
            let token = extract_token();
            match choice.as_str() {
                "Artist" => get_artist_response(search_query.as_str(), token),
                "Album" => println!("Album"),
                "Song" => println!("Song"),
                _ => {}
            }
        }
    });

    window.end();
    window.show();

    application.run().unwrap();
}