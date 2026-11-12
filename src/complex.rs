#[derive(Copy, Clone, PartialEq)]
pub struct Complex {
    // Uses fixed-point, with 16
    // fractional bits.
    real: i64,
    imag: i64,
}

impl std::ops::Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, other: Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real / 65536 - self.imag * other.imag / 65536,
            imag: self.real * other.imag / 65536 + self.imag * other.real / 65536,
        }
    }
}

impl Complex {
    pub fn abs(self) -> f32 {
        let a: (f32, f32) = self.into();
        return f32::sqrt(
            a.0*a.0 + a.1*a.1
        );
    }
    pub fn from_f32(
        r: f32,
        i: f32,
    ) -> Complex {
        Complex {
            real: (r * 65536.0) as i64,
            imag: (i * 65536.0) as i64,
        }
    }
    pub fn from_polar(
        r: f32,
        theta: f32
    ) -> Complex {
        Complex::from((
            r * theta.cos(),
            r * theta.sin(),
        ))
    }
    fn div_real(self, other: i64) -> Complex {
        if other == 0 {
            return Complex {
                real: i64::MAX,
                imag: i64::MAX,
            }
        }
        Complex {
            real: self.real * 65536 / other,
            imag: self.imag * 65536 / other,
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Complex;
    fn div(self, other: Complex) -> Complex {
        let conjugate = Complex {
            real: other.real,
            imag: -other.imag,
        };
        (self * conjugate).div_real((other * conjugate).real)
    }
}

impl From<Complex> for (f32, f32) {
    fn from(value: Complex) -> (f32, f32) {
        (
            (value.real as f32) / 65536.0,
            (value.imag as f32) / 65536.0,
        )
    }
}

impl From<(f32, f32)> for Complex {
    fn from(value: (f32, f32)) -> Complex {
        Complex {
            real: (value.0 * 65536.0) as i64,
            imag: (value.1 * 65536.0) as i64,
        }
    }
}
