mod shader;
mod stuff;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader, HtmlCanvasElement, Element};

use shader::Shader;
use stuff::{create_context, request_animation_frame};
use crate::stuff::document;


const MESH_VERTEX_SHADER: &'static str = r#"
attribute vec4 position;

void main() {
    gl_Position = position;
}
"#;

const MESH_FRAGMENT_SHADER: &'static str = r#"
void main() {
    gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
"#;


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let gl = create_context()?;

    let shader = Shader::create(&gl, MESH_VERTEX_SHADER, MESH_FRAGMENT_SHADER)?;

    gl.use_program(Some(&shader.program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    let draw_function = Rc::new(RefCell::new(None));
    let draw_function_clone = draw_function.clone();

    gl.clear_color(0.933, 0.933, 0.933, 1.0);

    *draw_function_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        resize(&gl.canvas().unwrap().dyn_into::<HtmlCanvasElement>().unwrap()).unwrap();


        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        gl.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (vertices.len() / 3) as i32,
        );

        request_animation_frame(draw_function.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(draw_function_clone.borrow().as_ref().unwrap());

    Ok(())
}


fn resize(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let display_width = canvas.client_width() as u32;
    let display_height = canvas.client_height() as u32;

    if canvas.width() != display_width || canvas.height() != display_height {
        canvas.set_width(display_width);
        canvas.set_height(display_height);
    }

    Ok(())
}
