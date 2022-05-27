extern crate gl;
use gl::types::{GLenum, GLuint, GLint, GLchar};

use std::ffi::CString;
use std::ptr;
use std::str;

pub struct Shader {
	pub id: GLuint
}

// http://nercury.github.io/rust/opengl/tutorial/2018/02/10/opengl-in-rust-from-scratch-03-compiling-shaders.html
// https://gist.github.com/simias/c140d1479ada4d6218c0

impl Shader {
	pub fn	load(
		src: &str,
		kind: GLenum
	) -> Shader {
		let id;
		unsafe {
			id = gl::CreateShader(kind);

			// Attempt to compile the shader
			let c_str = CString::from_slice(src.as_bytes());
			gl::ShaderSource(id, 1, &c_str.as_ptr(), ptr::null());
			gl::CompileShader(id);
	
			// Get the compile status
			let mut status = gl::FALSE as GLint;
			gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);
	
			// Fail on error
			if status != (gl::TRUE as GLint) {
				let mut len = 0;
				gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
				let mut buf = Vec::new();
				buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
				gl::GetShaderInfoLog(id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
				panic!("{}", str::from_utf8(buf.as_slice()).ok().expect("ShaderInfoLog not valid utf8"));
			}
		}
		Shader { id }
	}

    fn load_vert(src: &str) -> Shader {
        Shader::load(src, gl::VERTEX_SHADER)
    }

    fn load_frag(src: &str) -> Shader {
        Shader::load(src, gl::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}