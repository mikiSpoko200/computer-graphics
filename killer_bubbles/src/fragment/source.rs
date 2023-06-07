


pub struct TextureParameters {

}

pub mod filtering {
    pub mod parameters {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Mode {
            Nearest,
            Bilinear,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Type {
            Mipmap { intra: Mode, inter: Mode },
            Anisotropic
        }
    }

    pub mod edge_value_sampling {
        pub enum EdgeValues {
            Repeat,
            MirroredRepeat,
            ClampToEdge,
            ClampToBorder { color: todo!() },
            MirrorClampToEdge,
        }
    }
    // the above vs

    pub trait EdgeValueSamplingMode { }

    struct Repeat;
    impl EdgeValueSamplingMode for Repeat { }


}

pub struct FilteringParameters {
    mode: filtering::parameters::Mode,
    edge_values: filtering::edge_value_sampling::EdgeValues,

}

pub struct SamplingParameters {

}

pub struct TextureFormat {
    pub format: usize,  // counterpart for arity ??
    pub gl_type: GLenum,
}

// Specifies the format of the pixel data. The following symbolic values are accepted:
// GL_RED, GL_RG, GL_RGB, GL_BGR, GL_RGBA, GL_BGRA, GL_RED_INTEGER, GL_RG_INTEGER, GL_RGB_INTEGER,
// GL_BGR_INTEGER, GL_RGBA_INTEGER, GL_BGRA_INTEGER, GL_STENCIL_INDEX,
// GL_DEPTH_COMPONENT, GL_DEPTH_STENCIL.
pub trait DataFormat {

}

pub struct Foo;

pub trait Texture<DataFormat> {
    fn get_texture_parameters(&self) -> &TextureParameters;

    fn get_sampling_parameters(&self) -> &SamplingParameters;
}

impl TextureConfig {
    pub fn new() -> Self {
        todo!()
    }

    pub fn with_sampler() -> Self {
        todo!()
    }
}

// a source of samplable values
pub trait Source {

    fn pixel_format();

    // place in gl context where this source will be bound
    fn gl_target();
}