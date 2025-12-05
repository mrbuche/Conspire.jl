use conspire::math::{Scalar, Vector, special};
use std::slice::from_raw_parts;

#[unsafe(no_mangle)]
unsafe extern "C" fn lambert_w(x: Scalar) -> Scalar {
    special::lambert_w(x)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn langevin(x: Scalar) -> Scalar {
    special::langevin(x)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn inverse_langevin(y: Scalar) -> Scalar {
    special::inverse_langevin(y)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn rosenbrock(x: *const Scalar, len: usize, a: Scalar, b: Scalar) -> Scalar {
    unsafe { special::rosenbrock(&Vector::from(from_raw_parts(x, len)), a, b) }
}
