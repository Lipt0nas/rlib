use gfx::*;
use rlib::*;

struct Application {
    texture: gfx::texture::Texture,
    wall_texture: gfx::texture::Texture,
    batch: gfx::sprite_batch::SpriteBatch<1000>,
    test_val: f32,
    sprites: Vec<gfx::sprite_batch::Sprite>,
    lmao: f32,
    lmao2: f32,
}

impl rlib::RLibApp for Application {
    fn new() -> Self {
        let texture = gfx::texture::Texture::from_file("data/amogus.png").unwrap();
        let wall_texture = gfx::texture::Texture::from_file("data/stonewall.png").unwrap();

        let sprites = vec![
            gfx::sprite_batch::Sprite {
                texture: texture.clone(),
                x: 0.0,
                y: 0.0,
                width: 0.5,
                height: 0.5,
                color: gfx::color::Color::from_rgba(1.0, 1.0, 1.0, 1.0),
            },
            gfx::sprite_batch::Sprite {
                texture: wall_texture.clone(),
                x: -0.5,
                y: -0.5,
                width: 0.5,
                height: 0.5,
                color: gfx::color::Color::from_rgba(1.0, 1.0, 1.0, 1.0),
            },
        ];

        Self {
            texture,
            wall_texture,
            batch: gfx::sprite_batch::SpriteBatch::new().unwrap(),
            test_val: 0.0,
            sprites,
            lmao: 0.0,
            lmao2: 0.0,
        }
    }

    fn init(&mut self) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    fn on_key(&mut self, scancode: u32) {
        println!("{}", scancode);

        if scancode == 26 {
            self.lmao2 += 0.006;
        }

        if scancode == 22 {
            self.lmao2 -= 0.006;
        }

        if scancode == 7 {
            self.lmao += 0.006;
        }

        if scancode == 4 {
            self.lmao -= 0.006;
        }
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let x = f32::sin(self.test_val);
        self.test_val += 0.0004;

        self.batch.begin_batch();
        {
            self.batch.draw(
                &self.texture,
                -0.25 - x,
                0.0,
                0.5,
                0.5,
                Some(color::Color::from_rgba(1.0, x, 0.0, 1.0)),
            );
            self.batch
                .draw(&self.texture, self.lmao, self.lmao2, 0.1, 0.1, None);

            for spr in self.sprites.iter() {
                self.batch.draw_sprite(spr);
            }
        }
        self.batch.end_batch();
    }
}

fn main() {
    rlib::init::<Application>(rlib::RlibConfig {
        window_title: "Rlib Test".to_string(),
        window_width: 1600,
        window_height: 900,
    });
}
