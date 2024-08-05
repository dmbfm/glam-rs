use bincode::{impl_borrow_decode, Decode, Encode};

use crate::{
    Affine2, Affine3A, DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4,
    I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat3A,
    Mat4, Quat, U16Vec2, U16Vec3, U16Vec4, U64Vec2, U64Vec3, U64Vec4, UVec2, UVec3, UVec4, Vec2,
    Vec3, Vec3A, Vec4,
};

macro_rules! impl_vec_types {
    ($vec2:ident, $vec3:ident, $vec4:ident) => {
        impl Encode for $vec2 {
            fn encode<E: bincode::enc::Encoder>(
                &self,
                encoder: &mut E,
            ) -> Result<(), bincode::error::EncodeError> {
                Encode::encode(&self.x, encoder)?;
                Encode::encode(&self.y, encoder)?;
                Ok(())
            }
        }

        impl Decode for $vec2 {
            fn decode<D: bincode::de::Decoder>(
                decoder: &mut D,
            ) -> Result<Self, bincode::error::DecodeError> {
                let x = Decode::decode(decoder)?;
                let y = Decode::decode(decoder)?;
                Ok($vec2::new(x, y))
            }
        }
        impl_borrow_decode!($vec2);

        impl Encode for $vec3 {
            fn encode<E: bincode::enc::Encoder>(
                &self,
                encoder: &mut E,
            ) -> Result<(), bincode::error::EncodeError> {
                Encode::encode(&self.x, encoder)?;
                Encode::encode(&self.y, encoder)?;
                Encode::encode(&self.z, encoder)?;
                Ok(())
            }
        }

        impl Decode for $vec3 {
            fn decode<D: bincode::de::Decoder>(
                decoder: &mut D,
            ) -> Result<Self, bincode::error::DecodeError> {
                let x = Decode::decode(decoder)?;
                let y = Decode::decode(decoder)?;
                let z = Decode::decode(decoder)?;
                Ok($vec3::new(x, y, z))
            }
        }
        impl_borrow_decode!($vec3);

        impl Encode for $vec4 {
            fn encode<E: bincode::enc::Encoder>(
                &self,
                encoder: &mut E,
            ) -> Result<(), bincode::error::EncodeError> {
                Encode::encode(&self.x, encoder)?;
                Encode::encode(&self.y, encoder)?;
                Encode::encode(&self.z, encoder)?;
                Encode::encode(&self.w, encoder)?;
                Ok(())
            }
        }

        impl Decode for $vec4 {
            fn decode<D: bincode::de::Decoder>(
                decoder: &mut D,
            ) -> Result<Self, bincode::error::DecodeError> {
                let x = Decode::decode(decoder)?;
                let y = Decode::decode(decoder)?;
                let z = Decode::decode(decoder)?;
                let w = Decode::decode(decoder)?;
                Ok($vec4::new(x, y, z, w))
            }
        }
        impl_borrow_decode!($vec4);
    };
}

impl Encode for Vec3A {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.x, encoder)?;
        Encode::encode(&self.y, encoder)?;
        Encode::encode(&self.z, encoder)?;
        Ok(())
    }
}

impl Decode for Vec3A {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let x = Decode::decode(decoder)?;
        let y = Decode::decode(decoder)?;
        let z = Decode::decode(decoder)?;
        Ok(Vec3A::new(x, y, z))
    }
}
impl_borrow_decode!(Vec3A);

impl_vec_types!(Vec2, Vec3, Vec4);
impl_vec_types!(DVec2, DVec3, DVec4);
impl_vec_types!(IVec2, IVec3, IVec4);
impl_vec_types!(UVec2, UVec3, UVec4);
impl_vec_types!(U16Vec2, U16Vec3, U16Vec4);
impl_vec_types!(I16Vec2, I16Vec3, I16Vec4);
impl_vec_types!(U64Vec2, U64Vec3, U64Vec4);
impl_vec_types!(I64Vec2, I64Vec3, I64Vec4);

impl Encode for Quat {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.x, encoder)?;
        Encode::encode(&self.y, encoder)?;
        Encode::encode(&self.z, encoder)?;
        Encode::encode(&self.w, encoder)?;
        Ok(())
    }
}

impl Decode for Quat {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let x = Decode::decode(decoder)?;
        let y = Decode::decode(decoder)?;
        let z = Decode::decode(decoder)?;
        let w = Decode::decode(decoder)?;
        Ok(Quat::from_xyzw(x, y, z, w))
    }
}
impl_borrow_decode!(Quat);

impl Encode for DQuat {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.x, encoder)?;
        Encode::encode(&self.y, encoder)?;
        Encode::encode(&self.z, encoder)?;
        Encode::encode(&self.w, encoder)?;
        Ok(())
    }
}

impl Decode for DQuat {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let x = Decode::decode(decoder)?;
        let y = Decode::decode(decoder)?;
        let z = Decode::decode(decoder)?;
        let w = Decode::decode(decoder)?;
        Ok(DQuat::from_xyzw(x, y, z, w))
    }
}
impl_borrow_decode!(DQuat);

macro_rules! impl_mat_types {
    ($mat2: ident, $mat3: ident, $mat4: ident) => {
        impl Encode for $mat2 {
            fn encode<E: bincode::enc::Encoder>(
                &self,
                encoder: &mut E,
            ) -> Result<(), bincode::error::EncodeError> {
                Encode::encode(&self.x_axis, encoder)?;
                Encode::encode(&self.y_axis, encoder)?;
                Ok(())
            }
        }

        impl Decode for $mat2 {
            fn decode<D: bincode::de::Decoder>(
                decoder: &mut D,
            ) -> Result<Self, bincode::error::DecodeError> {
                let x_axis = Decode::decode(decoder)?;
                let y_axis = Decode::decode(decoder)?;
                Ok($mat2::from_cols(x_axis, y_axis))
            }
        }
        impl_borrow_decode!($mat2);

        impl Encode for $mat3 {
            fn encode<E: bincode::enc::Encoder>(
                &self,
                encoder: &mut E,
            ) -> Result<(), bincode::error::EncodeError> {
                Encode::encode(&self.x_axis, encoder)?;
                Encode::encode(&self.y_axis, encoder)?;
                Encode::encode(&self.z_axis, encoder)?;
                Ok(())
            }
        }

        impl Decode for $mat3 {
            fn decode<D: bincode::de::Decoder>(
                decoder: &mut D,
            ) -> Result<Self, bincode::error::DecodeError> {
                let x_axis = Decode::decode(decoder)?;
                let y_axis = Decode::decode(decoder)?;
                let z_axis = Decode::decode(decoder)?;
                Ok($mat3::from_cols(x_axis, y_axis, z_axis))
            }
        }
        impl_borrow_decode!($mat3);

        impl Encode for $mat4 {
            fn encode<E: bincode::enc::Encoder>(
                &self,
                encoder: &mut E,
            ) -> Result<(), bincode::error::EncodeError> {
                Encode::encode(&self.x_axis, encoder)?;
                Encode::encode(&self.y_axis, encoder)?;
                Encode::encode(&self.z_axis, encoder)?;
                Encode::encode(&self.w_axis, encoder)?;
                Ok(())
            }
        }

        impl Decode for $mat4 {
            fn decode<D: bincode::de::Decoder>(
                decoder: &mut D,
            ) -> Result<Self, bincode::error::DecodeError> {
                let x_axis = Decode::decode(decoder)?;
                let y_axis = Decode::decode(decoder)?;
                let z_axis = Decode::decode(decoder)?;
                let w_axis = Decode::decode(decoder)?;
                Ok($mat4::from_cols(x_axis, y_axis, z_axis, w_axis))
            }
        }
        impl_borrow_decode!($mat4);
    };
}

impl Encode for Mat3A {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.x_axis, encoder)?;
        Encode::encode(&self.y_axis, encoder)?;
        Encode::encode(&self.z_axis, encoder)?;
        Ok(())
    }
}

impl Decode for Mat3A {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let x_axis = Decode::decode(decoder)?;
        let y_axis = Decode::decode(decoder)?;
        let z_axis = Decode::decode(decoder)?;
        Ok(Mat3A::from_cols(x_axis, y_axis, z_axis))
    }
}
impl_borrow_decode!(Mat3A);

impl_mat_types!(Mat2, Mat3, Mat4);
impl_mat_types!(DMat2, DMat3, DMat4);

impl Encode for Affine2 {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.matrix2, encoder)?;
        Encode::encode(&self.translation, encoder)?;
        Ok(())
    }
}

impl Decode for Affine2 {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let matrix2 = Decode::decode(decoder)?;
        let translation = Decode::decode(decoder)?;
        Ok(Affine2::from_mat2_translation(matrix2, translation))
    }
}
impl_borrow_decode!(Affine2);

impl Encode for Affine3A {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.matrix3, encoder)?;
        Encode::encode(&self.translation, encoder)?;
        Ok(())
    }
}

impl Decode for Affine3A {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let matrix3 = Decode::decode(decoder)?;
        let translation = Decode::decode(decoder)?;
        Ok(Affine3A::from_mat3_translation(matrix3, translation))
    }
}
impl_borrow_decode!(Affine3A);

impl Encode for DAffine2 {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.matrix2, encoder)?;
        Encode::encode(&self.translation, encoder)?;
        Ok(())
    }
}

impl Decode for DAffine2 {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let matrix2 = Decode::decode(decoder)?;
        let translation = Decode::decode(decoder)?;
        Ok(DAffine2::from_mat2_translation(matrix2, translation))
    }
}
impl_borrow_decode!(DAffine2);

impl Encode for DAffine3 {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        Encode::encode(&self.matrix3, encoder)?;
        Encode::encode(&self.translation, encoder)?;
        Ok(())
    }
}

impl Decode for DAffine3 {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let matrix3 = Decode::decode(decoder)?;
        let translation = Decode::decode(decoder)?;
        Ok(DAffine3::from_mat3_translation(matrix3, translation))
    }
}
impl_borrow_decode!(DAffine3);

#[cfg(test)]
mod tests {
    use super::*;

    use bincode::{config, decode_from_slice, encode_to_vec, Decode, Encode};

    #[test]
    fn test_vec2() {
        let vec = Vec2::new(1.0, 2.0);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_dvec2() {
        let vec = DVec2::new(1.0, 2.0);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_ivec2() {
        let vec = IVec2::new(1, 2);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_uvec2() {
        let vec = UVec2::new(1, 2);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_u16vec2() {
        let vec = U16Vec2::new(1, 2);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_i16vec2() {
        let vec = I16Vec2::new(1, 2);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_u64vec2() {
        let vec = U64Vec2::new(1, 2);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_i64vec2() {
        let vec = I64Vec2::new(1, 2);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_vec3() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_dvec3() {
        let vec = DVec3::new(1.0, 2.0, 3.0);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_ivec3() {
        let vec = IVec3::new(1, 2, 3);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_uvec3() {
        let vec = UVec3::new(1, 2, 3);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_u16vec3() {
        let vec = U16Vec3::new(1, 2, 3);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_i16vec3() {
        let vec = I16Vec3::new(1, 2, 3);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_u64vec3() {
        let vec = U64Vec3::new(1, 2, 3);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_i64vec3() {
        let vec = I64Vec3::new(1, 2, 3);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_vec4() {
        let vec = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_dvec4() {
        let vec = DVec4::new(1.0, 2.0, 3.0, 4.0);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_uvec4() {
        let vec = UVec4::new(1, 2, 3, 4);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_ivec4() {
        let vec = IVec4::new(1, 2, 3, 4);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_i16vec4() {
        let vec = I16Vec4::new(1, 2, 3, 4);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }
    #[test]
    fn test_i64vec4() {
        let vec = I64Vec4::new(1, 2, 3, 4);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_u16vec4() {
        let vec = U16Vec4::new(1, 2, 3, 4);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }
    #[test]
    fn test_u64vec4() {
        let vec = U64Vec4::new(1, 2, 3, 4);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_quad() {
        let quad = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
        let buf = encode_to_vec(&quad, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(quad, decoded);
    }

    #[test]
    fn test_dquad() {
        let quad = DQuat::from_xyzw(1.0, 2.0, 3.0, 4.0);
        let buf = encode_to_vec(&quad, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(quad, decoded);
    }

    #[test]
    fn test_mat2() {
        let mat = Mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]);
        let buf = encode_to_vec(&mat, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(mat, decoded);
    }

    #[test]
    fn test_dmat2() {
        let mat = DMat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]);
        let buf = encode_to_vec(&mat, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(mat, decoded);
    }

    #[test]
    fn test_mat3() {
        let mat = Mat3::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let buf = encode_to_vec(&mat, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(mat, decoded);
    }

    #[test]
    fn test_dmat3() {
        let mat = DMat3::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let buf = encode_to_vec(&mat, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(mat, decoded);
    }

    #[test]
    fn test_mat4() {
        let mat = Mat4::from_cols_array(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ]);
        let buf = encode_to_vec(&mat, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(mat, decoded);
    }

    #[test]
    fn test_dmat4() {
        let mat = DMat4::from_cols_array(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ]);
        let buf = encode_to_vec(&mat, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(mat, decoded);
    }

    #[test]
    fn test_vec3a() {
        let vec = Vec3A::new(1.0, 2.0, 3.0);
        let buf = encode_to_vec(&vec, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_mat3a() {
        let mat = Mat3A::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let buf = encode_to_vec(&mat, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(mat, decoded);
    }

    #[test]
    fn test_affine2() {
        let mat = Affine2::from_cols_array(&[1.0, 0.0, 2.0, 0.0, 3.0, 4.0]);
        let buf = encode_to_vec(&mat, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(mat, decoded);
    }

    #[test]
    fn test_affine3a() {
        let mat = Affine3A::from_cols_array(&[
            1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0, 4.0, 5.0, 6.0,
        ]);
        let buf = encode_to_vec(&mat, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(mat, decoded);
    }

    #[test]
    fn test_daffine2() {
        let mat = DAffine2::from_cols_array(&[1.0, 0.0, 2.0, 0.0, 3.0, 4.0]);
        let buf = encode_to_vec(&mat, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(mat, decoded);
    }

    #[test]
    fn test_daffine3() {
        let mat = DAffine3::from_cols_array(&[
            1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0, 4.0, 5.0, 6.0,
        ]);
        let buf = encode_to_vec(&mat, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(mat, decoded);
    }

    #[derive(Debug, Encode, Decode, PartialEq)]
    struct Rect {
        min: Vec2,
        max: Vec2,
    }

    #[test]
    fn test_rect() {
        let rect = Rect {
            min: Vec2::new(1.0, 2.0),
            max: Vec2::new(3.0, 4.0),
        };
        let buf = encode_to_vec(&rect, config::standard()).unwrap();
        let (decoded, _) = decode_from_slice(&buf, config::standard()).unwrap();
        assert_eq!(rect, decoded);
    }
}
