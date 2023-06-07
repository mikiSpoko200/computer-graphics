use super::{Primitive, VertexAttribute, AttributeType};
use gl;
use gl::types::GLuint;

use crate::{gl_assert, gl_assert_no_err};
use crate::index_buffer::{GlBufferTargetProvider};
use crate::vertex::attribute::{Attribute, VertexFormat};

// note: dual purpose of the VertexAttribPointer - both describes the data AND links the attribute
//  being configured with shader input.
//  The glVertexAttribPointer function in OpenGL is used to specify the structure
//  and layout of vertex attribute data stored in a buffer object.
//  It tells OpenGL how to interpret the data and how to bind it to a vertex shader attribute.

use crate::context::targets::buffer;
use crate::object;

pub mod types {
    // note: only vertex attributes need to be self descriptive! i.e targeting GL_ARRAY_BUFFER and these can be:
    //  > Vertex Attributes in the Vertex Shader can be declared as a floating-point GLSL type (such as float or vec4),
    //  > an integral type (such as uint or ivec3), or a double-precision type (such as double or dvec4).
    //  > Double-precision attributes are only available in OpenGL 4.1 or ARB_vertex_attrib_64bit.
    // primitive types that can be used as input for buffer object and vertex processor.
    pub unsafe trait Primitive: Clone + Copy + Sized + Sync { }
}

mod vao {
    use crate::vertex::buffer::types::Primitive;
    
    // wouldn't that be nice? Similar to Borrow / AsRef
    pub trait Provider {
        type Target<T>;


    }

    // format requires: GL primitive type enum + known size

    pub mod vertex {
        // These types can be sources for vertex shader
        // This can require type to be attribute
        pub trait Source: AsRef<[u8]> {
            fn format(&self) -> super::attribute::Format
        }


    }

    pub mod attribute {
        use std::marker::PhantomData;
        use crate::vertex::buffer::types::Primitive;
        use crate::vertex::buffer::vao::ComponentCount;

        /// Universal format specifier for vertex attributes


        // From trait does not fit here - some other custom trait FromType?
        impl<P> From<&[P; 1]> for Format<P> where P: Primitive {
            fn from(_: &[P; 1]) -> Self {
                Self { components: ComponentCount::One, normalize: false, _primitive: PhantomData::<P>::default() }
            }
        }

        impl<P> Format<P> where P: Primitive {

        }
    }

    pub trait Attribute<P> where P: Primitive {

    }

    fn format<P>() where P: Primitive {

    }

    // state that is normally stored in VAO
    struct VertexArrayConfig {

    }

    // make Attribute a GAT?
    pub trait AttributeArray {
        type Store<P>: AsRef<[P]>;

        fn format() -> attribute::Fo
    }

    impl<A> Provider<VertexArrayConfig> for [A] where A: Attribute {

    }
}



pub struct GlslAttributeContext<'name> {
    location: usize,
    name: Option<&'name str>,
    // some more here: https://www.khronos.org/opengl/wiki/Layout_Qualifier_(GLSL)#Fragment_shader_buffer_output
}

pub struct AttributeArray {
    gl_object: ()
}

// internal data structure that associates attribute with parameters
pub(crate) struct AttributeConfig<'a> {
    pub attribute: &'a dyn Source,
    pub instancing: usize, // info about instancing
    pub glsl_attribute_context: GlslAttributeContext<'a>
}

// this article is gold!!!
// note: https://www.khronos.org/opengl/wiki/Vertex_Rendering#Multi-Draw

pub struct VertexBufferBuilder { }

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
