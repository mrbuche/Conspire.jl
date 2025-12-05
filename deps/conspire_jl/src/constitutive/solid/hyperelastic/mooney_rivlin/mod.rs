use conspire::{
    constitutive::solid::{
        elastic::Elastic,
        hyperelastic::{Hyperelastic, MooneyRivlin},
    },
    math::{Scalar, TensorArray},
    mechanics::DeformationGradient,
};
use std::slice::from_raw_parts;

#[unsafe(no_mangle)]
unsafe extern "C" fn mooney_rivlin_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            MooneyRivlin {
                bulk_modulus,
                shear_modulus,
                extra_modulus,
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
unsafe extern "C" fn mooney_rivlin_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            MooneyRivlin {
                bulk_modulus,
                shear_modulus,
                extra_modulus,
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
unsafe extern "C" fn mooney_rivlin_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            MooneyRivlin {
                bulk_modulus,
                shear_modulus,
                extra_modulus,
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
unsafe extern "C" fn mooney_rivlin_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            MooneyRivlin {
                bulk_modulus,
                shear_modulus,
                extra_modulus,
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
unsafe extern "C" fn mooney_rivlin_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            MooneyRivlin {
                bulk_modulus,
                shear_modulus,
                extra_modulus,
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
unsafe extern "C" fn mooney_rivlin_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            MooneyRivlin {
                bulk_modulus,
                shear_modulus,
                extra_modulus,
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
unsafe extern "C" fn mooney_rivlin_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> Scalar {
    unsafe {
        MooneyRivlin {
            bulk_modulus,
            shear_modulus,
            extra_modulus,
        }
        .helmholtz_free_energy_density(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
    }
}
