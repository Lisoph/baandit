use gl_generator as gl;

use std::fs::File;
use std::path::Path;
use std::env;

fn main() {
	let out = env::var("OUT_DIR").unwrap();
	let out = Path::new(&out).join("gl_bindings.rs");
	if !out.exists() {
		let mut file = File::create(&out).unwrap();
		gl::Registry::new(gl::Api::Gl, (3, 3), gl::Profile::Core, gl::Fallbacks::All, [])
			.write_bindings(gl::StructGenerator, &mut file)
			.unwrap();
	}
}