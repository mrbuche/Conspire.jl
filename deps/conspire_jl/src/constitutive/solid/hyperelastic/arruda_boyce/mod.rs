use conspire::{
    constitutive::{
        Constitutive,
        solid::{
            elastic::Elastic,
            hyperelastic::{ArrudaBoyce, Hyperelastic},
        },
    },
    math::{Scalar, TensorArray},
    mechanics::DeformationGradient,
};
use std::slice::from_raw_parts;

#[unsafe(no_mangle)]
unsafe extern "C" fn arruda_boyce_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            ArrudaBoyce::new(&[bulk_modulus, shear_modulus, number_of_links])
                .cauchy_stress(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn arruda_boyce_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            ArrudaBoyce::new(&[bulk_modulus, shear_modulus, number_of_links])
                .cauchy_tangent_stiffness(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn arruda_boyce_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            ArrudaBoyce::new(&[bulk_modulus, shear_modulus, number_of_links])
                .first_piola_kirchhoff_stress(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn arruda_boyce_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            ArrudaBoyce::new(&[bulk_modulus, shear_modulus, number_of_links])
                .first_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn arruda_boyce_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            ArrudaBoyce::new(&[bulk_modulus, shear_modulus, number_of_links])
                .second_piola_kirchhoff_stress(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn arruda_boyce_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            ArrudaBoyce::new(&[bulk_modulus, shear_modulus, number_of_links])
                .second_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn arruda_boyce_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> Scalar {
    unsafe {
        ArrudaBoyce::new(&[bulk_modulus, shear_modulus, number_of_links])
            .helmholtz_free_energy_density(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
    }
}
