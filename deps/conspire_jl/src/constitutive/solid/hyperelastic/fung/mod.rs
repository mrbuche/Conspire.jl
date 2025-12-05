use conspire::{
    constitutive::solid::{
        elastic::Elastic,
        hyperelastic::{Fung, Hyperelastic},
    },
    math::{Scalar, TensorArray},
    mechanics::DeformationGradient,
};
use std::slice::from_raw_parts;

#[unsafe(no_mangle)]
unsafe extern "C" fn fung_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            Fung {
                bulk_modulus,
                shear_modulus,
                extra_modulus,
                exponent,
            }
            .cauchy_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fung_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            Fung {
                bulk_modulus,
                shear_modulus,
                extra_modulus,
                exponent,
            }
            .cauchy_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fung_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            Fung {
                bulk_modulus,
                shear_modulus,
                extra_modulus,
                exponent,
            }
            .first_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fung_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            Fung {
                bulk_modulus,
                shear_modulus,
                extra_modulus,
                exponent,
            }
            .first_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fung_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            Fung {
                bulk_modulus,
                shear_modulus,
                extra_modulus,
                exponent,
            }
            .second_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fung_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            Fung {
                bulk_modulus,
                shear_modulus,
                extra_modulus,
                exponent,
            }
            .second_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fung_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> Scalar {
    unsafe {
        Fung {
            bulk_modulus,
            shear_modulus,
            extra_modulus,
            exponent,
        }
        .helmholtz_free_energy_density(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
    }
}
