use crate::gl;

use std::borrow::Cow;

pub struct Renderer {
	gl: gl::Gl,
	text_shader: Shader,
}

impl Renderer {
	pub fn new(gl: gl::Gl) -> Self {
		let text_shader = Shader::new(gl.clone());
		Self {
			gl,
			text_shader,
		}
	}

	pub fn update_text_shader(&mut self, update: UpdatedShaderSources) {
		// if let Some(vertex) = update.vertex {
		// 	let srcs = [vertex.as_ptr()];
		// 	let lens = [vertex.len() as gl::GLint];
		// 	self.gl.ShaderSource(self.text_shader.vertex, 1, srcs.as_ptr(), lens.as_ptr());
		// 	self.gl.CompileShader(self.text_shader.vertex);
		// }

		const S: &str = "Updating text shader because";
		match (update.vertex, update.fragment) {
			(Some(_), None) => println!("{} vertex changed.", S),
			(None   , Some(_)) => println!("{} fragment changed.", S),
			(Some(_), Some(_)) => println!("{} both changed.", S),
			(None   , None) => {},
		}
	}

	pub fn frame(&self) {
		unsafe {
			self.gl.ClearColor(0.2, 0.4, 0.8, 1.0);
			self.gl.Clear(gl::COLOR_BUFFER_BIT);
		}
	}
}

struct Shader {
	gl: gl::Gl,
	id: gl::types::GLuint,
	vertex: gl::types::GLuint,
	fragment: gl::types::GLuint,
}

impl Shader {
	fn new(gl: gl::Gl) -> Self {
		let vertex = unsafe { gl.CreateShader(gl::VERTEX_SHADER) };
		let fragment = unsafe { gl.CreateShader(gl::FRAGMENT_SHADER) };
		Self {
			gl,
			id: 0,
			vertex,
			fragment,
		}
	}
}

impl std::ops::Drop for Shader {
	fn drop(&mut self) {
		unsafe {
			self.gl.DeleteShader(self.vertex);
			self.gl.DeleteShader(self.fragment);
		}
	}
}

pub struct UpdatedShaderSources<'a> {
	vertex: Option<&'a str>,
	fragment: Option<&'a str>,
}

impl<'a> std::default::Default for UpdatedShaderSources<'a> {
	fn default() -> Self {
		Self {
			vertex: None,
			fragment: None,
		}
	}
}

enum ShaderKind {
	Vertex,
	Fragment,
}