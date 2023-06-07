use crate::primitives::Primitive;

 /// Universal format specifier for vertex attributes
pub struct Format {
    component_count: usize, // valid values: 1, 2, 3, 4
    component_size: usize, // valid values: 1, 2, 4, 8
    gl_type: gl::types::GLenum, // valid only for integer types
    normalize: bool,
}

struct Const<const N: usize>;

unsafe trait ComponentCount {}
unsafe impl ComponentCount for Const<1> { }
unsafe impl ComponentCount for Const<2> { }
unsafe impl ComponentCount for Const<3> { }
unsafe impl ComponentCount for Const<4> { }

// fixme: this division is not accurate. Each type has one or more glsl types to which it can be converted
//  I want the following declarative code to work
//  ```
//  let attr1 = [[1, 1, 1, 1], [2, 2, 2, 2]].glsl::<Float / Integer>::()
//  ```
//  user can determine the type per variable basis.
//  What I currently have are the glsl types to which cpu types can be converted.

// Type level implementation of logical division of primitive types
unsafe trait NumericKind: Primitive { }
unsafe trait SinglePrecision: Primitive { }
unsafe trait DoublePrecision: Primitive { }
unsafe trait Integer: Primitive { }

unsafe impl Integer for i8  { }
unsafe impl Integer for i16 { }
unsafe impl Integer for i32 { }
unsafe impl Integer for u8  { }
unsafe impl Integer for u16 { }
unsafe impl Integer for u32 { }

unsafe impl SinglePrecision for f32 { }

unsafe impl DoublePrecision for f64 { }

unsafe impl<T> NumericKind for T where T: SinglePrecision { }
unsafe impl<T> NumericKind for T where T: DoublePrecision { }
unsafe impl<T> NumericKind for T where T: Integer         { }

// region FromType / TypeInto
// todo: Can target be associated type? i.e. should objects have only one valid target?
//  well it makes sense for these traits to ge generic but maybe they are to general for
//  this particular use case. Maybe create new local traits that use these traits underneath
//  but themselves expose associated type instead.

/// Create instance of `T` only from type information.
pub trait FromType<T>: Sized /* Obviously not object safe */{
    fn from_type() -> Self;
}

/// Convert `Self` into an instance of `T` based only on type itself.
pub trait TypeInto<T> {
    fn type_into() -> T;
}

/// Blanket impl for types that impl `FromType`.
impl<SourceType, TargetType> TypeInto<TargetType> for SourceType where TargetType: FromType<SourceType> {
    fn type_into() -> TargetType {
        TargetType::from_type::<SourceType>()
    }
}
// endregion

// region Normalized new-type
// todo: test that this is indeed zero cost - if not add impls for [Integer]

/// Transparent wrapper to mark that integer should be marked as normalized during upload.
#[repr(transparent)]
pub struct Normalized<Int>(pub Int) where Int: Integer;

impl<Int> std::ops::Deref for Normalized<Int> where Int: Integer {
    type Target = Int;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<Int> std::ops::DerefMut for Normalized<Int> where Int: Integer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

pub trait Normalize: Integer {
    fn normalized(self) -> Normalized<Self> { Normalized(self) }
}

impl<Int> Normalize for Int where Int: Integer {
    fn normalized(self) -> Normalized<Self> { Normalized(self) }
}
// endregion

// region FromType impls for all kinds of primitives
// todo: either introduce bounds for const generics and impl for valid sizes
//  or enumerate manually all valid sizes
// Format from array of single precision
impl<P, const N: usize> FromType<[P; N]> for Format
where
    P: SinglePrecision,
    Const<N>: ComponentCount
{
    fn from_type() -> Self {
        Self {
            component_count: N,
            component_size: std::mem::size_of::<P>(),
            gl_type: P::GL_TYPE,
            normalize: false,
        }
    }
}

// Format from array of double precision
impl<P, const N: usize> FromType<[P; N]> for Format
where
    P: DoublePrecision,
    Const<N>: ComponentCount
{
    fn from_type() -> Self {
        Self {
            component_count: N,
            component_size: std::mem::size_of::<P>(),
            gl_type: P::GL_TYPE,
            normalize: false,
        }
    }
}

// Format from array of Integers - without normalization
impl<P, const N: usize> FromType<[P; N]> for Format
where
    P: Integer,
    Const<N>: ComponentCount
{
    fn from_type() -> Self {
        Self {
            component_count: N,
            component_size: std::mem::size_of::<P>(),
            gl_type: P::GL_TYPE,
            normalize: false,
        }
    }
}

// Format from array of Integers - with normalization
impl<P, const N: usize> FromType<[Normalized<P>; N]> for Format
where
    P: Integer,
    Const<N>: ComponentCount
{
    fn from_type() -> Self {
        Self {
            component_count: N,
            component_size: std::mem::size_of::<P>(),
            gl_type: P::GL_TYPE,
            normalize: true,
        }
    }
}
// endregion



// region Attribute and public module interface
/// A representation of data that is valid for vertex attribute.
/// Valid in a sense that it can has matching format descriptors for usage in VAO pointer configuration functions.
pub trait Attribute {
    fn format(&self) -> Format;
}

/// Blanket impl for self formatting data.
impl<A> Attribute for A where A: TypeInto<Format> {
    fn format(&self) -> Format {
        <A as TypeInto<Format>>::type_into()
    }
}

/// Attribute array is anything that can be converted into a slice of objects that can provide Format.
pub trait AttributeArray: AsRef<[A]> where A: Attribute {
    fn format(&self) -> Format {
        <P as TypeInto<Format>>::type_into()
    }
}

/// Well fuck, now i cannot use Primitive as bound since not ALL primitive types implement TypeInto<Format>
/// Maybe that's not such a bad thing though - this gives a possibility to internally implement
/// Attribute for all types and their combinations like [P; 2/3/4] and their 2D counter parts.
/// And Still preserve nice and clean facade
impl<T, P> AttributeArray for T where T: AsRef<[Attribute]> {

}
// endregion
