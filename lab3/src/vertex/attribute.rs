use std::fmt::Debug;
use std::mem;
use crate::gl::{BYTE, UNSIGNED_BYTE, SHORT, UNSIGNED_SHORT, INT, UNSIGNED_INT, FLOAT, DOUBLE};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AttributeType {
    I8,
    I8I8,
    I8I8I8,
    I8I8I8I8,
    U8,
    U8U8,
    U8U8U8,
    U8U8U8U8,
    I16,
    I16I16,
    I16I16I16,
    I16I16I16I16,
    U16,
    U16U16,
    U16U16U16,
    U16U16U16U16,
    I32,
    I32I32,
    I32I32I32,
    I32I32I32I32,
    U32,
    U32U32,
    U32U32U32,
    U32U32U32U32,
    I64,
    I64I64,
    I64I64I64,
    I64I64I64I64,
    U64,
    U64U64,
    U64U64U64,
    U64U64U64U64,
    F16,
    F16F16,
    F16F16F16,
    F16F16F16F16,
    /// 2x2 matrix of `f16`s
    F16x2x2,
    /// 2x3 matrix of `f16`s
    F16x2x3,
    /// 2x3 matrix of `f16`s
    F16x2x4,
    /// 3x2 matrix of `f16`s
    F16x3x2,
    /// 3x3 matrix of `f16`s
    F16x3x3,
    /// 3x4 matrix of `f16`s
    F16x3x4,
    /// 4x2 matrix of `f16`s
    F16x4x2,
    /// 4x3 matrix of `f16`s
    F16x4x3,
    /// 4x4 matrix of `f16`s
    F16x4x4,
    F32,
    F32F32,
    F32F32F32,
    F32F32F32F32,
    /// 2x2 matrix of `f32`s
    F32x2x2,
    /// 2x3 matrix of `f32`s
    F32x2x3,
    /// 2x3 matrix of `f32`s
    F32x2x4,
    /// 3x2 matrix of `f32`s
    F32x3x2,
    /// 3x3 matrix of `f32`s
    F32x3x3,
    /// 3x4 matrix of `f32`s
    F32x3x4,
    /// 4x2 matrix of `f32`s
    F32x4x2,
    /// 4x3 matrix of `f32`s
    F32x4x3,
    /// 4x4 matrix of `f32`s
    F32x4x4,
    /// Warning: using `f64`s can be very slow.
    F64,
    /// Warning: using `f64`s can be very slow.
    F64F64,
    /// Warning: using `f64`s can be very slow.
    F64F64F64,
    /// Warning: using `f64`s can be very slow.
    F64F64F64F64,
    /// 2x2 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x2x2,
    /// 2x3 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x2x3,
    /// 2x3 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x2x4,
    /// 3x2 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x3x2,
    /// 3x3 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x3x3,
    /// 3x4 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x3x4,
    /// 4x2 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x4x2,
    /// 4x3 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x4x3,
    /// 4x4 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x4x4,
    /// From MSB to LSB: two bits for the alpha, ten bits for the blue, ten bits for the green,
    /// ten bits for the red.
    ///
    /// Corresponds to `GL_INT_2_10_10_10_REV`.
    I2I10I10I10Reversed,
    /// From MSB to LSB: two bits for the alpha, ten bits for the blue, ten bits for the green,
    /// ten bits for the red.
    ///
    /// Corresponds to `GL_UNSIGNED_INT_2_10_10_10_REV`.
    U2U10U10U10Reversed,
    /// Corresponds to `GL_INT_10_10_10_2`.
    I10I10I10I2,
    /// Corresponds to `GL_UNSIGNED_INT_10_10_10_2`.
    U10U10U10U2,
    /// Three floating points values turned into unsigned integers./
    ///
    /// Corresponds to `GL_UNSIGNED_INT_10F_11F_11F_REV`.
    F10F11F11UnsignedIntReversed,
    /// Fixed floating points. A 16bits signed value followed by the 16bits unsigned exponent.
    ///
    /// Corresponds to `GL_FIXED`.
    FixedFloatI16U16,
}

impl AttributeType {
    /// Returns the size in bytes of a value of this type.
    pub fn size_bytes(&self) -> usize {
        match *self {
            AttributeType::I8 => 1 * mem::size_of::<i8>(),
            AttributeType::I8I8 => 2 * mem::size_of::<i8>(),
            AttributeType::I8I8I8 => 3 * mem::size_of::<i8>(),
            AttributeType::I8I8I8I8 => 4 * mem::size_of::<i8>(),
            AttributeType::U8 => 1 * mem::size_of::<u8>(),
            AttributeType::U8U8 => 2 * mem::size_of::<u8>(),
            AttributeType::U8U8U8 => 3 * mem::size_of::<u8>(),
            AttributeType::U8U8U8U8 => 4 * mem::size_of::<u8>(),
            AttributeType::I16 => 1 * mem::size_of::<i16>(),
            AttributeType::I16I16 => 2 * mem::size_of::<i16>(),
            AttributeType::I16I16I16 => 3 * mem::size_of::<i16>(),
            AttributeType::I16I16I16I16 => 4 * mem::size_of::<i16>(),
            AttributeType::U16 => 1 * mem::size_of::<u16>(),
            AttributeType::U16U16 => 2 * mem::size_of::<u16>(),
            AttributeType::U16U16U16 => 3 * mem::size_of::<u16>(),
            AttributeType::U16U16U16U16 => 4 * mem::size_of::<u16>(),
            AttributeType::I32 => 1 * mem::size_of::<i32>(),
            AttributeType::I32I32 => 2 * mem::size_of::<i32>(),
            AttributeType::I32I32I32 => 3 * mem::size_of::<i32>(),
            AttributeType::I32I32I32I32 => 4 * mem::size_of::<i32>(),
            AttributeType::U32 => 1 * mem::size_of::<u32>(),
            AttributeType::U32U32 => 2 * mem::size_of::<u32>(),
            AttributeType::U32U32U32 => 3 * mem::size_of::<u32>(),
            AttributeType::U32U32U32U32 => 4 * mem::size_of::<u32>(),
            AttributeType::I64 => 1 * mem::size_of::<i64>(),
            AttributeType::I64I64 => 2 * mem::size_of::<i64>(),
            AttributeType::I64I64I64 => 3 * mem::size_of::<i64>(),
            AttributeType::I64I64I64I64 => 4 * mem::size_of::<i64>(),
            AttributeType::U64 => 1 * mem::size_of::<u64>(),
            AttributeType::U64U64 => 2 * mem::size_of::<u64>(),
            AttributeType::U64U64U64 => 3 * mem::size_of::<u64>(),
            AttributeType::U64U64U64U64 => 4 * mem::size_of::<u64>(),
            AttributeType::F16 => 1 * 2,
            AttributeType::F16F16 => 2 * 2,
            AttributeType::F16F16F16 => 3 * 2,
            AttributeType::F16F16F16F16 => 4 * 2,
            AttributeType::F16x2x2 => 4 * 2,
            AttributeType::F16x2x3 => 6 * 2,
            AttributeType::F16x2x4 => 8 * 2,
            AttributeType::F16x3x2 => 6 * 2,
            AttributeType::F16x3x3 => 9 * 2,
            AttributeType::F16x3x4 => 12 * 2,
            AttributeType::F16x4x2 => 8 * 2,
            AttributeType::F16x4x3 => 12 * 2,
            AttributeType::F16x4x4 => 16 * 2,
            AttributeType::F32 => 1 * mem::size_of::<f32>(),
            AttributeType::F32F32 => 2 * mem::size_of::<f32>(),
            AttributeType::F32F32F32 => 3 * mem::size_of::<f32>(),
            AttributeType::F32F32F32F32 => 4 * mem::size_of::<f32>(),
            AttributeType::F32x2x2 => 4 * mem::size_of::<f32>(),
            AttributeType::F32x2x3 => 6 * mem::size_of::<f32>(),
            AttributeType::F32x2x4 => 8 * mem::size_of::<f32>(),
            AttributeType::F32x3x2 => 6 * mem::size_of::<f32>(),
            AttributeType::F32x3x3 => 9 * mem::size_of::<f32>(),
            AttributeType::F32x3x4 => 12 * mem::size_of::<f32>(),
            AttributeType::F32x4x2 => 8 * mem::size_of::<f32>(),
            AttributeType::F32x4x3 => 12 * mem::size_of::<f32>(),
            AttributeType::F32x4x4 => 16 * mem::size_of::<f32>(),
            AttributeType::F64 => 1 * mem::size_of::<f64>(),
            AttributeType::F64F64 => 2 * mem::size_of::<f64>(),
            AttributeType::F64F64F64 => 3 * mem::size_of::<f64>(),
            AttributeType::F64F64F64F64 => 4 * mem::size_of::<f64>(),
            AttributeType::F64x2x2 => 4 * mem::size_of::<f64>(),
            AttributeType::F64x2x3 => 6 * mem::size_of::<f64>(),
            AttributeType::F64x2x4 => 8 * mem::size_of::<f64>(),
            AttributeType::F64x3x2 => 6 * mem::size_of::<f64>(),
            AttributeType::F64x3x3 => 9 * mem::size_of::<f64>(),
            AttributeType::F64x3x4 => 12 * mem::size_of::<f64>(),
            AttributeType::F64x4x2 => 8 * mem::size_of::<f64>(),
            AttributeType::F64x4x3 => 12 * mem::size_of::<f64>(),
            AttributeType::F64x4x4 => 16 * mem::size_of::<f64>(),
            AttributeType::I2I10I10I10Reversed => 4,
            AttributeType::U2U10U10U10Reversed => 4,
            AttributeType::I10I10I10I2 => 4,
            AttributeType::U10U10U10U2 => 4,
            AttributeType::F10F11F11UnsignedIntReversed => 4,
            AttributeType::FixedFloatI16U16 => 4,
        }
    }

    pub fn component_type(&self) -> gl::types::GLenum {
        match *self {
            AttributeType::I8 => BYTE,
            AttributeType::I8I8 => BYTE,
            AttributeType::I8I8I8 => BYTE,
            AttributeType::I8I8I8I8 => BYTE,
            AttributeType::U8 => UNSIGNED_BYTE,
            AttributeType::U8U8 => UNSIGNED_BYTE,
            AttributeType::U8U8U8 => UNSIGNED_BYTE,
            AttributeType::U8U8U8U8 => UNSIGNED_BYTE,
            AttributeType::I16 => SHORT,
            AttributeType::I16I16 => SHORT,
            AttributeType::I16I16I16 => SHORT,
            AttributeType::I16I16I16I16 => SHORT,
            AttributeType::U16 => UNSIGNED_SHORT,
            AttributeType::U16U16 => UNSIGNED_SHORT,
            AttributeType::U16U16U16 => UNSIGNED_SHORT,
            AttributeType::U16U16U16U16 => UNSIGNED_SHORT,
            AttributeType::I32 => INT,
            AttributeType::I32I32 => INT,
            AttributeType::I32I32I32 => INT,
            AttributeType::I32I32I32I32 => INT,
            AttributeType::U32 => UNSIGNED_INT,
            AttributeType::U32U32 => UNSIGNED_INT,
            AttributeType::U32U32U32 => UNSIGNED_INT,
            AttributeType::U32U32U32U32 => UNSIGNED_INT,
            AttributeType::F32 => FLOAT,
            AttributeType::F32F32 => FLOAT,
            AttributeType::F32F32F32 => FLOAT,
            AttributeType::F32F32F32F32 => FLOAT,
            AttributeType::F32x2x2 => FLOAT,
            AttributeType::F32x2x3 => FLOAT,
            AttributeType::F32x2x4 => FLOAT,
            AttributeType::F32x3x2 => FLOAT,
            AttributeType::F32x3x3 => FLOAT,
            AttributeType::F32x3x4 => FLOAT,
            AttributeType::F32x4x2 => FLOAT,
            AttributeType::F32x4x3 => FLOAT,
            AttributeType::F32x4x4 => FLOAT,
            AttributeType::F64 => DOUBLE,
            AttributeType::F64F64 => DOUBLE,
            AttributeType::F64F64F64 => DOUBLE,
            AttributeType::F64F64F64F64 => DOUBLE,
            AttributeType::F64x2x2 => DOUBLE,
            AttributeType::F64x2x3 => DOUBLE,
            AttributeType::F64x2x4 => DOUBLE,
            AttributeType::F64x3x2 => DOUBLE,
            AttributeType::F64x3x3 => DOUBLE,
            AttributeType::F64x3x4 => DOUBLE,
            AttributeType::F64x4x2 => DOUBLE,
            AttributeType::F64x4x3 => DOUBLE,
            AttributeType::F64x4x4 => DOUBLE,
            _ => panic!("Unsupported type")
        }
    }

    /// Returns the number of values for this type.
    pub fn component_count(&self) -> usize {
        match *self {
            AttributeType::I8 => 1,
            AttributeType::I8I8 => 2,
            AttributeType::I8I8I8 => 3,
            AttributeType::I8I8I8I8 => 4,
            AttributeType::U8 => 1,
            AttributeType::U8U8 => 2,
            AttributeType::U8U8U8 => 3,
            AttributeType::U8U8U8U8 => 4,
            AttributeType::I16 => 1,
            AttributeType::I16I16 => 2,
            AttributeType::I16I16I16 => 3,
            AttributeType::I16I16I16I16 => 4,
            AttributeType::U16 => 1,
            AttributeType::U16U16 => 2,
            AttributeType::U16U16U16 => 3,
            AttributeType::U16U16U16U16 => 4,
            AttributeType::I32 => 1,
            AttributeType::I32I32 => 2,
            AttributeType::I32I32I32 => 3,
            AttributeType::I32I32I32I32 => 4,
            AttributeType::U32 => 1,
            AttributeType::U32U32 => 2,
            AttributeType::U32U32U32 => 3,
            AttributeType::U32U32U32U32 => 4,
            AttributeType::I64 => 1,
            AttributeType::I64I64 => 2,
            AttributeType::I64I64I64 => 3,
            AttributeType::I64I64I64I64 => 4,
            AttributeType::U64 => 1,
            AttributeType::U64U64 => 2,
            AttributeType::U64U64U64 => 3,
            AttributeType::U64U64U64U64 => 4,
            AttributeType::F16 => 1,
            AttributeType::F16F16 => 2,
            AttributeType::F16F16F16 => 3,
            AttributeType::F16F16F16F16 => 4,
            AttributeType::F16x2x2 => 4,
            AttributeType::F16x2x3 => 6,
            AttributeType::F16x2x4 => 8,
            AttributeType::F16x3x2 => 6,
            AttributeType::F16x3x3 => 9,
            AttributeType::F16x3x4 => 12,
            AttributeType::F16x4x2 => 8,
            AttributeType::F16x4x3 => 12,
            AttributeType::F16x4x4 => 16,
            AttributeType::F32 => 1,
            AttributeType::F32F32 => 2,
            AttributeType::F32F32F32 => 3,
            AttributeType::F32F32F32F32 => 4,
            AttributeType::F32x2x2 => 4,
            AttributeType::F32x2x3 => 6,
            AttributeType::F32x2x4 => 8,
            AttributeType::F32x3x2 => 6,
            AttributeType::F32x3x3 => 9,
            AttributeType::F32x3x4 => 12,
            AttributeType::F32x4x2 => 8,
            AttributeType::F32x4x3 => 12,
            AttributeType::F32x4x4 => 16,
            AttributeType::F64 => 1,
            AttributeType::F64F64 => 2,
            AttributeType::F64F64F64 => 3,
            AttributeType::F64F64F64F64 => 4,
            AttributeType::F64x2x2 => 4,
            AttributeType::F64x2x3 => 6,
            AttributeType::F64x2x4 => 8,
            AttributeType::F64x3x2 => 6,
            AttributeType::F64x3x3 => 9,
            AttributeType::F64x3x4 => 12,
            AttributeType::F64x4x2 => 8,
            AttributeType::F64x4x3 => 12,
            AttributeType::F64x4x4 => 16,
            AttributeType::I2I10I10I10Reversed => 4,
            AttributeType::U2U10U10U10Reversed => 4,
            AttributeType::I10I10I10I2 => 4,
            AttributeType::U10U10U10U2 => 4,
            AttributeType::F10F11F11UnsignedIntReversed => 3,
            AttributeType::FixedFloatI16U16 => 1,
        }
    }
}

pub trait GlPrimitive: Copy + Debug {}

macro_rules! gl_primitive {
    ($type_:ty) => {
        impl GlPrimitive for $type_ {}
    }
}

gl_primitive!(u8);
gl_primitive!(u16);
gl_primitive!(u32);
gl_primitive!(i8);
gl_primitive!(i16);
gl_primitive!(i32);
gl_primitive!(f32);
gl_primitive!(f64);

macro_rules! impl_from_attributes {
    ($primitive: ty) => {
        impl From<$primitive> for AttributeArray<$primitive> {
            fn from(data: $primitive) -> Self {
                Self { data: vec!(data).into_boxed_slice() }
            }
        }

        impl From<($primitive, $primitive)> for AttributeArray<$primitive> {
            fn from(data: ($primitive, $primitive)) -> Self {
                Self { data: vec!(data.0, data.1).into_boxed_slice() }
            }
        }

        impl From<($primitive, $primitive, $primitive)> for AttributeArray<$primitive> {
            fn from(data: ($primitive, $primitive, $primitive)) -> Self {
                Self { data: vec!(data.0, data.1, data.2).into_boxed_slice() }
            }
        }

        impl From<($primitive, $primitive, $primitive, $primitive)> for AttributeArray<$primitive> {
            fn from(data: ($primitive, $primitive, $primitive, $primitive)) -> Self {
                Self { data: vec!(data.0, data.1, data.2, data.3).into_boxed_slice() }
            }
        }

        impl From<[$primitive; 2]> for AttributeArray<$primitive> {
            fn from(data: [$primitive; 2]) -> Self {
                let data: Box<[$primitive]> = Vec::from(data).into_boxed_slice();
                Self { data }
            }
        }

        impl From<[$primitive; 3]> for AttributeArray<$primitive> {
            fn from(data: [$primitive; 3]) -> Self {
                let data: Box<[$primitive]> = Vec::from(data).into_boxed_slice();
                Self { data }
            }
        }

        impl From<[$primitive; 4]> for AttributeArray<$primitive> {
            fn from(data: [$primitive; 4]) -> Self {
                let data: Box<[$primitive]> = Vec::from(data).into_boxed_slice();
                Self { data }
            }
        }

        impl From<[$primitive; 9]> for AttributeArray<$primitive> {
            fn from(data: [$primitive; 9]) -> Self {
                let data: Box<[$primitive]> = Vec::from(data).into_boxed_slice();
                Self { data }
            }
        }

        impl From<[$primitive; 16]> for AttributeArray<$primitive> {
            fn from(data: [$primitive; 16]) -> Self {
                let data: Box<[$primitive]> = Vec::from(data).into_boxed_slice();
                Self { data }
            }
        }

        impl From<[[$primitive; 2]; 2]> for AttributeArray<$primitive> {
            fn from(data: [[$primitive; 2]; 2]) -> Self {
                let data: Box<[$primitive]> = Vec::from_iter(data.into_iter().flat_map(|arr| arr)).into_boxed_slice();
                Self { data }
            }
        }

        impl From<[[$primitive; 3]; 3]> for AttributeArray<$primitive> {
            fn from(data: [[$primitive; 3]; 3]) -> Self {
                let data: Box<[$primitive]> = Vec::from_iter(data.into_iter().flat_map(|arr| arr)).into_boxed_slice();
                Self { data }
            }
        }

        impl From<[[$primitive; 4]; 4]> for AttributeArray<$primitive> {
            fn from(data: [[$primitive; 4]; 4]) -> Self {
                let data: Box<[$primitive]> = Vec::from_iter(data.into_iter().flat_map(|arr| arr)).into_boxed_slice();
                Self { data }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AttributeArray<P: GlPrimitive> {
    data: Box<[P]>,
}

impl<P: GlPrimitive> AttributeArray<P> {
    
}
    

impl<P> AsRef<[P]> for AttributeArray<P>
where
    P: GlPrimitive
{
    fn as_ref(&self) -> &[P] {
        self.data.as_ref()
    }
}

impl_from_attributes!(u8);
impl_from_attributes!(u16);
impl_from_attributes!(u32);
impl_from_attributes!(i8);
impl_from_attributes!(i16);
impl_from_attributes!(i32);
impl_from_attributes!(f32);
impl_from_attributes!(f64);

pub trait Attribute {
    fn get_type() -> AttributeType;
}

// macro_rules! impl_attribute {
//     ($type_:ty, $pointer_type: ty, $enum_type: ident) => {
//         impl Attribute for $type_ {
//             fn get_type() -> AttributeType { AttributeType::$enum_type }
//         }
//     }
// }
//
// impl_attribute!(f32, f32, F32);
// impl_attribute!([f32; 2], f32, F32F32);
// impl_attribute!([f32; 3], f32, F32F32F32);
// impl_attribute!([f32; 4], f32, F32F32F32F32);
// impl_attribute!([f32; 9], f32, F32x3x3);
// impl_attribute!([f32; 16], f32, F32x4x4);
// impl_attribute!([[f32; 2]; 2], f32, F32x4x4);
// impl_attribute!([[f32; 3]; 3], f32, F32x3x3);
// impl_attribute!([[f32; 4]; 4], f32, F32x4x4);

//region i8
impl Attribute for i8 {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I8
    }
}

impl Attribute for (i8, i8) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I8I8
    }
}

impl Attribute for [i8; 2] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I8I8
    }
}

impl Attribute for (i8, i8, i8) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I8I8I8
    }
}

impl Attribute for [i8; 3] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I8I8I8
    }
}

impl Attribute for (i8, i8, i8, i8) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I8I8I8I8
    }
}

impl Attribute for [i8; 4] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I8I8I8I8
    }
}
//endregion

//region u8
impl Attribute for u8 {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U8
    }
}

impl Attribute for (u8, u8) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U8U8
    }
}

impl Attribute for [u8; 2] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U8U8
    }
}

impl Attribute for (u8, u8, u8) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U8U8U8
    }
}

impl Attribute for [u8; 3] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U8U8U8
    }
}

impl Attribute for (u8, u8, u8, u8) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U8U8U8U8
    }
}

impl Attribute for [u8; 4] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U8U8U8U8
    }
}
//endregion

//region i16
impl Attribute for i16 {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I16
    }
}

impl Attribute for (i16, i16) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I16I16
    }
}

impl Attribute for [i16; 2] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I16I16
    }
}

impl Attribute for (i16, i16, i16) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I16I16I16
    }
}

impl Attribute for [i16; 3] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I16I16I16
    }
}

impl Attribute for (i16, i16, i16, i16) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I16I16I16I16
    }
}

impl Attribute for [i16; 4] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I16I16I16I16
    }
}
//endregion

//region u16
impl Attribute for u16 {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U16
    }
}

impl Attribute for (u16, u16) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U16U16
    }
}

impl Attribute for [u16; 2] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U16U16
    }
}

impl Attribute for (u16, u16, u16) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U16U16U16
    }
}

impl Attribute for [u16; 3] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U16U16U16
    }
}

impl Attribute for (u16, u16, u16, u16) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U16U16U16U16
    }
}

impl Attribute for [u16; 4] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U16U16U16U16
    }
}
//endregion

//region i32
impl Attribute for i32 {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I32
    }
}

impl Attribute for (i32, i32) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I32I32
    }
}

impl Attribute for [i32; 2] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I32I32
    }
}

impl Attribute for (i32, i32, i32) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I32I32I32
    }
}

impl Attribute for [i32; 3] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I32I32I32
    }
}

impl Attribute for (i32, i32, i32, i32) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I32I32I32I32
    }
}

impl Attribute for [i32; 4] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I32I32I32I32
    }
}
//endregion

//region u32
impl Attribute for u32 {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U32
    }
}

impl Attribute for (u32, u32) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U32U32
    }
}

impl Attribute for [u32; 2] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U32U32
    }
}

impl Attribute for (u32, u32, u32) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U32U32U32
    }
}

impl Attribute for [u32; 3] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U32U32U32
    }
}

impl Attribute for (u32, u32, u32, u32) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U32U32U32U32
    }
}

impl Attribute for [u32; 4] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U32U32U32U32
    }
}
//endregion

//region i64
impl Attribute for i64 {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I64
    }
}

impl Attribute for (i64, i64) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I64I64
    }
}

impl Attribute for [i64; 2] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I64I64
    }
}

impl Attribute for (i64, i64, i64) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I64I64I64
    }
}

impl Attribute for [i64; 3] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I64I64I64
    }
}

impl Attribute for (i64, i64, i64, i64) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I64I64I64I64
    }
}

impl Attribute for [i64; 4] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I64I64I64I64
    }
}
//endregion

//region u64
impl Attribute for u64 {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U64
    }
}

impl Attribute for (u64, u64) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U64U64
    }
}

impl Attribute for [u64; 2] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U64U64
    }
}

impl Attribute for (u64, u64, u64) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U64U64U64
    }
}

impl Attribute for [u64; 3] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U64U64U64
    }
}

impl Attribute for (u64, u64, u64, u64) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U64U64U64U64
    }
}

impl Attribute for [u64; 4] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::U64U64U64U64
    }
}
//endregion


//region f32
impl Attribute for f32 {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F32
    }
}

impl Attribute for (f32, f32) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F32F32
    }
}

impl Attribute for [f32; 2] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F32F32
    }
}

impl Attribute for (f32, f32, f32) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F32F32F32
    }
}

impl Attribute for [f32; 3] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F32F32F32
    }
}

impl Attribute for (f32, f32, f32, f32) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F32F32F32F32
    }
}

impl Attribute for [f32; 4] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F32F32F32F32
    }
}

impl Attribute for [[f32; 2]; 2] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F32x2x2
    }
}

impl Attribute for [[f32; 3]; 3] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F32x3x3
    }
}

impl Attribute for [[f32; 4]; 4] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F32x4x4
    }
}
//endregion


//region f64
impl Attribute for f64 {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F64
    }
}

impl Attribute for (f64, f64) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F64F64
    }
}

impl Attribute for [f64; 2] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F64F64
    }
}

impl Attribute for (f64, f64, f64) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F64F64F64
    }
}

impl Attribute for [f64; 3] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F64F64F64
    }
}

impl Attribute for (f64, f64, f64, f64) {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F64F64F64F64
    }
}

impl Attribute for [f64; 4] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F64F64F64F64
    }
}

impl Attribute for [[f64; 2]; 2] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F64x2x2
    }
}

impl Attribute for [[f64; 3]; 3] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F64x3x3
    }
}

impl Attribute for [[f64; 4]; 4] {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F64x4x4
    }
}
//endregion
