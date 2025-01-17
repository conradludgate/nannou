// P_4_3_2_01
//
// Generative Gestaltung – Creative Coding im Web
// ISBN: 978-3-87439-902-9, First Edition, Hermann Schmidt, Mainz, 2018
// Benedikt Groß, Hartmut Bohnacker, Julia Laub, Claudius Lazzeroni
// with contributions by Joey Lee and Niels Poldervaart
// Copyright 2018
//
// http://www.generative-gestaltung.de
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/**
 * pixel mapping. each pixel is translated into a new element (letter)
 *
 * KEYS
 * 1                 : toggle font size mode (dynamic/static)
 * 2                 : toggle font color mode (color/b&w)
 * arrow up/down     : maximal fontsize +/-
 * arrow right/left  : minimal fontsize +/-
 * s                 : save png
 */
use nannou::prelude::*;

use nannou::image;
use nannou::image::GenericImageView;

fn main() {
    nannou::app(Box::new(model)).run();
}

struct Model {
    image: image::DynamicImage,
    input_text: String,
    font_size_min: u32,
    font_size_max: u32,
    spacing: f32,
    kerning: f32,
    font_size_static: bool,
    black_and_white: bool,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::Wait);

    app.new_window()
        .size(533, 796)
        .view(view)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .build()
        .unwrap();

    let assets = app.assets_path().unwrap();
    let img_path = assets
        .join("images")
        .join("generative_examples")
        .join("p_4_3_1_01.png");

    let input_text = "All the world's a stage, And all the men and women merely players; They have their exits and their entrances; And one man in his time plays many parts, His acts being seven ages. At first the infant, Mewling and puking in the nurse\'s arms; Then the whining school-boy, with his satchel And shining morning face, creeping like snail Unwillingly to school. And then the lover, Sighing like furnace, with a woeful ballad Made to his mistress\' eyebrow. Then a soldier, Full of strange oaths, and bearded like the pard, Jealous in honour, sudden and quick in quarrel, Seeking the bubble reputation Even in the cannon\'s mouth. And then the justice, In fair round belly with good capon lin\'d, With eyes severe and beard of formal cut, Full of wise saws and modern instances; And so he plays his part. The sixth age shifts Into the lean and slipper\'d pantaloon, With spectacles on nose and pouch on side, His youthful hose, well sav\'d, a world too wide For his shrunk shank; and his big manly voice, Turning again toward childish treble, pipes And whistles in his sound. Last scene of all, That ends this strange eventful history, Is second childishness and mere oblivion; Sans teeth, sans eyes, sans taste, sans every thing.".to_string();

    let image = image::open(img_path).unwrap();
    Model {
        image,
        input_text,
        font_size_min: 10,
        font_size_max: 20,
        spacing: 12.0,
        kerning: 0.5,
        font_size_static: false,
        black_and_white: false,
    }
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);

    let draw = app.draw();
    let win = app.window_rect();

    let mut x = win.left();
    let mut y = win.top();
    let mut counter = 0;

    while y > win.bottom() {
        // translate position (display) to position (image)
        let (w, h) = model.image.dimensions();
        let img_x = map_range(x, win.left(), win.right(), 0, w);
        let img_y = map_range(y, win.top(), win.bottom(), 0, h);
        // get current color
        let c = model.image.get_pixel(img_x, img_y);
        // greyscale conversion
        let red = c[0] as f32 / 255.0;
        let green = c[1] as f32 / 255.0;
        let blue = c[2] as f32 / 255.0;
        let greyscale = red * 0.222 + green * 0.707 + blue * 0.071;

        let draw = draw.x_y(x, y);

        let (font_size, col) = if model.font_size_static {
            let col = if model.black_and_white {
                rgb(greyscale, greyscale, greyscale)
            } else {
                rgb(red, green, blue)
            };
            (model.font_size_max, col)
        } else {
            // greyscale to fontsize
            let font_size = map_range(
                greyscale,
                0.0,
                1.0,
                model.font_size_max,
                model.font_size_min,
            )
            .max(1);
            let col = if model.black_and_white {
                rgb(0.0, 0.0, 0.0)
            } else {
                rgb(red, green, blue)
            };
            (font_size, col)
        };

        let letter = &model.input_text.chars().nth(counter).unwrap().to_string();
        draw.text(letter)
            .font_size(font_size)
            .x_y(0.0, 0.0)
            .color(col);

        let letter_width = 9.0 + model.kerning;
        x += letter_width;

        // linebreaks
        if x + letter_width >= win.right() {
            x = win.left();
            y -= model.spacing;
        }

        counter += 1;
        if counter >= model.input_text.len() {
            counter = 0;
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        // change fontSizeMax with arrow keys up/down
        Key::Up => {
            model.font_size_max += 2;
        }
        Key::Down => {
            model.font_size_max -= 2;
        }
        // change fontSizeMin with arrow keys left/right
        Key::Right => {
            model.font_size_min += 2;
        }
        Key::Left => {
            model.font_size_min -= 2;
        }
        _otherkey => (),
    }
}

fn key_released(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            app.main_window()
                .capture_frame(app.exe_name().unwrap() + ".png");
        }
        // change render mode
        Key::Key1 => {
            model.font_size_static = !model.font_size_static;
        }
        // change color style
        Key::Key2 => {
            model.black_and_white = !model.black_and_white;
        }
        _otherkey => (),
    }
}
