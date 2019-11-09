use web_sys::{WebGlRenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};
use wasm_bindgen::prelude::*;


pub struct Shader {
    pub program: WebGlProgram,
    vertex_shader: WebGlShader,
    fragment_shader: WebGlShader,
}


impl Shader {
    pub fn create(gl: &WebGlRenderingContext, vertex_shader: &str, fragment_shader: &str)
                  -> Result<Self, JsValue>
    {
        let vertex_shader = compile_shader(
            &gl,
            WebGlRenderingContext::VERTEX_SHADER,
            vertex_shader)?;

        let fragment_shader = compile_shader(
            &gl,
            WebGlRenderingContext::FRAGMENT_SHADER,
            fragment_shader)?;

        let program = gl
            .create_program()
            .expect("Unable to create shader object");

        Ok(Shader {
            program,
            vertex_shader,
            fragment_shader,
        })
    }


    pub fn get_uniform_location(&self, gl: &WebGlRenderingContext, name: &str) -> Option<WebGlUniformLocation> {
        gl.get_uniform_location(&self.program, name)
    }


    pub fn link(&self, gl: &WebGlRenderingContext) -> Result<(), String> {
        gl.attach_shader(&self.program, &self.vertex_shader);
        gl.attach_shader(&self.program, &self.fragment_shader);
        gl.link_program(&self.program);

        if gl
            .get_program_parameter(&self.program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            return Ok(());
        }

        Err(gl
            .get_program_info_log(&self.program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}


fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}
