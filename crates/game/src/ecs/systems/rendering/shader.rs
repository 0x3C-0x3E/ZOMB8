use macroquad::prelude::*;

const DEFAULT_VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    // color = color0 / 255.0;
    uv = texcoord;
}
";

const GAMEOVER_FRAG_SHADER: &str = include_str!("gameover.glsl");
const CRT_FRAG_SHADER: &str = include_str!("crt.glsl");
const BLOOD_FRAG_SHADER: &str = include_str!("blood.glsl");

#[derive(Clone, Copy)]
pub enum AvailableShaders {
    None,
    GameoverMaterial,
    CrtMaterial,
    BloodMaterial,
}

pub struct ShaderState {
    pub gameover_material: Material,
    pub crt_material: Material,
    pub blood_material: Material,

    pub targets: [RenderTarget; 2],
    pub cameras: [Camera2D; 2],
}

impl ShaderState {
    pub fn load() -> Option<Self> {
        let (w, h) = (screen_width() as u32, screen_height() as u32);
        let targets = [render_target(w, h), render_target(w, h)];

        let gameover_material = load_material(
            ShaderSource::Glsl {
                vertex: DEFAULT_VERTEX_SHADER,
                fragment: GAMEOVER_FRAG_SHADER,
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("screen_size", UniformType::Float2),
                    UniformDesc::new("center", UniformType::Float2),
                    UniformDesc::new("radius", UniformType::Float1),
                ],
                ..Default::default()
            },
        )
        .unwrap();

        let crt_material = load_material(
            ShaderSource::Glsl {
                vertex: DEFAULT_VERTEX_SHADER,
                fragment: CRT_FRAG_SHADER,
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("screen_size", UniformType::Float2),
                    UniformDesc::new("time", UniformType::Float1),
                    UniformDesc::new("strength", UniformType::Float1),
                ],
                ..Default::default()
            },
        )
        .unwrap();

        let blood_material = load_material(
            ShaderSource::Glsl {
                vertex: DEFAULT_VERTEX_SHADER,
                fragment: BLOOD_FRAG_SHADER,
            },
            MaterialParams {
                ..Default::default()
            },
        )
        .unwrap();

        let cameras = [0, 1].map(|i| {
            let mut cam =
                Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));
            cam.render_target = Some(targets[i].clone());
            cam
        });

        Some(Self {
            targets,
            cameras,

            gameover_material,
            crt_material,
            blood_material,
        })
    }

    pub fn check_screen_changed(&mut self) {
        if (screen_width(), screen_height())
            != (
                self.targets[0].texture.width(),
                self.targets[0].texture.height(),
            )
        {
            self.update_render_targets();
        }
    }

    pub fn update_render_targets(&mut self) {
        let (w, h) = (screen_width() as u32, screen_height() as u32);
        self.targets = [render_target(w, h), render_target(w, h)];
        for i in 0..2 {
            let mut cam =
                Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));
            cam.render_target = Some(self.targets[i].clone());
            self.cameras[i] = cam;
        }
    }

    fn material_for(&mut self, shader: &AvailableShaders) -> Option<&Material> {
        match shader {
            AvailableShaders::None => None,
            AvailableShaders::GameoverMaterial => {
                self.update_gameover_material();
                Some(&self.gameover_material)
            }
            AvailableShaders::CrtMaterial => {
                self.update_crt_material();
                Some(&self.crt_material)
            }
            AvailableShaders::BloodMaterial => Some(&self.blood_material),
        }
    }

    pub fn begin_scene(&self) {
        set_camera(&self.cameras[0]);
        clear_background(BLANK);
        gl_use_default_material();
    }

    pub fn run_pipeline(&mut self, passes: &[AvailableShaders], offset_pos: (f32, f32)) -> usize {
        let mut src = 0;
        let mut dst = 1;

        for pass in passes {
            set_camera(&self.cameras[dst]);
            let material = self.material_for(pass);

            match material {
                Some(m) => gl_use_material(m),
                None => gl_use_default_material(),
            }
            let src_tex = self.targets[src].texture.clone();
            draw_texture_ex(
                &src_tex,
                offset_pos.0,
                offset_pos.1,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    flip_y: true,
                    ..Default::default()
                },
            );

            std::mem::swap(&mut src, &mut dst);
        }

        gl_use_default_material();
        src
    }

    pub fn present(&self, target_index: usize) {
        set_default_camera();
        gl_use_default_material();
        draw_texture_ex(
            &self.targets[target_index].texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );
    }

    pub fn update_gameover_material(&mut self) {
        let radius = 150.0_f32;

        self.gameover_material
            .set_uniform("screen_size", (screen_width(), screen_height()));
        self.gameover_material
            .set_uniform("center", (screen_width() / 2., screen_height() / 2.));
        self.gameover_material.set_uniform("radius", radius);
    }

    pub fn update_crt_material(&mut self) {
        self.crt_material
            .set_uniform("screen_size", (screen_width(), screen_height()));
        self.crt_material.set_uniform("time", get_time() as f32);
        self.crt_material.set_uniform("strength", 0.03_f32);
    }
}
