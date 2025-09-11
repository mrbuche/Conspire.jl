use conspire::{
    constitutive::{
        Constitutive,
        solid::{
            elastic::{AlmansiHamel, Elastic},
            hyperelastic::{
                ArrudaBoyce, Fung, Gent, Hyperelastic, MooneyRivlin, NeoHookean,
                SaintVenantKirchhoff,
            },
        },
    },
    math::{TensorArray, TensorVec, Vector, special},
    mechanics::{DeformationGradient, Scalar},
};
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
    unsafe { special::rosenbrock(&Vector::new(from_raw_parts(x, len)), a, b) }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn almansi_hamel_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: &[[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            AlmansiHamel::new(&[bulk_modulus, shear_modulus])
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
            AlmansiHamel::new(&[bulk_modulus, shear_modulus])
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
            AlmansiHamel::new(&[bulk_modulus, shear_modulus])
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
            AlmansiHamel::new(&[bulk_modulus, shear_modulus])
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
            AlmansiHamel::new(&[bulk_modulus, shear_modulus])
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
            AlmansiHamel::new(&[bulk_modulus, shear_modulus])
                .second_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

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

#[unsafe(no_mangle)]
unsafe extern "C" fn gent_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            Gent::new(&[bulk_modulus, shear_modulus, extensibility])
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
            Gent::new(&[bulk_modulus, shear_modulus, extensibility])
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
            Gent::new(&[bulk_modulus, shear_modulus, extensibility])
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
            Gent::new(&[bulk_modulus, shear_modulus, extensibility])
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
            Gent::new(&[bulk_modulus, shear_modulus, extensibility])
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
            Gent::new(&[bulk_modulus, shear_modulus, extensibility])
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
        Gent::new(&[bulk_modulus, shear_modulus, extensibility])
            .helmholtz_free_energy_density(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn mooney_rivlin_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            MooneyRivlin::new(&[bulk_modulus, shear_modulus, extra_modulus])
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
            MooneyRivlin::new(&[bulk_modulus, shear_modulus, extra_modulus])
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
            MooneyRivlin::new(&[bulk_modulus, shear_modulus, extra_modulus])
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
            MooneyRivlin::new(&[bulk_modulus, shear_modulus, extra_modulus])
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
            MooneyRivlin::new(&[bulk_modulus, shear_modulus, extra_modulus])
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
            MooneyRivlin::new(&[bulk_modulus, shear_modulus, extra_modulus])
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
        MooneyRivlin::new(&[bulk_modulus, shear_modulus, extra_modulus])
            .helmholtz_free_energy_density(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn neo_hookean_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            NeoHookean::new(&[bulk_modulus, shear_modulus])
                .cauchy_stress(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn neo_hookean_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            NeoHookean::new(&[bulk_modulus, shear_modulus])
                .cauchy_tangent_stiffness(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn neo_hookean_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            NeoHookean::new(&[bulk_modulus, shear_modulus])
                .first_piola_kirchhoff_stress(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn neo_hookean_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            NeoHookean::new(&[bulk_modulus, shear_modulus])
                .first_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn neo_hookean_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            NeoHookean::new(&[bulk_modulus, shear_modulus])
                .second_piola_kirchhoff_stress(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn neo_hookean_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            NeoHookean::new(&[bulk_modulus, shear_modulus])
                .second_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn neo_hookean_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> Scalar {
    unsafe {
        NeoHookean::new(&[bulk_modulus, shear_modulus])
            .helmholtz_free_energy_density(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn saint_venant_kirchhoff_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            SaintVenantKirchhoff::new(&[bulk_modulus, shear_modulus])
                .cauchy_stress(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn saint_venant_kirchhoff_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            SaintVenantKirchhoff::new(&[bulk_modulus, shear_modulus])
                .cauchy_tangent_stiffness(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn saint_venant_kirchhoff_first_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            SaintVenantKirchhoff::new(&[bulk_modulus, shear_modulus])
                .first_piola_kirchhoff_stress(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn saint_venant_kirchhoff_first_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            SaintVenantKirchhoff::new(&[bulk_modulus, shear_modulus])
                .first_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn saint_venant_kirchhoff_second_piola_kirchhoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            SaintVenantKirchhoff::new(&[bulk_modulus, shear_modulus])
                .second_piola_kirchhoff_stress(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn saint_venant_kirchhoff_second_piola_kirchhoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    unsafe {
        Box::into_raw(Box::new(
            SaintVenantKirchhoff::new(&[bulk_modulus, shear_modulus])
                .second_piola_kirchhoff_tangent_stiffness(&DeformationGradient::new(
                    from_raw_parts(deformation_gradient, 9)[0],
                ))
                .unwrap()
                .as_array(),
        ))
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn saint_venant_kirchhoff_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> Scalar {
    unsafe {
        SaintVenantKirchhoff::new(&[bulk_modulus, shear_modulus])
            .helmholtz_free_energy_density(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
    }
}

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
            Fung::new(&[bulk_modulus, shear_modulus, extra_modulus, exponent])
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
            Fung::new(&[bulk_modulus, shear_modulus, extra_modulus, exponent])
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
            Fung::new(&[bulk_modulus, shear_modulus, extra_modulus, exponent])
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
            Fung::new(&[bulk_modulus, shear_modulus, extra_modulus, exponent])
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
            Fung::new(&[bulk_modulus, shear_modulus, extra_modulus, exponent])
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
            Fung::new(&[bulk_modulus, shear_modulus, extra_modulus, exponent])
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
        Fung::new(&[bulk_modulus, shear_modulus, extra_modulus, exponent])
            .helmholtz_free_energy_density(&DeformationGradient::new(
                from_raw_parts(deformation_gradient, 9)[0],
            ))
            .unwrap()
    }
}
