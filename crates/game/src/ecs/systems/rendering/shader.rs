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

pub enum AvailableShaders {
    GameoverMaterial,
    CrtMaterial,
}

pub struct ShaderState {
    pub gameover_material: Material,
    pub crt_material: Material,

    pub render_target: RenderTarget,
    pub render_camera: Camera2D,
}

impl ShaderState {
    pub fn load() -> Option<Self> {
        let render_target = render_target(screen_width() as u32, screen_height() as u32);

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

        let mut render_camera =
            Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));

        render_camera.render_target = Some(render_target.clone());

        Some(Self {
            render_target,
            gameover_material,
            crt_material,
            render_camera,
        })
    }

    pub fn check_screen_changed(&mut self) {
        if (screen_width(), screen_height())
            != (
                self.render_target.texture.width(),
                self.render_target.texture.height(),
            )
        {
            self.update_render_target();
        }
    }

    pub fn update_render_target(&mut self) {
        self.render_target = render_target(screen_width() as u32, screen_height() as u32);
        self.render_camera =
            Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));
        self.render_camera.render_target = Some(self.render_target.clone());
    }

    pub fn set_camera(&self) {
        set_camera(&self.render_camera);
    }

    pub fn set_shader(&mut self, shader_kind: AvailableShaders) {
        match shader_kind {
            AvailableShaders::GameoverMaterial => {
                self.update_gameover_material();
                gl_use_material(&self.gameover_material);
            }
            AvailableShaders::CrtMaterial => {
                self.update_crt_material();
                gl_use_material(&self.crt_material)
            }
        }
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
