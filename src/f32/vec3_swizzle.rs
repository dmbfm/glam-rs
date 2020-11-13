// Generated by swizzlegen. Do not edit.

use super::{Vec2, Vec3, Vec4};

pub trait Vec3Swizzles {
    fn xxxx(self) -> Vec4;
    fn xxxy(self) -> Vec4;
    fn xxxz(self) -> Vec4;
    fn xxyx(self) -> Vec4;
    fn xxyy(self) -> Vec4;
    fn xxyz(self) -> Vec4;
    fn xxzx(self) -> Vec4;
    fn xxzy(self) -> Vec4;
    fn xxzz(self) -> Vec4;
    fn xyxx(self) -> Vec4;
    fn xyxy(self) -> Vec4;
    fn xyxz(self) -> Vec4;
    fn xyyx(self) -> Vec4;
    fn xyyy(self) -> Vec4;
    fn xyyz(self) -> Vec4;
    fn xyzx(self) -> Vec4;
    fn xyzy(self) -> Vec4;
    fn xyzz(self) -> Vec4;
    fn xzxx(self) -> Vec4;
    fn xzxy(self) -> Vec4;
    fn xzxz(self) -> Vec4;
    fn xzyx(self) -> Vec4;
    fn xzyy(self) -> Vec4;
    fn xzyz(self) -> Vec4;
    fn xzzx(self) -> Vec4;
    fn xzzy(self) -> Vec4;
    fn xzzz(self) -> Vec4;
    fn yxxx(self) -> Vec4;
    fn yxxy(self) -> Vec4;
    fn yxxz(self) -> Vec4;
    fn yxyx(self) -> Vec4;
    fn yxyy(self) -> Vec4;
    fn yxyz(self) -> Vec4;
    fn yxzx(self) -> Vec4;
    fn yxzy(self) -> Vec4;
    fn yxzz(self) -> Vec4;
    fn yyxx(self) -> Vec4;
    fn yyxy(self) -> Vec4;
    fn yyxz(self) -> Vec4;
    fn yyyx(self) -> Vec4;
    fn yyyy(self) -> Vec4;
    fn yyyz(self) -> Vec4;
    fn yyzx(self) -> Vec4;
    fn yyzy(self) -> Vec4;
    fn yyzz(self) -> Vec4;
    fn yzxx(self) -> Vec4;
    fn yzxy(self) -> Vec4;
    fn yzxz(self) -> Vec4;
    fn yzyx(self) -> Vec4;
    fn yzyy(self) -> Vec4;
    fn yzyz(self) -> Vec4;
    fn yzzx(self) -> Vec4;
    fn yzzy(self) -> Vec4;
    fn yzzz(self) -> Vec4;
    fn zxxx(self) -> Vec4;
    fn zxxy(self) -> Vec4;
    fn zxxz(self) -> Vec4;
    fn zxyx(self) -> Vec4;
    fn zxyy(self) -> Vec4;
    fn zxyz(self) -> Vec4;
    fn zxzx(self) -> Vec4;
    fn zxzy(self) -> Vec4;
    fn zxzz(self) -> Vec4;
    fn zyxx(self) -> Vec4;
    fn zyxy(self) -> Vec4;
    fn zyxz(self) -> Vec4;
    fn zyyx(self) -> Vec4;
    fn zyyy(self) -> Vec4;
    fn zyyz(self) -> Vec4;
    fn zyzx(self) -> Vec4;
    fn zyzy(self) -> Vec4;
    fn zyzz(self) -> Vec4;
    fn zzxx(self) -> Vec4;
    fn zzxy(self) -> Vec4;
    fn zzxz(self) -> Vec4;
    fn zzyx(self) -> Vec4;
    fn zzyy(self) -> Vec4;
    fn zzyz(self) -> Vec4;
    fn zzzx(self) -> Vec4;
    fn zzzy(self) -> Vec4;
    fn zzzz(self) -> Vec4;
    fn xxx(self) -> Vec3;
    fn xxy(self) -> Vec3;
    fn xxz(self) -> Vec3;
    fn xyx(self) -> Vec3;
    fn xyy(self) -> Vec3;
    fn xzx(self) -> Vec3;
    fn xzy(self) -> Vec3;
    fn xzz(self) -> Vec3;
    fn yxx(self) -> Vec3;
    fn yxy(self) -> Vec3;
    fn yxz(self) -> Vec3;
    fn yyx(self) -> Vec3;
    fn yyy(self) -> Vec3;
    fn yyz(self) -> Vec3;
    fn yzx(self) -> Vec3;
    fn yzy(self) -> Vec3;
    fn yzz(self) -> Vec3;
    fn zxx(self) -> Vec3;
    fn zxy(self) -> Vec3;
    fn zxz(self) -> Vec3;
    fn zyx(self) -> Vec3;
    fn zyy(self) -> Vec3;
    fn zyz(self) -> Vec3;
    fn zzx(self) -> Vec3;
    fn zzy(self) -> Vec3;
    fn zzz(self) -> Vec3;
    fn xx(self) -> Vec2;
    fn xy(self) -> Vec2;
    fn xz(self) -> Vec2;
    fn yx(self) -> Vec2;
    fn yy(self) -> Vec2;
    fn yz(self) -> Vec2;
    fn zx(self) -> Vec2;
    fn zy(self) -> Vec2;
    fn zz(self) -> Vec2;
}

impl Vec3Swizzles for Vec3 {
    #[inline]
    fn xxxx(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.x, self.x)
    }
    #[inline]
    fn xxxy(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.x, self.y)
    }
    #[inline]
    fn xxxz(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.x, self.z)
    }
    #[inline]
    fn xxyx(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.y, self.x)
    }
    #[inline]
    fn xxyy(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.y, self.y)
    }
    #[inline]
    fn xxyz(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.y, self.z)
    }
    #[inline]
    fn xxzx(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.z, self.x)
    }
    #[inline]
    fn xxzy(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.z, self.y)
    }
    #[inline]
    fn xxzz(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.z, self.z)
    }
    #[inline]
    fn xyxx(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.x, self.x)
    }
    #[inline]
    fn xyxy(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.x, self.y)
    }
    #[inline]
    fn xyxz(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.x, self.z)
    }
    #[inline]
    fn xyyx(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.y, self.x)
    }
    #[inline]
    fn xyyy(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.y, self.y)
    }
    #[inline]
    fn xyyz(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.y, self.z)
    }
    #[inline]
    fn xyzx(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.x)
    }
    #[inline]
    fn xyzy(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.y)
    }
    #[inline]
    fn xyzz(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.z)
    }
    #[inline]
    fn xzxx(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.x, self.x)
    }
    #[inline]
    fn xzxy(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.x, self.y)
    }
    #[inline]
    fn xzxz(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.x, self.z)
    }
    #[inline]
    fn xzyx(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.y, self.x)
    }
    #[inline]
    fn xzyy(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.y, self.y)
    }
    #[inline]
    fn xzyz(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.y, self.z)
    }
    #[inline]
    fn xzzx(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.z, self.x)
    }
    #[inline]
    fn xzzy(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.z, self.y)
    }
    #[inline]
    fn xzzz(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.z, self.z)
    }
    #[inline]
    fn yxxx(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.x, self.x)
    }
    #[inline]
    fn yxxy(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.x, self.y)
    }
    #[inline]
    fn yxxz(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.x, self.z)
    }
    #[inline]
    fn yxyx(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.y, self.x)
    }
    #[inline]
    fn yxyy(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.y, self.y)
    }
    #[inline]
    fn yxyz(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.y, self.z)
    }
    #[inline]
    fn yxzx(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.z, self.x)
    }
    #[inline]
    fn yxzy(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.z, self.y)
    }
    #[inline]
    fn yxzz(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.z, self.z)
    }
    #[inline]
    fn yyxx(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.x, self.x)
    }
    #[inline]
    fn yyxy(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.x, self.y)
    }
    #[inline]
    fn yyxz(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.x, self.z)
    }
    #[inline]
    fn yyyx(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.y, self.x)
    }
    #[inline]
    fn yyyy(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.y, self.y)
    }
    #[inline]
    fn yyyz(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.y, self.z)
    }
    #[inline]
    fn yyzx(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.z, self.x)
    }
    #[inline]
    fn yyzy(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.z, self.y)
    }
    #[inline]
    fn yyzz(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.z, self.z)
    }
    #[inline]
    fn yzxx(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.x, self.x)
    }
    #[inline]
    fn yzxy(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.x, self.y)
    }
    #[inline]
    fn yzxz(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.x, self.z)
    }
    #[inline]
    fn yzyx(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.y, self.x)
    }
    #[inline]
    fn yzyy(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.y, self.y)
    }
    #[inline]
    fn yzyz(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.y, self.z)
    }
    #[inline]
    fn yzzx(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.z, self.x)
    }
    #[inline]
    fn yzzy(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.z, self.y)
    }
    #[inline]
    fn yzzz(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.z, self.z)
    }
    #[inline]
    fn zxxx(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.x, self.x)
    }
    #[inline]
    fn zxxy(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.x, self.y)
    }
    #[inline]
    fn zxxz(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.x, self.z)
    }
    #[inline]
    fn zxyx(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.y, self.x)
    }
    #[inline]
    fn zxyy(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.y, self.y)
    }
    #[inline]
    fn zxyz(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.y, self.z)
    }
    #[inline]
    fn zxzx(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.z, self.x)
    }
    #[inline]
    fn zxzy(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.z, self.y)
    }
    #[inline]
    fn zxzz(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.z, self.z)
    }
    #[inline]
    fn zyxx(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.x, self.x)
    }
    #[inline]
    fn zyxy(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.x, self.y)
    }
    #[inline]
    fn zyxz(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.x, self.z)
    }
    #[inline]
    fn zyyx(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.y, self.x)
    }
    #[inline]
    fn zyyy(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.y, self.y)
    }
    #[inline]
    fn zyyz(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.y, self.z)
    }
    #[inline]
    fn zyzx(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.z, self.x)
    }
    #[inline]
    fn zyzy(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.z, self.y)
    }
    #[inline]
    fn zyzz(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.z, self.z)
    }
    #[inline]
    fn zzxx(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.x, self.x)
    }
    #[inline]
    fn zzxy(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.x, self.y)
    }
    #[inline]
    fn zzxz(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.x, self.z)
    }
    #[inline]
    fn zzyx(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.y, self.x)
    }
    #[inline]
    fn zzyy(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.y, self.y)
    }
    #[inline]
    fn zzyz(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.y, self.z)
    }
    #[inline]
    fn zzzx(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.z, self.x)
    }
    #[inline]
    fn zzzy(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.z, self.y)
    }
    #[inline]
    fn zzzz(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.z, self.z)
    }
    #[inline]
    fn xxx(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }
    #[inline]
    fn xxy(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.x,
            z: self.y,
        }
    }
    #[inline]
    fn xxz(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.x,
            z: self.z,
        }
    }
    #[inline]
    fn xyx(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.x,
        }
    }
    #[inline]
    fn xyy(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.y,
        }
    }
    #[inline]
    fn xzx(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.z,
            z: self.x,
        }
    }
    #[inline]
    fn xzy(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.z,
            z: self.y,
        }
    }
    #[inline]
    fn xzz(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.z,
            z: self.z,
        }
    }
    #[inline]
    fn yxx(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.x,
            z: self.x,
        }
    }
    #[inline]
    fn yxy(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.x,
            z: self.y,
        }
    }
    #[inline]
    fn yxz(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.x,
            z: self.z,
        }
    }
    #[inline]
    fn yyx(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.y,
            z: self.x,
        }
    }
    #[inline]
    fn yyy(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }
    #[inline]
    fn yyz(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.y,
            z: self.z,
        }
    }
    #[inline]
    fn yzx(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.z,
            z: self.x,
        }
    }
    #[inline]
    fn yzy(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.z,
            z: self.y,
        }
    }
    #[inline]
    fn yzz(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.z,
            z: self.z,
        }
    }
    #[inline]
    fn zxx(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.x,
            z: self.x,
        }
    }
    #[inline]
    fn zxy(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.x,
            z: self.y,
        }
    }
    #[inline]
    fn zxz(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.x,
            z: self.z,
        }
    }
    #[inline]
    fn zyx(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.y,
            z: self.x,
        }
    }
    #[inline]
    fn zyy(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.y,
            z: self.y,
        }
    }
    #[inline]
    fn zyz(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.y,
            z: self.z,
        }
    }
    #[inline]
    fn zzx(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.z,
            z: self.x,
        }
    }
    #[inline]
    fn zzy(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.z,
            z: self.y,
        }
    }
    #[inline]
    fn zzz(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.z,
            z: self.z,
        }
    }
    #[inline]
    fn xx(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.x,
        }
    }
    #[inline]
    fn xy(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
    #[inline]
    fn xz(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.z,
        }
    }
    #[inline]
    fn yx(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.x,
        }
    }
    #[inline]
    fn yy(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.y,
        }
    }
    #[inline]
    fn yz(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.z,
        }
    }
    #[inline]
    fn zx(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.x,
        }
    }
    #[inline]
    fn zy(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.y,
        }
    }
    #[inline]
    fn zz(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.z,
        }
    }
}
