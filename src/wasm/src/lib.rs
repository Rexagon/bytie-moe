mod cat;
mod cube;
mod mesh;
mod shader;
mod stuff;

use std::cell::RefCell;
use std::rc::Rc;

use nalgebra_glm as glm;
use nalgebra_glm::Mat4;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

use mesh::Mesh;
use shader::Shader;
use stuff::{create_context, request_animation_frame};

type Color = (f32, f32, f32);

static BACKGROUND_COLOR: Color = (0.933, 0.933, 0.933);

static MESH_VERTEX_SHADER: &'static str = r#"
attribute vec3 position;
attribute vec2 texture_coords;
attribute vec3 normal;

uniform mat4 projection_matrix;
uniform vec3 translation;

varying vec3 f_normal;

void main() {
    gl_Position = projection_matrix * vec4(translation + position, 1.0);
    f_normal = normal;
}
"#;

static MESH_FRAGMENT_SHADER: &'static str = r#"
precision mediump float;

varying vec3 f_normal;

void main() {
    vec3 normal = normalize(f_normal);
    vec3 direction = vec3(-1.0, 0.5, 2.0);

    float light = 0.1 + clamp(dot(normal, direction), 0.2, 0.8);

    vec3 color = vec3(1.0, 1.0, 1.0);
    gl_FragColor = vec4(light * color, 1.0);
}
"#;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let gl = create_context()?;

    gl.enable(WebGlRenderingContext::DEPTH_TEST);

    let shader = Shader::create(&gl, MESH_VERTEX_SHADER, MESH_FRAGMENT_SHADER)?;
    gl.bind_attrib_location(&shader.program, 0, "position");
    gl.bind_attrib_location(&shader.program, 1, "texture_coords");
    gl.bind_attrib_location(&shader.program, 2, "normal");
    let _ = shader.link(&gl);

    gl.use_program(Some(&shader.program));

    let mesh = Mesh::create(&gl, &cube::VERTICES, &cube::INDICES)?;

    let projections_matrix_uniform = shader.get_uniform_location(&gl, "projection_matrix");
    let translation_uniform = shader.get_uniform_location(&gl, "translation");

    let mut projection_matrix: Mat4 = glm::identity();
    let mut matrix_array = [0.; 16];

    let draw_function = Rc::new(RefCell::new(None));
    let draw_function_clone = draw_function.clone();

    let (r, g, b) = BACKGROUND_COLOR;
    gl.clear_color(r, g, b, 1.0);

    *draw_function_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        resize(&gl, &mut projection_matrix);

        matrix_array.copy_from_slice(projection_matrix.as_slice());
        gl.uniform_matrix4fv_with_f32_array(
            projections_matrix_uniform.as_ref(),
            false,
            &mut matrix_array,
        );

        let gl = create_context().unwrap();

        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        let offset = (-10.0, 20.0);

        for row in 0..10 {
            for col in 0..32 {
                if cat::BITMAP[row * 32 + col] == 0 {
                    continue;
                }

                gl.uniform3f(
                    translation_uniform.as_ref(),
                    0.0,
                    offset.1 - 2.0 * row as f32,
                    offset.0 + 2.0 * col as f32,
                );

                gl.draw_elements_with_i32(
                    WebGlRenderingContext::TRIANGLES,
                    mesh.index_count,
                    WebGlRenderingContext::UNSIGNED_SHORT,
                    0,
                );
            }
        }

        request_animation_frame(draw_function.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(draw_function_clone.borrow().as_ref().unwrap());

    Ok(())
}

fn resize(gl: &WebGlRenderingContext, projection_matrix: &mut glm::Mat4) {
    let canvas = &gl
        .canvas()
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let display_width = canvas.client_width() as u32;
    let display_height = canvas.client_height() as u32;

    if canvas.width() != display_width || canvas.height() != display_height {
        canvas.set_width(display_width);
        canvas.set_height(display_height);

        gl.viewport(0, 0, display_width as i32, display_height as i32);
    }

    let aspect_ratio = display_height as f32 / display_width as f32;

    let half_size = 50.0;

    *projection_matrix = glm::ortho(
        -half_size,
        half_size,
        -half_size * aspect_ratio,
        half_size * aspect_ratio,
        -1000.0,
        1000.0,
    );

    *projection_matrix = glm::rotate_x(projection_matrix, 0.615);
    *projection_matrix = glm::rotate_y(projection_matrix, glm::quarter_pi());
}
