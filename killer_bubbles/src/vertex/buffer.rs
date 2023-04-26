use super::{Primitive, VertexAttribute, AttributeType};
use gl;
use gl::types::GLuint;

use crate::{gl_assert, gl_assert_no_err};
use crate::index_buffer::{GlBufferTargetProvider};
use crate::vertex::attribute::{Attribute, VertexFormat};

// note: https://www.khronos.org/opengl/wiki/Debug_Output
//   article about the debugging capabilities of opengl - integrate message severity with
//   logging levels and context type with cargo build type (find out if the cfg(debug) is possible)

pub trait GlObjectName {
    type Name;

    // note: https://www.khronos.org/opengl/wiki/OpenGL_Object/Object_Name
    //  strings can be associated with system generated object names - use it in debug mode?

    fn name(&self) -> Self::Name;
}

impl GlObjectName for usize {
    type Name = usize;

    fn name(&self) -> Self::Name {
        *self
    }
}

todo!(pub unsafe configure_vertex_array_pointer(vertex_format, ));

/// A handle to opengl object that manages it bindings and frees any GPU memory on Drop.
/// todo: rename
pub struct Handle {
    name: usize // Rc<dyn GlObjectName>
}

pub struct GlslAttributeContext<'name> {
    location: usize,
    name: Option<&'name str>,
    // some more here: https://www.khronos.org/opengl/wiki/Layout_Qualifier_(GLSL)#Fragment_shader_buffer_output
}

pub struct AttributeArray {
    gl_object: Handle
}

// internal data structure that associates attribute with parameters
pub(crate) struct AttributeConfig<'a> {
    pub attribute: &'a dyn Source,
    pub instancing: usize, // info about instancing
    pub glsl_attribute_context: GlslAttributeContext<'a>
}

// this article is gold!!!
// note: https://www.khronos.org/opengl/wiki/Vertex_Rendering#Multi-Draw

pub struct VertexBufferBuilder {}

pub struct VertexBuffer<'attrs> {
    // note: here comes type erasure for the attribute streams
    //   all that is needed is already stored in VertexFormat
    // todo: what about instancing?
    // fixme: assert(len <  GL_MAX_VERTEX_ATTRIBS)
    attributes: Box<[AttributeConfig<'attrs>]>
}

impl<A> From<&[A]> for VertexBuffer<'_> where A: Attribute {
    fn from(attrs: &[A]) -> Self {


    }
}

impl VertexBuffer<'_> {
    // todo: move all such consts into lazy_static!
    const MAX_VERTEX_ATTRIBUTE_COUNT: usize = todo!();
}

impl TryFrom<&[& dyn Attribute]> for VertexBuffer<'_> {
    type Error = (); // vertex buffer creation error

    fn try_from(attrs: &[&dyn Attribute]) -> Self {
        // this may fail when attrs.len() > MAX_VERTEX_ATTRIBUTE_COUNT
        todo!();
    }
}



pub struct FrameBuffer {

}



/// Source of vertex attributes ? what about textures?
/// todo: rename
pub trait Source {
    fn data(&self) -> &[u8];

    fn format(&self) -> VertexFormat;
}

impl<T, P, const N: usize> Source for T
where
    T: AsRef<[P; N]>,
    P: Primitive
{
    fn data(&self) -> &[u8] {
        // Safety: data layout information is preserved by format.
        let ([], aligned_bytes, &[]) = unsafe { self.as_ref().align_to::<u8>() } else {
            panic!("Transmutation of slice into bytes not aligned.")
        };
        aligned_bytes
    }

    fn format(&self) -> VertexFormat {
        <T as Attribute>::format()
    }
}
