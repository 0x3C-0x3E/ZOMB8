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

pub struct ShaderMaterials {
    pub gameover_material: Material,
    pub render_target: RenderTarget,
}

impl ShaderMaterials {
    pub fn update_render_target(&mut self) {
        self.render_target = render_target(screen_width() as u32, screen_height() as u32);
    }
}
