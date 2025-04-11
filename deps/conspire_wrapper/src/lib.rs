use conspire::{
    constitutive::{
        solid::{
            elastic::{AlmansiHamel, Elastic},
            hyperelastic::{
                ArrudaBoyce, Fung, Gent, Hyperelastic, MooneyRivlin, NeoHookean,
                SaintVenantKirchhoff, Yeoh,
            },
        },
        Constitutive,
    },
    math::{special, TensorArray},
    mechanics::{DeformationGradient, Scalar},
};
use std::slice::from_raw_parts;

#[unsafe(no_mangle)]
unsafe extern "C" fn langevin(x: Scalar) -> Scalar {
    special::langevin(x)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn inverse_langevin(y: Scalar) -> Scalar {
    special::inverse_langevin(y)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn almansi_hamel_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        AlmansiHamel::new(&[bulk_modulus, shear_modulus])
            .cauchy_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn almansi_hamel_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        AlmansiHamel::new(&[bulk_modulus, shear_modulus])
            .cauchy_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn almansi_hamel_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        AlmansiHamel::new(&[bulk_modulus, shear_modulus])
            .first_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn almansi_hamel_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        AlmansiHamel::new(&[bulk_modulus, shear_modulus])
            .first_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn almansi_hamel_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        AlmansiHamel::new(&[bulk_modulus, shear_modulus])
            .second_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn almansi_hamel_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        AlmansiHamel::new(&[bulk_modulus, shear_modulus])
            .second_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn arruda_boyce_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        ArrudaBoyce::new(&[bulk_modulus, shear_modulus, number_of_links])
            .cauchy_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn arruda_boyce_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        ArrudaBoyce::new(&[bulk_modulus, shear_modulus, number_of_links])
            .cauchy_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn arruda_boyce_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        ArrudaBoyce::new(&[bulk_modulus, shear_modulus, number_of_links])
            .first_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn arruda_boyce_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        ArrudaBoyce::new(&[bulk_modulus, shear_modulus, number_of_links])
            .first_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn arruda_boyce_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        ArrudaBoyce::new(&[bulk_modulus, shear_modulus, number_of_links])
            .second_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn arruda_boyce_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        ArrudaBoyce::new(&[bulk_modulus, shear_modulus, number_of_links])
            .second_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn arruda_boyce_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> Scalar {
    ArrudaBoyce::new(&[bulk_modulus, shear_modulus, number_of_links])
        .helmholtz_free_energy_density(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn gent_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Gent::new(&[bulk_modulus, shear_modulus, extensibility])
            .cauchy_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn gent_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Gent::new(&[bulk_modulus, shear_modulus, extensibility])
            .cauchy_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn gent_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Gent::new(&[bulk_modulus, shear_modulus, extensibility])
            .first_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn gent_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Gent::new(&[bulk_modulus, shear_modulus, extensibility])
            .first_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn gent_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Gent::new(&[bulk_modulus, shear_modulus, extensibility])
            .second_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn gent_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Gent::new(&[bulk_modulus, shear_modulus, extensibility])
            .second_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn gent_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> Scalar {
    Gent::new(&[bulk_modulus, shear_modulus, extensibility])
        .helmholtz_free_energy_density(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn mooney_rivlin_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        MooneyRivlin::new(&[bulk_modulus, shear_modulus, extra_modulus])
            .cauchy_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn mooney_rivlin_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        MooneyRivlin::new(&[bulk_modulus, shear_modulus, extra_modulus])
            .cauchy_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn mooney_rivlin_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        MooneyRivlin::new(&[bulk_modulus, shear_modulus, extra_modulus])
            .first_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn mooney_rivlin_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        MooneyRivlin::new(&[bulk_modulus, shear_modulus, extra_modulus])
            .first_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn mooney_rivlin_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        MooneyRivlin::new(&[bulk_modulus, shear_modulus, extra_modulus])
            .second_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn mooney_rivlin_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        MooneyRivlin::new(&[bulk_modulus, shear_modulus, extra_modulus])
            .second_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn mooney_rivlin_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> Scalar {
    MooneyRivlin::new(&[bulk_modulus, shear_modulus, extra_modulus])
        .helmholtz_free_energy_density(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn neo_hookean_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        NeoHookean::new(&[bulk_modulus, shear_modulus])
            .cauchy_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn neo_hookean_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        NeoHookean::new(&[bulk_modulus, shear_modulus])
            .cauchy_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn neo_hookean_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        NeoHookean::new(&[bulk_modulus, shear_modulus])
            .first_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn neo_hookean_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        NeoHookean::new(&[bulk_modulus, shear_modulus])
            .first_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn neo_hookean_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        NeoHookean::new(&[bulk_modulus, shear_modulus])
            .second_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn neo_hookean_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        NeoHookean::new(&[bulk_modulus, shear_modulus])
            .second_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn neo_hookean_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> Scalar {
    NeoHookean::new(&[bulk_modulus, shear_modulus])
        .helmholtz_free_energy_density(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn saint_venant_kirchhoff_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        SaintVenantKirchhoff::new(&[bulk_modulus, shear_modulus])
            .cauchy_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn saint_venant_kirchhoff_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        SaintVenantKirchhoff::new(&[bulk_modulus, shear_modulus])
            .cauchy_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn saint_venant_kirchhoff_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        SaintVenantKirchhoff::new(&[bulk_modulus, shear_modulus])
            .first_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn saint_venant_kirchhoff_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        SaintVenantKirchhoff::new(&[bulk_modulus, shear_modulus])
            .first_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn saint_venant_kirchhoff_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        SaintVenantKirchhoff::new(&[bulk_modulus, shear_modulus])
            .second_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn saint_venant_kirchhoff_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        SaintVenantKirchhoff::new(&[bulk_modulus, shear_modulus])
            .second_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn saint_venant_kirchhoff_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> Scalar {
    SaintVenantKirchhoff::new(&[bulk_modulus, shear_modulus])
        .helmholtz_free_energy_density(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fung_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Fung::new(&[bulk_modulus, shear_modulus, extra_modulus, exponent])
            .cauchy_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fung_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Fung::new(&[bulk_modulus, shear_modulus, extra_modulus, exponent])
            .cauchy_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fung_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Fung::new(&[bulk_modulus, shear_modulus, extra_modulus, exponent])
            .first_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fung_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Fung::new(&[bulk_modulus, shear_modulus, extra_modulus, exponent])
            .first_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fung_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Fung::new(&[bulk_modulus, shear_modulus, extra_modulus, exponent])
            .second_piola_kirchhoff_stress(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fung_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Fung::new(&[bulk_modulus, shear_modulus, extra_modulus, exponent])
            .second_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
            .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fung_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> Scalar {
    Fung::new(&[bulk_modulus, shear_modulus, extra_modulus, exponent])
        .helmholtz_free_energy_density(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn yeoh_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_moduli: *const Scalar,
    len_extra_moduli: usize,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Yeoh::new(
            &[
                &[bulk_modulus],
                &[shear_modulus],
                from_raw_parts(extra_moduli, len_extra_moduli),
            ]
            .concat(),
        )
        .cauchy_stress(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
        .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn yeoh_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_moduli: *const Scalar,
    len_extra_moduli: usize,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Yeoh::new(
            &[
                &[bulk_modulus],
                &[shear_modulus],
                from_raw_parts(extra_moduli, len_extra_moduli),
            ]
            .concat(),
        )
        .cauchy_tangent_stiffness(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
        .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn yeoh_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_moduli: *const Scalar,
    len_extra_moduli: usize,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Yeoh::new(
            &[
                &[bulk_modulus],
                &[shear_modulus],
                from_raw_parts(extra_moduli, len_extra_moduli),
            ]
            .concat(),
        )
        .first_piola_kirchhoff_stress(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
        .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn yeoh_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_moduli: *const Scalar,
    len_extra_moduli: usize,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Yeoh::new(
            &[
                &[bulk_modulus],
                &[shear_modulus],
                from_raw_parts(extra_moduli, len_extra_moduli),
            ]
            .concat(),
        )
        .first_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
        .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn yeoh_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_moduli: *const Scalar,
    len_extra_moduli: usize,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Yeoh::new(
            &[
                &[bulk_modulus],
                &[shear_modulus],
                from_raw_parts(extra_moduli, len_extra_moduli),
            ]
            .concat(),
        )
        .second_piola_kirchhoff_stress(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
        .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn yeoh_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_moduli: *const Scalar,
    len_extra_moduli: usize,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Yeoh::new(
            &[
                &[bulk_modulus],
                &[shear_modulus],
                from_raw_parts(extra_moduli, len_extra_moduli),
            ]
            .concat(),
        )
        .second_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
            from_raw_parts(deformation_gradient, 9)[0],
        ))
        .unwrap()
        .as_array(),
    ))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn yeoh_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_moduli: *const Scalar,
    len_extra_moduli: usize,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> Scalar {
    Yeoh::new(
        &[
            &[bulk_modulus],
            &[shear_modulus],
            from_raw_parts(extra_moduli, len_extra_moduli),
        ]
        .concat(),
    )
    .helmholtz_free_energy_density(&DeformationGradient::new(
        from_raw_parts(deformation_gradient, 9)[0],
    ))
    .unwrap()
}
