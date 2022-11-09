// ==========================================================================
// AGL3:  GL/GLFW init AGLWindow and AGLDrawable class definitions
//
// Ver.3  14.I.2020 (c) A. Łukaszewski
// ==========================================================================
// AGLDrawable
//===========================================================================
#include <vector>
#include <iostream>
#include <string>
#include <fstream>

#include <GL/glew.h>

#include "helpers.h"
#include "types.h"

namespace components {

template <usize BUFFER_COUNT = 1>
struct IDrawable {
protected:
	virtual void register_shaders() = 0;
	virtual void register_buffers() = 0;
	virtual void draw() = 0;
	virtual void bind_vao() = 0;
	virtual void unbind_vao() = 0;
	virtual void bind_shaders() = 0;

public:
	virtual ~IDrawable() {}

	void start() {
		DEBUGLN("Drawable: registering shaders");
		this->register_shaders();
		DEBUGLN("Drawable: binding shaders");
		this->bind_shaders();
		DEBUGLN("Drawable: binding vao");
		this->bind_vao();
		DEBUGLN("Drawable: registering buffers");
		this->register_buffers();
		DEBUGLN("Drawable: unbinding vao");
		this->unbind_vao();
	}

	virtual void draw_call() final {
		DEBUGLN("Drawable: binding shader");
		this->bind_shaders();
		DEBUGLN("Drawable: binding vao");
		this->bind_vao();
		DEBUGLN("Drawable: issuing draw call");
		this->draw();
		DEBUGLN("Drawable: unbinding vao");
		this->unbind_vao();
	}
};

template <usize BUFFER_COUNT = 1>
struct StateManager {
	StateManager(GLuint program_id = 0) : program_id(program_id) {
		DEBUGLN("Initializing OpenGL state manager");
		glGenVertexArrays(1, &this->vaoId);
		if (BUFFER_COUNT > 0) {
			glGenBuffers(BUFFER_COUNT, this->vbos.data());
		}
		DEBUGFMTLN("GL state: vao %d, vbo count: %lu, vbo ids:", vaoId, BUFFER_COUNT);
		for (GLuint buffer_id : this->vbos) {
			DEBUGFMTLN("  %d", buffer_id);
		}
	}

	~StateManager() {
		DEBUGLN("GL Manager: Destroying OpenGL state manager");
		glDeleteBuffers(BUFFER_COUNT, this->vbos.data());
		glDeleteVertexArrays(1, &this->vaoId);
		if (this->program_id) glDeleteProgram(this->program_id);
	}

	int compile_shaders(const char* vs, const char* fs, const char* gs = NULL) {
		DEBUGLN("GL Manager: Compiling shaders");

		GLuint  v = glCreateShader(GL_VERTEX_SHADER);
		GLuint  f = glCreateShader(GL_FRAGMENT_SHADER);
		GLuint  g = 0;
		if (gs) g = glCreateShader(GL_GEOMETRY_SHADER);
		glShaderSource(v, 1, &vs, NULL);   // Also read from file: next fun
		glShaderSource(f, 1, &fs, NULL);   // ...
		if (gs) glShaderSource(g, 1, &gs, NULL);   // ...

		int res = compile_shaders(v, f, g);
		glUseProgram(this->get_program_id());
		return res;
	}

	int compile_shaders_from_file(const char* vs, const char* fs, const char* gs = NULL) {
		GLuint v = glCreateShader(GL_VERTEX_SHADER);
		GLuint f = glCreateShader(GL_FRAGMENT_SHADER);
		GLuint g = 0;

		if (gs) g = glCreateShader(GL_GEOMETRY_SHADER);
		get_shader_source(v, vs);
		get_shader_source(f, fs);
		if (gs) get_shader_source(g, gs);

		int res = compile_shaders(v, f, g);
		glUseProgram(this->get_program_id());
		return res;
	}

	void bind_vao() {
		DEBUGFMTLN("GL Manager: binding vao %d", this->vaoId);
		glBindVertexArray(vaoId);
	}

	void unbind_vao() {
		DEBUGFMTLN("GL Manager: unbinding vao %d", this->vaoId);
		glBindVertexArray(0);
	}

	template <usize BUFFER_INDEX = 0>
	void bind_buffer() {
		static_assert(BUFFER_INDEX < BUFFER_COUNT, "Buffer index out of bounds.");
		DEBUGFMTLN("GL Manager: binding buffer: %lu", this->vbos[BUFFER_INDEX]);
		glBindBuffer(GL_ARRAY_BUFFER, this->vbos[BUFFER_INDEX]);
	}

	void bind_program() {
		DEBUGFMTLN("GL Manager: binding program %d", this->program_id);
		glUseProgram(this->program_id);
	}

	GLuint get_program_id() const {
		return this->program_id;
	}

	GLuint vaoId, program_id;
	std::array<GLuint, BUFFER_COUNT> vbos;
private:
	int compile_shaders(GLuint v, GLuint f, GLuint g = 0) {
		GLint Result = GL_FALSE;
		if (g) Result = compile_link(g, "GS");
		if (Result = compile_link(v, "VS")) {
			if (compile_link(f, "FS")) {
				this->program_id = glCreateProgram(); ASSERTGL();
				printf("Program id %d", this->program_id);
				glAttachShader(this->program_id, v); ASSERTGL();
				glAttachShader(this->program_id, f); ASSERTGL();
				if (g) {
					glAttachShader(this->program_id, g); ASSERTGL();
				}
				compile_link(this->program_id, "Linking", 3);
			}
		}
		glDeleteShader(v);
		glDeleteShader(f);
		if (g) glDeleteShader(g);
		return Result;
	}

	GLint compile_link(GLuint v, const char* which, int prog = 0) {
		GLint Result = GL_FALSE;
		int InfoLogLength;
		if (prog) {
			glLinkProgram(v); ASSERTGL();
			glGetProgramiv(v, GL_LINK_STATUS, &Result); ASSERTGL();
			glGetProgramiv(v, GL_INFO_LOG_LENGTH, &InfoLogLength); ASSERTGL();
		}
		else {
			glCompileShader(v); ASSERTGL();
			glGetShaderiv(v, GL_COMPILE_STATUS, &Result); ASSERTGL();
			printf("Shader compilation status: %d", Result);
			glGetShaderiv(v, GL_INFO_LOG_LENGTH, &InfoLogLength); ASSERTGL();
		}
		if (InfoLogLength > 0 && !Result) {
			std::vector<char> Message(InfoLogLength + 1);
			if (prog)
				glGetProgramInfoLog(v, InfoLogLength, NULL, &Message[0]);
			else
				glGetShaderInfoLog(v, InfoLogLength, NULL, &Message[0]);
			printf("%s: %s\n", which, &Message[0]);
		}
		return Result;
	}

	void get_shader_source(GLuint sId, const char* file) {
		std::string sCode;
		std::ifstream sStream(file, std::ios::in);
		if (sStream.is_open()) {
			std::string Line = "";
			while (std::getline(sStream, Line))
				sCode += "\n" + Line;
			sStream.close();
		}
		else {
			printf("Error opening file:  %s !\n", file);
			getchar();
			return;
		}
		char const* SourcePointer = sCode.c_str();
		glShaderSource(sId, 1, &SourcePointer, NULL);
	}
};
}
