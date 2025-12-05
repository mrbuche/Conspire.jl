use conspire::{
    constitutive::solid::elastic::{AlmansiHamel, Elastic},
    math::{Scalar, TensorArray},
    mechanics::DeformationGradient,
};
use std::slice::from_raw_parts;

#[unsafe(no_mangle)]
unsafe extern "C" fn almansi_hamel_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            AlmansiHamel {
                bulk_modulus,
                shear_modulus,
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
unsafe extern "C" fn almansi_hamel_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            AlmansiHamel {
                bulk_modulus,
                shear_modulus,
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
unsafe extern "C" fn almansi_hamel_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            AlmansiHamel {
                bulk_modulus,
                shear_modulus,
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
unsafe extern "C" fn almansi_hamel_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            AlmansiHamel {
                bulk_modulus,
                shear_modulus,
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
unsafe extern "C" fn almansi_hamel_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            AlmansiHamel {
                bulk_modulus,
                shear_modulus,
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
unsafe extern "C" fn almansi_hamel_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            AlmansiHamel {
                bulk_modulus,
                shear_modulus,
            }
            .second_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
        ))
    }
}
