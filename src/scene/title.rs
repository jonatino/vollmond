use crate::utils::tween::Tween;
use crate::{MainState, FONT_COLOR, TITLE_ZOOM};
use keyframe::functions::{EaseIn, EaseInOut, EaseOut, Linear};
use keyframe::Keyframe;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use quad_snd::decoder;
use quad_snd::mixer::{SoundMixer, Volume};

const MUSIC_BYTES: &[u8] = include_bytes!("../../assets/music/start.ogg");

pub struct Title {
    background: Texture2D,
    title: Texture2D,
    font: Font,
    camera: Camera2D,
    animations: Vec<Tween>,
    start: bool,
}

impl Title {
    pub async fn init() -> Title {
        let tween1 = Tween::from_keyframes(
            vec![
                Keyframe::new(0.0, 0.0, EaseOut),
                Keyframe::new(8.0, 0.5, EaseOut),
                Keyframe::new(0.0, 1.0, EaseInOut),
            ],
            0,
            3,
            true,
        );
        let tween2 = Tween::from_keyframes(
            vec![
                Keyframe::new(0.0, 0.0, EaseOut),
                Keyframe::new(4.0, 0.5, EaseOut),
                Keyframe::new(0.0, 1.0, EaseIn),
            ],
            0,
            2,
            true,
        );
        let tween3 = Tween::from_keyframes(
            vec![Keyframe::new(0.0, 0.0, Linear), Keyframe::new(6.283_185_5, 1.0, Linear)],
            0,
            10,
            true,
        );
        let tween = vec![tween1, tween2, tween3];

        let camera = Camera2D {
            zoom: vec2(TITLE_ZOOM / screen_width() * 2.0, -TITLE_ZOOM / screen_height() * 2.0),
            target: vec2(0.0, 0.0),
            ..Default::default()
        };
        let background = load_texture("./assets/images/title.png").await.expect("Couldnt load title.png");
        background.set_filter(FilterMode::Nearest);
        let title = load_texture("./assets/images/vollmond.png").await.expect("Couldnt load vollmond.png");
        title.set_filter(FilterMode::Nearest);
        let font = load_ttf_font("./assets/fonts/GothicPixels.ttf").await.expect("Couldnt load main font title");
        Title {
            background,
            font,
            camera,
            title,
            animations: tween,
            start: true,
        }
    }

    pub fn run(&mut self,mixer: &mut SoundMixer) -> Option<MainState> {
        #[cfg(not(target_arch = "wasm32"))]
        if self.start {
            let id = mixer.play(decoder::read_ogg(MUSIC_BYTES).unwrap());
            mixer.set_volume(id, Volume(0.6));
            self.start = false;
        }

        self.animations[0].update();
        self.animations[1].update();
        self.animations[2].update();
        update_camera(self, vec2(0.0, 0.0));
        set_camera(&self.camera);
        draw_texture_ex(&self.background, -100.0, -50.0, WHITE, Default::default());
        draw_texture_ex(
            &self.title,
            -78.0,
            -90.0 + self.animations[0].value(),
            WHITE,
            Default::default(),
        );
        set_default_camera();
        draw_text_ex(
            "press any key",
            (screen_width() / 2.0) - 180.0,
            (screen_height() / 2.0) + 250.0 + self.animations[1].value(),
            TextParams {
                font: Some(&self.font),
                font_size: 100,
                font_scale: 0.5,
                color: FONT_COLOR,
                font_scale_aspect: 1.0,
                rotation: 0.0,
            },
        );
        process_action(self, mixer)
    }
}

fn update_camera(scene: &mut Title, new_target: Vec2) {
    scene.camera.target = new_target;
    scene.camera.zoom = vec2(TITLE_ZOOM / screen_width() * 2.0, -TITLE_ZOOM / screen_height() * 2.0);
}

fn process_action(_title: &mut Title,_mixer: &mut SoundMixer) -> Option<MainState> {
    #[cfg(target_arch = "wasm32")]
    if is_mouse_button_pressed(MouseButton::Left) && _title.start {
        let id = _mixer.play(decoder::read_ogg(MUSIC_BYTES).unwrap());
        _mixer.set_volume(id, Volume(0.6));
        _title.start = false;
    }
    if get_last_key_pressed().is_some() {
        if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
            #[cfg(not(target_arch = "wasm32"))]
            return Some(MainState::EXIT);
        } else {
            return Some(MainState::STORY);
        }
    }
    None
}
