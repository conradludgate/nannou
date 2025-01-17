// M_1_3_02
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
 * creates a texture based on random values
 *
 * MOUSE
 * click               : new noise line
 *
 * KEYS
 * s                   : save png
 */
use nannou::image;
use nannou::prelude::*;
use nannou::rand::rngs::SmallRng;
use nannou::rand::{Rng, SeedableRng};

fn main() {
    nannou::app(Box::new(model)).run();
}

struct Model {
    act_random_seed: u64,
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(512, 512)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let window = app.main_window();
    let win = window.rect();
    let texture = wgpu::TextureBuilder::new()
        .size([win.w() as u32, win.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(window.device());
    Model {
        act_random_seed: 42,
        texture,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let win = app.window_rect();
    let mut rng = SmallRng::seed_from_u64(model.act_random_seed);

    let image = image::ImageBuffer::from_fn(win.w() as u32, win.h() as u32, |_x, _y| {
        let r: u8 = rng.gen_range(0..std::u8::MAX);
        nannou::image::Rgba([r, r, r, std::u8::MAX])
    });

    let flat_samples = image.as_flat_samples();
    model.texture.upload_data(
        app.main_window().device(),
        &mut *frame.command_encoder(),
        &flat_samples.as_slice(),
    );

    let draw = app.draw();
    draw.texture(&model.texture);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.act_random_seed = (random_f32() * 100000.0) as u64;
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    if key == Key::S {
        app.main_window()
            .capture_frame(app.exe_name().unwrap() + ".png");
    }
}
