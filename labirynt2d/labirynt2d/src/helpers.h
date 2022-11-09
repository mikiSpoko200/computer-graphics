#pragma once

#include <stdio.h>
#include <assert.h>

#define _CODE_BLOCK_MACRO(...) do { \
__VA_ARGS__ \
} while (0)

#ifdef _DEBUG
#define _DEBUG_MACRO(...) _CODE_BLOCK_MACRO(__VA_ARGS__)
#else
#define _DEBUG_MACRO(...)
#endif

#define DEBUGFMT(fmt, ...) _DEBUG_MACRO( \
	fprintf(stderr, "[LOG  ] :: " fmt, __VA_ARGS__); \
)
#define DEBUGFMTLN(fmt, ...) DEBUGFMT(fmt "\n", __VA_ARGS__)

#define ERRORFMT(fmt, ...) _CODE_BLOCK_MACRO( \
	fprintf(stderr, "[ERROR] || " fmt, __VA_ARGS__); \
)
#define ERRORFMTLN(fmt, ...) ERRORFMT(fmt "\n", __VA_ARGS__)

#define DEBUG(msg) DEBUGFMT("%s", msg)
#define DEBUGLN(msg) DEBUG(msg "\n")

#define ERROR(msg) ERRORFMT("%s", msg)
#define ERRORLN(msg) ERROR(msg "\n")

#define ASSERT(exp) assert(exp)
#define ASSERTEQ(lhs, rhs) ASSERT((lhs) == (rhs))

#define DEBUGASSERT(exp) _DEBUG_MACRO(ASSERT(exp);)
#define DEBUGASSERTEQ(lhs, rhs) DEBUGASSERT((lhs) == (rhs))

#ifdef __GLEW_H__
#define ASSERTGL() assert(glGetError() == GL_NO_ERROR)

#define ERRORGL(msg) _CODE_BLOCK_MACRO( \
	GLenum er; \
	while (er = glGetError()) ERRORFMTLN("OpenGL error: 0x%04x **%s**", er, msg); \
) 

#define DEBUGASSERTGL() _DEBUG_MACRO(ASSERTGL();)

#else
#define ASSERTGL()
#define DEBUGASSERTGL()
#define ERRORGL(msg)
#endif

#ifdef LOG_VERBOSE
#define VDEBUGLN(msg) DEBUGLN(msg)
#define VDEBUGFMTLN(fmt, ...) DEBUGFMTLN(fmt, __VA_ARGS__)
#else
#define VDEBUGLN(msg)
#define VDEBUGFMTLN(fmt, ...)
#endif