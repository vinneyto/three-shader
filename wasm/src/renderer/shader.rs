use super::define::Define;
use anyhow::{anyhow, Result};
use log::warn;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub struct Shader {
  pub gl: WebGl2RenderingContext,
  pub program: WebGlProgram,
}

impl Shader {
  pub fn new(
    gl: &WebGl2RenderingContext,
    vertex_src: &str,
    fragment_src: &str,
    defines: &[Define],
  ) -> Result<Shader> {
    let vert = add_header(vertex_src, defines, false);
    let frag = add_header(fragment_src, defines, true);

    let vert_shader = compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, &vert)?;
    let frag_shader = compile_shader(gl, WebGl2RenderingContext::FRAGMENT_SHADER, &frag)?;

    let program = link_program(&gl, &vert_shader, &frag_shader)?;

    Ok(Shader {
      gl: gl.clone(),
      program,
    })
  }
}

pub fn compile_shader(
  gl: &WebGl2RenderingContext,
  shader_type: u32,
  source: &str,
) -> Result<WebGlShader> {
  let shader = gl
    .create_shader(shader_type)
    .ok_or_else(|| anyhow!("Unable to create shader object"))?;
  gl.shader_source(&shader, source);
  gl.compile_shader(&shader);

  if gl
    .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(shader)
  } else {
    let message = gl
      .get_shader_info_log(&shader)
      .unwrap_or_else(|| String::from("Unknown error creating shader"));

    warn!(
      "{}",
      &format!("\n{}\n\n{}\n", message, add_row_numbers(source))
    );

    Err(anyhow!("shader compile error"))
  }
}

pub fn link_program(
  gl: &WebGl2RenderingContext,
  vert_shader: &WebGlShader,
  frag_shader: &WebGlShader,
) -> Result<WebGlProgram> {
  let program = gl
    .create_program()
    .ok_or_else(|| anyhow!("Unable to create shader object"))?;

  gl.attach_shader(&program, vert_shader);
  gl.attach_shader(&program, frag_shader);
  gl.link_program(&program);

  if gl
    .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(program)
  } else {
    Err(anyhow!(gl.get_program_info_log(&program).unwrap_or_else(
      || String::from("Unknown error creating program object")
    )))
  }
}

pub fn add_row_numbers(src: &str) -> String {
  let v: Vec<&str> = src.split('\n').collect();
  let mut result: String = String::from("");
  for (i, row) in v.iter().enumerate() {
    result.push_str(&format!("{}  {}\n", i + 1, row));
  }
  result
}

pub fn add_header(src: &str, defines: &[Define], with_precision: bool) -> String {
  let mut result = String::from("");
  if with_precision {
    result.push_str("precision highp float;\n\n");
  }
  if !defines.is_empty() {
    for define in defines {
      result.push_str(&format!("{}\n", &define.as_string()));
    }
    result.push_str("\n");
  }
  result.push_str(src);
  result
}
