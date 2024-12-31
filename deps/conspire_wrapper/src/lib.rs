use conspire::
{
    constitutive::
    {
        Constitutive,
        solid::
        {
            elastic::
            {
                Elastic,
                AlmansiHamel
            },
            hyperelastic::
            {
                Hyperelastic,
                ArrudaBoyce,
                Fung,
                Gent,
                MooneyRivlin,
                NeoHookean,
                SaintVenantKirchoff,
                Yeoh,
            }
        }
    },
    math::TensorArray,
    mechanics::
    {
        DeformationGradient,
        Scalar
    }
};
use std::slice::from_raw_parts;

#[no_mangle]
unsafe extern fn almansi_hamel_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        AlmansiHamel::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_cauchy_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn almansi_hamel_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        AlmansiHamel::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_cauchy_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn almansi_hamel_first_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        AlmansiHamel::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_first_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn almansi_hamel_first_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        AlmansiHamel::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_first_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn almansi_hamel_second_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        AlmansiHamel::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_second_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn almansi_hamel_second_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        AlmansiHamel::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_second_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn arruda_boyce_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        ArrudaBoyce::new(
            &[bulk_modulus, shear_modulus, number_of_links]
        ).calculate_cauchy_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn arruda_boyce_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        ArrudaBoyce::new(
            &[bulk_modulus, shear_modulus, number_of_links]
        ).calculate_cauchy_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn arruda_boyce_first_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        ArrudaBoyce::new(
            &[bulk_modulus, shear_modulus, number_of_links]
        ).calculate_first_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn arruda_boyce_first_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        ArrudaBoyce::new(
            &[bulk_modulus, shear_modulus, number_of_links]
        ).calculate_first_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn arruda_boyce_second_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        ArrudaBoyce::new(
            &[bulk_modulus, shear_modulus, number_of_links]
        ).calculate_second_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn arruda_boyce_second_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        ArrudaBoyce::new(
            &[bulk_modulus, shear_modulus, number_of_links]
        ).calculate_second_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn arruda_boyce_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    number_of_links: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> Scalar {
    ArrudaBoyce::new(
        &[bulk_modulus, shear_modulus, number_of_links]
    ).calculate_helmholtz_free_energy_density(
        &DeformationGradient::new(
            from_raw_parts(
                deformation_gradient, 9
            )[0]
        )
    ).unwrap()
}

#[no_mangle]
unsafe extern fn gent_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Gent::new(
            &[bulk_modulus, shear_modulus, extensibility]
        ).calculate_cauchy_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn gent_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Gent::new(
            &[bulk_modulus, shear_modulus, extensibility]
        ).calculate_cauchy_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn gent_first_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Gent::new(
            &[bulk_modulus, shear_modulus, extensibility]
        ).calculate_first_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn gent_first_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Gent::new(
            &[bulk_modulus, shear_modulus, extensibility]
        ).calculate_first_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn gent_second_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Gent::new(
            &[bulk_modulus, shear_modulus, extensibility]
        ).calculate_second_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn gent_second_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Gent::new(
            &[bulk_modulus, shear_modulus, extensibility]
        ).calculate_second_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn gent_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extensibility: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> Scalar {
    Gent::new(
        &[bulk_modulus, shear_modulus, extensibility]
    ).calculate_helmholtz_free_energy_density(
        &DeformationGradient::new(
            from_raw_parts(
                deformation_gradient, 9
            )[0]
        )
    ).unwrap()
}

#[no_mangle]
unsafe extern fn mooney_rivlin_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        MooneyRivlin::new(
            &[bulk_modulus, shear_modulus, extra_modulus]
        ).calculate_cauchy_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn mooney_rivlin_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        MooneyRivlin::new(
            &[bulk_modulus, shear_modulus, extra_modulus]
        ).calculate_cauchy_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn mooney_rivlin_first_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        MooneyRivlin::new(
            &[bulk_modulus, shear_modulus, extra_modulus]
        ).calculate_first_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn mooney_rivlin_first_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        MooneyRivlin::new(
            &[bulk_modulus, shear_modulus, extra_modulus]
        ).calculate_first_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn mooney_rivlin_second_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        MooneyRivlin::new(
            &[bulk_modulus, shear_modulus, extra_modulus]
        ).calculate_second_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn mooney_rivlin_second_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        MooneyRivlin::new(
            &[bulk_modulus, shear_modulus, extra_modulus]
        ).calculate_second_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn mooney_rivlin_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> Scalar {
    MooneyRivlin::new(
        &[bulk_modulus, shear_modulus, extra_modulus]
    ).calculate_helmholtz_free_energy_density(
        &DeformationGradient::new(
            from_raw_parts(
                deformation_gradient, 9
            )[0]
        )
    ).unwrap()
}

#[no_mangle]
unsafe extern fn neo_hookean_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        NeoHookean::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_cauchy_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn neo_hookean_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        NeoHookean::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_cauchy_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn neo_hookean_first_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        NeoHookean::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_first_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn neo_hookean_first_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        NeoHookean::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_first_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn neo_hookean_second_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        NeoHookean::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_second_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn neo_hookean_second_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        NeoHookean::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_second_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn neo_hookean_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> Scalar {
    NeoHookean::new(
        &[bulk_modulus, shear_modulus]
    ).calculate_helmholtz_free_energy_density(
        &DeformationGradient::new(
            from_raw_parts(
                deformation_gradient, 9
            )[0]
        )
    ).unwrap()
}

#[no_mangle]
unsafe extern fn saint_venant_kirchoff_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        SaintVenantKirchoff::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_cauchy_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn saint_venant_kirchoff_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        SaintVenantKirchoff::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_cauchy_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn saint_venant_kirchoff_first_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        SaintVenantKirchoff::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_first_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn saint_venant_kirchoff_first_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        SaintVenantKirchoff::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_first_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn saint_venant_kirchoff_second_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        SaintVenantKirchoff::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_second_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn saint_venant_kirchoff_second_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        SaintVenantKirchoff::new(
            &[bulk_modulus, shear_modulus]
        ).calculate_second_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn saint_venant_kirchoff_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> Scalar {
    SaintVenantKirchoff::new(
        &[bulk_modulus, shear_modulus]
    ).calculate_helmholtz_free_energy_density(
        &DeformationGradient::new(
            from_raw_parts(
                deformation_gradient, 9
            )[0]
        )
    ).unwrap()
}

#[no_mangle]
unsafe extern fn fung_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Fung::new(
            &[bulk_modulus, shear_modulus, extra_modulus, exponent]
        ).calculate_cauchy_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn fung_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Fung::new(
            &[bulk_modulus, shear_modulus, extra_modulus, exponent]
        ).calculate_cauchy_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn fung_first_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Fung::new(
            &[bulk_modulus, shear_modulus, extra_modulus, exponent]
        ).calculate_first_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn fung_first_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Fung::new(
            &[bulk_modulus, shear_modulus, extra_modulus, exponent]
        ).calculate_first_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn fung_second_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Fung::new(
            &[bulk_modulus, shear_modulus, extra_modulus, exponent]
        ).calculate_second_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn fung_second_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Fung::new(
            &[bulk_modulus, shear_modulus, extra_modulus, exponent]
        ).calculate_second_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn fung_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_modulus: Scalar,
    exponent: Scalar,
    deformation_gradient: *const [[Scalar; 3]; 3]
) -> Scalar {
    Fung::new(
        &[bulk_modulus, shear_modulus, extra_modulus, exponent]
    ).calculate_helmholtz_free_energy_density(
        &DeformationGradient::new(
            from_raw_parts(
                deformation_gradient, 9
            )[0]
        )
    ).unwrap()
}

#[no_mangle]
unsafe extern fn yeoh_cauchy_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_moduli: *const Scalar,
    len_extra_moduli: usize,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Yeoh::new(
            &[
                &[bulk_modulus], &[shear_modulus], from_raw_parts(
                    extra_moduli, len_extra_moduli
                )
            ].concat()
        ).calculate_cauchy_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn yeoh_cauchy_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_moduli: *const Scalar,
    len_extra_moduli: usize,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Yeoh::new(
            &[
                &[bulk_modulus], &[shear_modulus], from_raw_parts(
                    extra_moduli, len_extra_moduli
                )
            ].concat()
        ).calculate_cauchy_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn yeoh_first_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_moduli: *const Scalar,
    len_extra_moduli: usize,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Yeoh::new(
            &[
                &[bulk_modulus], &[shear_modulus], from_raw_parts(
                    extra_moduli, len_extra_moduli
                )
            ].concat()
        ).calculate_first_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn yeoh_first_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_moduli: *const Scalar,
    len_extra_moduli: usize,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Yeoh::new(
            &[
                &[bulk_modulus], &[shear_modulus], from_raw_parts(
                    extra_moduli, len_extra_moduli
                )
            ].concat()
        ).calculate_first_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn yeoh_second_piola_kirchoff_stress(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_moduli: *const Scalar,
    len_extra_moduli: usize,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[Scalar; 3]; 3] {
    Box::into_raw(Box::new(
        Yeoh::new(
            &[
                &[bulk_modulus], &[shear_modulus], from_raw_parts(
                    extra_moduli, len_extra_moduli
                )
            ].concat()
        ).calculate_second_piola_kirchoff_stress(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn yeoh_second_piola_kirchoff_tangent_stiffness(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_moduli: *const Scalar,
    len_extra_moduli: usize,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> *const [[[[Scalar; 3]; 3]; 3]; 3] {
    Box::into_raw(Box::new(
        Yeoh::new(
            &[
                &[bulk_modulus], &[shear_modulus], from_raw_parts(
                    extra_moduli, len_extra_moduli
                )
            ].concat()
        ).calculate_second_piola_kirchoff_tangent_stiffness(
            &DeformationGradient::new(
                from_raw_parts(
                    deformation_gradient, 9
                )[0]
            )
        ).unwrap().as_array()
    ))
}

#[no_mangle]
unsafe extern fn yeoh_helmholtz_free_energy_density(
    bulk_modulus: Scalar,
    shear_modulus: Scalar,
    extra_moduli: *const Scalar,
    len_extra_moduli: usize,
    deformation_gradient: *const [[Scalar; 3]; 3],
) -> Scalar {
    Yeoh::new(
        &[
            &[bulk_modulus], &[shear_modulus], from_raw_parts(
                extra_moduli, len_extra_moduli
            )
        ].concat()
    ).calculate_helmholtz_free_energy_density(
        &DeformationGradient::new(
            from_raw_parts(
                deformation_gradient, 9
            )[0]
        )
    ).unwrap()
}
