use num_bigint::BigUint;

pub type Point = (BigUint, BigUint);

pub struct EcOps {
    p: BigUint,
}

impl EcOps {
    pub fn new(field: BigUint) -> EcOps {
        EcOps { p: field }
    }

    fn mul(&self, a: &BigUint, b: &BigUint) -> BigUint {
        (a * b) % self.p.clone()
    }

    fn sub(&self, a: &BigUint, b: &BigUint) -> BigUint {
        return (a + self.p.clone() - b) % self.p.clone();
    }

    pub fn point_add(&self, p0: &Point, p1: &Point) -> Point {
        let (x0, y0) = p0;
        let (x1, y1) = p1;

        let temp = self.sub(x1, x0).modinv(&self.p).unwrap();
        let lambda = self.mul(&self.sub(y1, y0), &temp);
        let x2 = self.sub(&self.sub(&self.mul(&lambda, &lambda), &x0), &x1);
        let y2 = self.sub(&self.mul(&lambda, &self.sub(&x0, &x2)), &y0);
        (x2, y2)
    }

    pub fn point_double(&self, p: &Point) -> Point {
        let (x, y) = p;
        let two = BigUint::from(2_u32);
        let three = BigUint::from(3_u32);

        let temp = self.mul(&three, &self.mul(&x, &x));
        let lambda = self.mul(&temp, &self.mul(&two, &y).modinv(&self.p).unwrap());
        let x3 = self.sub(&self.mul(&lambda, &lambda), &self.mul(&two, &x));
        let y3 = self.sub(&self.mul(&lambda, &self.sub(&x, &x3)), &y);
        (x3, y3)
    }

    pub fn point_mul(&self, scalar: &BigUint, mut p: Point) -> Point {
        let bytes = scalar.to_bytes_le();
        let mut res = (BigUint::ZERO, BigUint::ZERO);
        let mut first_add = true;

        for i in 0..bytes.len() {
            for j in 0..8 {
                if bytes[i] & (1 << j) != 0 {
                    if first_add {
                        res = p.clone();
                        first_add = false;
                    } else {
                        res = self.point_add(&res, &p);
                    }
                }
                p = self.point_double(&p);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_add() {
        let secp256k1 = EcOps::new(BigUint::from_bytes_le(&[
            47, 252, 255, 255, 254, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ]));

        let p0 = (
            BigUint::from_bytes_le(&[
                215, 34, 83, 138, 195, 143, 59, 82, 216, 218, 186, 187, 204, 4, 102, 67, 34, 23,
                169, 179, 209, 87, 152, 177, 95, 132, 38, 212, 50, 148, 221, 7,
            ]),
            BigUint::from_bytes_le(&[
                63, 179, 46, 18, 103, 164, 20, 64, 237, 27, 175, 187, 35, 229, 224, 203, 11, 85,
                184, 144, 213, 187, 119, 50, 254, 207, 56, 81, 239, 136, 69, 96,
            ]),
        );

        let p1 = (
            BigUint::from_bytes_le(&[
                120, 31, 50, 101, 252, 100, 135, 184, 30, 73, 248, 5, 172, 141, 107, 4, 130, 39,
                24, 108, 224, 81, 231, 221, 119, 78, 93, 209, 148, 151, 239, 233,
            ]),
            BigUint::from_bytes_le(&[
                179, 23, 247, 231, 232, 159, 145, 18, 240, 65, 170, 20, 99, 215, 45, 86, 92, 69,
                113, 200, 226, 110, 35, 191, 20, 83, 13, 103, 83, 206, 7, 203,
            ]),
        );

        let actual_p = secp256k1.point_add(&p0, &p1);
        let expected_p = (
            BigUint::from_bytes_le(&[
                75, 44, 203, 179, 31, 241, 159, 189, 96, 87, 32, 249, 249, 236, 182, 114, 238, 108,
                12, 18, 147, 34, 81, 107, 41, 219, 66, 171, 30, 224, 119, 51,
            ]),
            BigUint::from_bytes_le(&[
                246, 113, 131, 156, 123, 108, 124, 72, 141, 104, 148, 111, 17, 182, 15, 144, 224,
                190, 177, 1, 42, 56, 77, 4, 209, 235, 239, 51, 15, 137, 149, 127,
            ]),
        );
        assert_eq!(actual_p, expected_p);
    }

    #[test]
    fn point_double() {
        let secp256k1 = EcOps::new(BigUint::from_bytes_le(&[
            47, 252, 255, 255, 254, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ]));

        let p = (
            BigUint::from_bytes_le(&[
                101, 11, 128, 176, 13, 25, 162, 54, 17, 77, 197, 73, 188, 255, 42, 31, 192, 205,
                171, 149, 147, 136, 24, 194, 35, 159, 103, 18, 14, 45, 172, 188,
            ]),
            BigUint::from_bytes_le(&[
                110, 150, 112, 205, 240, 135, 93, 20, 82, 55, 43, 227, 83, 26, 169, 176, 35, 161,
                144, 31, 8, 72, 211, 87, 231, 192, 116, 201, 57, 179, 207, 59,
            ]),
        );

        let actual_p = secp256k1.point_double(&p);
        let expected_p = (
            BigUint::from_bytes_le(&[
                205, 200, 174, 0, 106, 168, 219, 224, 33, 247, 220, 167, 239, 208, 216, 160, 32,
                65, 227, 240, 29, 168, 227, 231, 209, 129, 235, 8, 128, 104, 203, 158,
            ]),
            BigUint::from_bytes_le(&[
                119, 60, 142, 202, 18, 202, 17, 238, 99, 192, 59, 137, 13, 72, 166, 103, 11, 141,
                18, 240, 159, 52, 69, 113, 203, 191, 52, 104, 100, 125, 64, 122,
            ]),
        );
        assert_eq!(actual_p, expected_p);
    }

    #[test]
    fn point_mul_with_scalar() {
        let secp256k1 = EcOps::new(BigUint::from_bytes_le(&[
            47, 252, 255, 255, 254, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ]));

        let p = (
            BigUint::from_bytes_le(&[
                101, 11, 128, 176, 13, 25, 162, 54, 17, 77, 197, 73, 188, 255, 42, 31, 192, 205,
                171, 149, 147, 136, 24, 194, 35, 159, 103, 18, 14, 45, 172, 188,
            ]),
            BigUint::from_bytes_le(&[
                110, 150, 112, 205, 240, 135, 93, 20, 82, 55, 43, 227, 83, 26, 169, 176, 35, 161,
                144, 31, 8, 72, 211, 87, 231, 192, 116, 201, 57, 179, 207, 59,
            ]),
        );
        let scalar = BigUint::from_bytes_le(&[
            100, 180, 114, 218, 109, 165, 84, 202, 172, 62, 78, 11, 19, 200, 68, 91, 26, 119, 244,
            89, 238, 168, 79, 31, 88, 139, 95, 113, 61, 66, 155, 81,
        ]);

        let actual_p = secp256k1.point_mul(&scalar, p);
        let expected_p = (
            BigUint::from_bytes_le(&[
                48, 88, 229, 103, 12, 29, 111, 98, 30, 92, 164, 115, 100, 166, 197, 252, 69, 231,
                204, 194, 224, 185, 228, 221, 138, 239, 114, 129, 210, 62, 188, 87,
            ]),
            BigUint::from_bytes_le(&[
                115, 203, 158, 229, 127, 8, 214, 145, 168, 143, 163, 126, 230, 58, 159, 159, 96,
                17, 225, 136, 33, 221, 110, 228, 58, 243, 88, 141, 72, 65, 47, 106,
            ]),
        );
        assert_eq!(actual_p, expected_p);
    }
}
