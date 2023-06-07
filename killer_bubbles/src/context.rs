

pub mod targets {
    use gl::types::GLenum;
    // make this sealed trait
    pub(crate) unsafe trait Target {
        const BIND_TARGET: GLenum;
    }

    macro_rules! impl_target {
        ($target_type:ty, $target_module:ident, $t:ident) => {
            unsafe impl crate::context::targets::Target for $target_type {
                const BIND_TARGET: crate::gl::types::GLenum = crate::gl::$t;
            }
            impl crate::context::targets::$target_module::Target for $target_type { }
        };
    }

    // Buffer object targets: https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBindBuffer.xhtml
    // module buffer instead of _BUFFER suffix present in C
    pub mod buffer {
        use gl::types::GLenum;

        // A target marker for a buffer object
        pub trait Target: super::Target { }

        // todo: replace manual impls with macros
        // impl_target!(Array, ARRAY_BUFFER);

        pub struct Array;
        pub struct AtomicCounter;
        pub struct CopyRead;
        pub struct CopyWrite;
        pub struct DispatchIndirect;
        pub struct DrawIndirect;
        pub struct ElementArray;
        pub struct PixelPack;
        pub struct PixelUnpack;
        pub struct Query;
        pub struct ShaderStorage;
        pub struct Texture;
        pub struct TransformFeedback;
        pub struct Uniform;

        // impl_target!(Array, ARRAY_BUFFER);
        // impl_target!(AtomicCounter, ATOMIC_COUNTER_BUFFER);

        // impl_target!(Array, buffer, ARRAY_BUFFER);
        unsafe impl super::Target for Array      { const BIND_TARGET: GLenum = gl::ARRAY_BUFFER; }
        impl Target for Array { }

        unsafe impl Target for AtomicCounter     { const BIND_TARGET: GLenum = gl::ATOMIC_COUNTER_BUFFER; }
        unsafe impl Target for CopyRead          { const BIND_TARGET: GLenum = gl::COPY_READ_BUFFER; }
        unsafe impl Target for CopyWrite         { const BIND_TARGET: GLenum = gl::COPY_WRITE_BUFFER; }
        unsafe impl Target for DispatchIndirect  { const BIND_TARGET: GLenum = gl::DISPATCH_INDIRECT_BUFFER;}
        unsafe impl Target for DrawIndirect      { const BIND_TARGET: GLenum = gl::DRAW_INDIRECT_BUFFER;}
        unsafe impl Target for ElementArray      { const BIND_TARGET: GLenum = gl::ELEMENT_ARRAY_BUFFER;}
        unsafe impl Target for PixelPack         { const BIND_TARGET: GLenum = gl::PIXEL_PACK_BUFFER;}
        unsafe impl Target for PixelUnpack       { const BIND_TARGET: GLenum = gl::PIXEL_UNPACK_BUFFER;}
        unsafe impl Target for Query             { const BIND_TARGET: GLenum = gl::QUERY_BUFFER;}
        unsafe impl Target for ShaderStorage     { const BIND_TARGET: GLenum = gl::SHADER_STORAGE_BUFFER;}
        unsafe impl Target for Texture           { const BIND_TARGET: GLenum = gl::TEXTURE_BUFFER;}
        unsafe impl Target for TransformFeedback { const BIND_TARGET: GLenum = gl::TRANSFORM_FEEDBACK_BUFFER;}
        unsafe impl Target for Uniform           { const BIND_TARGET: GLenum = gl::UNIFORM_BUFFER;}
    }

    // https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBindTexture.xhtml
    pub mod texture {
        use super::Target;
        use gl::types::GLenum;

        struct Dim<const N: usize>;
        struct Array<T: Target>;
        struct MultiSample<T: Target>;

        struct Rectangle;
        struct CubeMap;
        struct Buffer;

        unsafe impl Target for Dim<1>                     { const BIND_TARGET: GLenum = gl::TEXTURE_1D; }
        unsafe impl Target for Dim<2>                     { const BIND_TARGET: GLenum = gl::TEXTURE_2D; }
        unsafe impl Target for Dim<3>                     { const BIND_TARGET: GLenum = gl::TEXTURE_3D; }
        unsafe impl Target for Array<Dim<1>>              { const BIND_TARGET: GLenum = gl::TEXTURE_1D_ARRAY; }
        unsafe impl Target for Array<Dim<2>>              { const BIND_TARGET: GLenum = gl::TEXTURE_2D_ARRAY; }
        unsafe impl Target for Rectangle                  { const BIND_TARGET: GLenum = gl::TEXTURE_RECTANGLE; }
        unsafe impl Target for CubeMap                    { const BIND_TARGET: GLenum = gl::TEXTURE_CUBE_MAP; }
        unsafe impl Target for Array<CubeMap>             { const BIND_TARGET: GLenum = gl::TEXTURE_CUBE_MAP_ARRAY; }
        unsafe impl Target for Buffer                     { const BIND_TARGET: GLenum = gl::TEXTURE_BUFFER; }

        // 3.2+
        unsafe impl Target for MultiSample<Dim<2>>        { const BIND_TARGET: GLenum = gl::TEXTURE_2D_MULTISAMPLE; }
        unsafe impl Target for Array<MultiSample<Dim<2>>> { const BIND_TARGET: GLenum = gl::TEXTURE_2D_MULTISAMPLE_ARRAY; }
    }

    // todo(structure): for frame_buffer and render_buffer maybe replace modules with types
    //  pros: more concise path - just FrameBuffer instead of frame_buffer::FrameBuffer
    //  cons: not consistent for all bind targets - buffer:: and texture::

    // todo(rename): or maybe fremebuffer?
    pub mod frame_buffer {
        use super::Target;
        use gl::types::GLenum;

        pub unsafe trait AccessPermissions { }

        // todo(style): or maybe mod access { Read, Draw, Both } ?
        pub struct Read;
        pub struct Draw;
        pub struct ReadDraw;

        unsafe impl AccessPermissions for Read { }
        unsafe impl AccessPermissions for Draw { }
        unsafe impl AccessPermissions for ReadDraw { }

        struct FrameBuffer<P: AccessPermissions = ReadDraw>;
        unsafe impl Target for FrameBuffer       { const BIND_TARGET: GLenum = gl::FRAMEBUFFER; }
        unsafe impl Target for FrameBuffer<Read> { const BIND_TARGET: GLenum = gl::READ_FRAMEBUFFER; }
        unsafe impl Target for FrameBuffer<Draw> { const BIND_TARGET: GLenum = gl::DRAW_FRAMEBUFFER; }
    }

    // todo(rename): or maybe renderbuffer?
    pub mod render_buffer {
        use super::Target;
        use gl::types::GLenum;

        pub struct RenderBuffer;
        unsafe impl Target for RenderBuffer { const BIND_TARGET: GLenum = gl::RENDERBUFFER; }
    }

    // note(polymorphism): objects that have only one viable bind target should also provide
    //  types that represent that bind target so polymorphism like T: context::targets::Target
    //  would accept them


}