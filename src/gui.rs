extern crate core;
extern crate image as new_image;

use fltk::{app, button, frame, image, input, menu, window};
use fltk::enums::{Event};
use fltk::prelude::{DisplayExt, GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt};
use fltk::text::{TextBuffer, TextDisplay};
use new_image::{ColorType, EncodableLayout};
use random_string::generate;

use crate::entities::{get_album_response, get_artist_response, get_track_response};
use crate::token::extract_token;


const DOWNLOAD_PATH: &str = "images/";
const CHAR_SET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn create_gui()
{
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

    let mut frame1 = frame::Frame::default();
    frame1.set_pos(20, 50);
    frame1.set_size(200, 200);
    frame1.hide();

    let mut frame2 = frame::Frame::default();
    frame2.set_pos(20, 280);
    frame2.set_size(200, 200);
    frame2.hide();

    let mut frame3 = frame::Frame::default();
    frame3.set_pos(20, 500);
    frame3.set_size(200, 200);
    frame3.hide();

    let mut artist_frame = frame::Frame::default();
    artist_frame.set_pos(150, 50);
    artist_frame.set_size(500, 500);
    artist_frame.hide();

    let mut artist_text = TextDisplay::default();
    artist_text.set_pos(50, 550);
    artist_text.set_size(700, 230);
    artist_text.set_text_size(20);
    artist_text.hide();

    let mut text1 = TextDisplay::default();
    text1.set_pos(250, 50);
    text1.set_size(500, 200);
    text1.set_text_size(20);
    text1.hide();

    let mut text2 = TextDisplay::default();
    text2.set_pos(250, 280);
    text2.set_size(500, 200);
    text2.set_text_size(20);
    text2.hide();

    let mut text3 = TextDisplay::default();
    text3.set_pos(250, 500);
    text3.set_size(500, 200);
    text3.set_text_size(20);
    text3.hide();

    dropdown_menu.set_callback({
        let mut button_clone = button.clone();
        move |this_dropdown_menu| {
            if this_dropdown_menu.changed() && this_dropdown_menu.choice().unwrap() != "" {
                button_clone.show();
            }
        }
    });

    frame1.handle({
        let frame_copy = frame1.clone();
        move |_, ev| match ev {
            Event::Push => {
                if app::event_mouse_button() == app::MouseButton::Left {
                    let image = frame_copy.image().unwrap().to_rgb_data();
                    println!("SIZE: {}", image.len());
                    let slice = image.as_slice();
                    let random_name = generate(5, CHAR_SET);
                    let path = DOWNLOAD_PATH.to_string() + random_name.as_str() + ".png";
                    new_image::save_buffer(path, slice, frame_copy.width() as u32, frame_copy.height() as u32, ColorType::Rgb8).unwrap();
                    true
                } else {
                    false
                }
            }
            _ => false,
        }});

    frame2.handle({
        let frame_copy = frame2.clone();
        move |_, ev| match ev {
            Event::Push => {
                if app::event_mouse_button() == app::MouseButton::Left {
                    let image = frame_copy.image().unwrap().to_rgb_data();
                    println!("SIZE: {}", image.len());
                    let slice = image.as_slice();
                    let random_name = generate(5, CHAR_SET);
                    let path = DOWNLOAD_PATH.to_string() + random_name.as_str() + ".png";
                    new_image::save_buffer(path, slice, frame_copy.width() as u32, frame_copy.height() as u32, ColorType::Rgb8).unwrap();
                    true
                } else {
                    false
                }
            }
            _ => false,
        }});

    frame3.handle({
        let frame_copy = frame3.clone();
        move |_, ev| match ev {
            Event::Push => {
                if app::event_mouse_button() == app::MouseButton::Left {
                    let image = frame_copy.image().unwrap().to_rgb_data();
                    println!("SIZE: {}", image.len());
                    let slice = image.as_slice();
                    let random_name = generate(5, CHAR_SET);
                    let path = DOWNLOAD_PATH.to_string() + random_name.as_str() + ".png";
                    new_image::save_buffer(path, slice, frame_copy.width() as u32, frame_copy.height() as u32, ColorType::Rgb8).unwrap();
                    true
                } else {
                    false
                }
            }
            _ => false,
        }});

    artist_frame.handle({
        let frame_copy = artist_frame.clone();
        move |_, ev| match ev {
            Event::Push => {
                if app::event_mouse_button() == app::MouseButton::Left {
                    let image = frame_copy.image().unwrap().to_rgb_data();
                    println!("SIZE: {}", image.len());
                    let slice = image.as_slice();
                    let random_name = generate(5, CHAR_SET);
                    let path = DOWNLOAD_PATH.to_string() + random_name.as_str() + ".png";
                    new_image::save_buffer(path, slice, frame_copy.width() as u32, frame_copy.height() as u32, ColorType::Rgb8).unwrap();
                    true
                } else {
                    false
                }
            }
            _ => false,
        }});

    button.set_callback({
        let dropdown_menu = dropdown_menu.clone();
        let search_bar = search_bar.clone();
        move |_| {
            let choice = dropdown_menu.choice().unwrap();
            let search_query = search_bar.value();
            let token = extract_token();

            match choice.as_str() {
                "Album" => {
                    let albums = get_album_response(search_query.as_str(), token.clone());
                    let mut image_url_vector = Vec::new();
                    for album in &albums {
                        image_url_vector.push(album.get_album_image().clone());
                    }

                    let image_bytes1 = reqwest::blocking::get(image_url_vector.get(0).unwrap()).unwrap().bytes().unwrap();
                    let image_bytes2 = reqwest::blocking::get(image_url_vector.get(1).unwrap()).unwrap().bytes().unwrap();
                    let image_bytes3 = reqwest::blocking::get(image_url_vector.get(2).unwrap()).unwrap().bytes().unwrap();

                    let image1 = image::JpegImage::from_data(image_bytes1.as_bytes()).unwrap();
                    let image2 = image::JpegImage::from_data(image_bytes2.as_bytes()).unwrap();
                    let image3 = image::JpegImage::from_data(image_bytes3.as_bytes()).unwrap();

                    frame1.set_image_scaled(Some(image1));
                    frame1.redraw();
                    frame1.show();

                    frame2.set_image_scaled(Some(image2));
                    frame2.redraw();
                    frame2.show();

                    frame3.set_image_scaled(Some(image3));
                    frame3.redraw();
                    frame3.show();

                    let mut text_buffer1 = TextBuffer::default();
                    let mut text_buffer2 = TextBuffer::default();
                    let mut text_buffer3 = TextBuffer::default();

                    text_buffer1.set_text(albums.get(0).unwrap().print_album_info().as_str());
                    text1.set_buffer(text_buffer1.clone());

                    text_buffer2.set_text(albums.get(1).unwrap().print_album_info().as_str());
                    text2.set_buffer(text_buffer2.clone());

                    text_buffer3.set_text(albums.get(2).unwrap().print_album_info().as_str());
                    text3.set_buffer(text_buffer3.clone());

                    text1.show();
                    text2.show();
                    text3.show();

                    artist_frame.hide();
                    artist_text.hide();
                },
                "Artist" => {
                    let artist_vec = get_artist_response(search_query.as_str(), token.clone());
                    let artist = artist_vec.first().unwrap();
                    let image_bytes = reqwest::blocking::get(artist.get_artist_image()).unwrap().bytes().unwrap();
                    let image = image::JpegImage::from_data(image_bytes.as_bytes()).unwrap();
                    artist_frame.set_image_scaled(Some(image));
                    artist_frame.redraw();
                    artist_frame.show();

                    let mut text_buffer = TextBuffer::default();
                    text_buffer.set_text(artist.print_artist_info().as_str());
                    artist_text.set_buffer(text_buffer.clone());

                    text1.hide();
                    text2.hide();
                    text3.hide();

                    frame1.hide();
                    frame2.hide();
                    frame3.hide();

                    artist_text.show();
                }
                "Song" => {
                    let tracks = get_track_response(search_query.as_str(), token.clone());
                    let mut image_url_vector = Vec::new();
                    for track in &tracks {
                        image_url_vector.push(track.album.get_album_image());
                    }

                    let image_bytes1 = reqwest::blocking::get(image_url_vector.get(0).unwrap().as_str()).unwrap().bytes().unwrap();
                    let image_bytes2 = reqwest::blocking::get(image_url_vector.get(1).unwrap().as_str()).unwrap().bytes().unwrap();
                    let image_bytes3 = reqwest::blocking::get(image_url_vector.get(2).unwrap().as_str()).unwrap().bytes().unwrap();

                    let image1 = image::JpegImage::from_data(image_bytes1.as_bytes()).unwrap();
                    let image2 = image::JpegImage::from_data(image_bytes2.as_bytes()).unwrap();
                    let image3 = image::JpegImage::from_data(image_bytes3.as_bytes()).unwrap();

                    frame1.set_image_scaled(Some(image1));
                    frame1.redraw();
                    frame1.show();

                    frame2.set_image_scaled(Some(image2));
                    frame2.redraw();
                    frame2.show();

                    frame3.set_image_scaled(Some(image3));
                    frame3.redraw();
                    frame3.show();

                    let mut text_buffer1 = TextBuffer::default();
                    let mut text_buffer2 = TextBuffer::default();
                    let mut text_buffer3 = TextBuffer::default();

                    text_buffer1.set_text(tracks.get(0).unwrap().print_track_info().as_str());
                    text1.set_buffer(text_buffer1.clone());

                    text_buffer2.set_text(tracks.get(1).unwrap().print_track_info().as_str());
                    text2.set_buffer(text_buffer2.clone());

                    text_buffer3.set_text(tracks.get(2).unwrap().print_track_info().as_str());
                    text3.set_buffer(text_buffer3.clone());

                    text1.show();
                    text2.show();
                    text3.show();

                    artist_frame.hide();
                    artist_text.hide();
                },
                _ => {}
            }
        }
    });

    window.end();
    window.show();

    application.run().unwrap();
}