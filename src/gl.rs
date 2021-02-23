use std::sync::Arc;

mod bindings {
	include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub use bindings::*;

#[derive(Clone)]
pub struct Gl {
	inner: Arc<bindings::Gl>,
}

impl Gl {
	pub fn load_with(loadfn: impl FnMut(&'static str) -> *const types::GLvoid) -> Self {
		Self {
			inner: Arc::new(bindings::Gl::load_with(loadfn))
		}
	}
}

impl std::ops::Deref for Gl {
	type Target = bindings::Gl;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}