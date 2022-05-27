pub struct Program {
    pub id: gl::types::GLuint,
}


impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id); }
        }

        unsafe { gl::LinkProgram(program_id); }

        // continue with error handling here

        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id); }
        }

        Ok(Program { id: program_id })
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}