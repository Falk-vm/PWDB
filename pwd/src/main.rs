use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Password Generator");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();
    but.set_callback(move |_| frame.set_label("Hello World!")); // the closure capture is mutable borrow to our button
    app.run().unwrap();
}

fn gen() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 8;
    let mut rng = rand::thread_rng();
    let mut pwd_vec = Vec::new();
    for _i in 0..1 {
        let password: String = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        pwd_vec.push(password)
    }/*
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("data.txt")
        .unwrap();
        
    file.write_all(pwd_vec.join("\n").as_bytes()).unwrap();
    */
    return pwd_vec
}