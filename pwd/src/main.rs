use rand::Rng;
use std::str;
//use std::fs::OpenOptions;
//use std::io::Write;
use fltk::{app,input, button::Button, frame::Frame, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Password Generator");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Generate");
    let mut inp = input::Input::default().with_size(200, 40).with_label("Length").center_of_parent();
    wind.end();
    wind.show();
    but.set_callback(move |_| {
        let passw = gen(inp.value());
        frame.set_label(&passw);
    }); // the closure capture is mutable borrow to our button
    app.run().unwrap();
}

fn gen(password_len: String) -> String {
    let my_int: i32 = password_len.parse().unwrap();
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    for _i in 0..my_int {
        let rn = rng.gen_range(0..CHARSET.len());
        password.push_str(str::from_utf8(&[CHARSET[rn]]).unwrap());
    }

    return password;
}



/* unused/obsulete code
fn write_to_file() {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("data.txt")
        .unwrap();
        
    file.write_all(pwd_vec.join("\n").as_bytes()).unwrap();
    }


    pub struct PasswordGenerator {
    pub length: usize,
    pub numbers: bool,
    pub lowercase_letters: bool,
    pub uppercase_letters: bool,
    pub symbols: bool,
    pub spaces: bool,
    pub exclude_similar_characters: bool,
    pub strict: bool,
}
    */