use macroquad::prelude::*;
use std::future::Future;
use crate::{MainState, TITLE_ZOOM, FONT_COLOR};
use crate::utils::timer::Timer;

pub struct End{
    camera: Camera2D,
    font: Font,
    show_text1: bool,
    text1: String,
    text2: String,
    timer: Timer,
}

impl End{
    pub fn init() -> impl Future<Output = End>{
        async move {
            let camera = Camera2D {
                zoom: vec2(TITLE_ZOOM / screen_width() * 2.0, -TITLE_ZOOM/ screen_height() * 2.0),
                target: vec2(0.0, 0.0),
                ..Default::default()
            };
            // todo write end text
            let font = load_ttf_font_from_bytes(include_bytes!("../../assets/fonts/GothicPixels.ttf"));
            let text1 = "It is almost full moon.\nThe night of your transformation\nis about to begin.\nThe only thing that can stop it\nis the Moonshot potion.";
            let text2 = "Bring me a werewolf hair,\na piece of moonmilk,\nthe fruits of the moonseed\nand a moonflower.\nI can then brew\nthe moonshot for you.";
            End {
                camera,
                font,
                show_text1: true,
                text1: text1.to_string(),
                text2: text2.to_string(),
                timer: Timer::new_sec(3),
            }
        }
    }

    pub fn run(&mut self) -> Option<MainState>{
        update_camera(self, vec2(0.0,0.0));
        set_camera(self.camera);
        //draw_texture_ex(self.texture1, 0.0, 0.0, WHITE, Default::default());
        set_default_camera();
        let tp = TextParams{ font: self.font, font_size: 80, font_scale: 0.5, color: FONT_COLOR };
        if self.show_text1{
            for (i, line) in self.text1.split("\n").enumerate(){
                draw_text_ex(line, (screen_width()/2.0)-350.0, (screen_height()/2.0) - 350.0 + i as f32 * 80.0, tp);
            }
        }else{
            for (i, line) in self.text2.split("\n").enumerate(){
                draw_text_ex(line, (screen_width()/2.0)-350.0, (screen_height()/2.0) - 350.0 + i as f32 * 80.0, tp);
            }
        }

        if self.timer.finished(){
            draw_text_ex("press any key",(screen_width()/2.0)-180.0,(screen_height()/2.0) + 250.0,TextParams{
                font: self.font,
                font_size: 100,
                font_scale: 0.5,
                color: FONT_COLOR
            });
        }

        if get_last_key_pressed().is_some() {
            #[cfg(not(target_arch = "wasm32"))]
            if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
                return Some(MainState::EXIT);
            } else {
                if self.timer.finished(){
                    return Some(MainState::EXIT);
                }
            }
        }
        None
    }
}

fn update_camera(scene: &mut End, new_target: Vec2){
    scene.camera.target = new_target;
    scene.camera.zoom =  vec2(TITLE_ZOOM / screen_width()* 2.0, -TITLE_ZOOM / screen_height()* 2.0);
}

