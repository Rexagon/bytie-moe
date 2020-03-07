use wasm_bindgen::prelude::*;
use web_sys::{WebGlBuffer, WebGlRenderingContext};

pub struct Mesh {
    pub index_count: i32,
}

impl Mesh {
    pub fn create(
        gl: &WebGlRenderingContext,
        vertices: &[f32],
        indices: &[u16],
    ) -> Result<Self, JsValue> {
        let vertex_buffer = gl.create_buffer().ok_or("failed to create buffer")?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

        unsafe {
            let view = js_sys::Float32Array::view(&vertices);

            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &view,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let float_size = std::mem::size_of::<f32>() as i32;
        let stride = 8 * float_size;

        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(0);

        gl.vertex_attrib_pointer_with_i32(
            1,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            stride,
            3 * float_size,
        );
        gl.enable_vertex_attrib_array(1);

        gl.vertex_attrib_pointer_with_i32(
            2,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            stride,
            5 * float_size,
        );
        gl.enable_vertex_attrib_array(2);

        let index_buffer = gl.create_buffer().ok_or("failed to create buffer")?;
        gl.bind_buffer(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&index_buffer),
        );

        unsafe {
            let view = js_sys::Uint16Array::view(&indices);

            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                &view,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        Ok(Mesh {
            index_count: indices.len() as i32,
        })
    }
}
