use conspire::{
    constitutive::solid::{
        elastic::Elastic,
        hyperelastic::{Gent, Hyperelastic},
    },
    math::{Scalar, TensorArray},
    mechanics::DeformationGradient,
};
use std::slice::from_raw_parts;

#[unsafe(no_mangle)]
unsafe extern "C" fn gent_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            Gent {
                bulk_modulus,
                shear_modulus,
                extensibility,
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
unsafe extern "C" fn gent_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            Gent {
                bulk_modulus,
                shear_modulus,
                extensibility,
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
unsafe extern "C" fn gent_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            Gent {
                bulk_modulus,
                shear_modulus,
                extensibility,
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
unsafe extern "C" fn gent_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            Gent {
                bulk_modulus,
                shear_modulus,
                extensibility,
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
unsafe extern "C" fn gent_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            Gent {
                bulk_modulus,
                shear_modulus,
                extensibility,
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
unsafe extern "C" fn gent_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            Gent {
                bulk_modulus,
                shear_modulus,
                extensibility,
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
unsafe extern "C" fn gent_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> Scalar {
    unsafe {
        Gent {
            bulk_modulus,
            shear_modulus,
            extensibility,
        }
        .helmholtz_free_energy_density(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
    }
}
