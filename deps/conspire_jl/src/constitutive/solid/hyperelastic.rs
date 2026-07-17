macro_rules! hyperelastic_ffi {
    ($prefix:ident, $model:ident { $($field:ident),+ $(,)? }) => {
        mod $prefix {
            $crate::constitutive::solid::elastic::elastic_ffi_items!(
                $prefix,
                ::conspire::constitutive::solid::hyperelastic::$model { $($field),+ }
            );

            #[unsafe(export_name = concat!(stringify!($prefix), "_helmholtz_free_energy_density"))]
            unsafe extern "C" fn helmholtz_free_energy_density(
                $($field: ::conspire::math::Scalar,)+
                deformation_gradient: *const [[::conspire::math::Scalar; 3]; 3],
            ) -> ::conspire::math::Scalar {
                unsafe {
                    ::conspire::constitutive::solid::hyperelastic::Hyperelastic::helmholtz_free_energy_density(
                        &Model { $($field),+ },
                        &::std::slice::from_raw_parts(deformation_gradient, 9)[0].into(),
                    )
                    .unwrap()
                }
            }
        }
    };
}

hyperelastic_ffi!(arruda_boyce, ArrudaBoyce { bulk_modulus, shear_modulus, number_of_links });
hyperelastic_ffi!(fung, Fung { bulk_modulus, shear_modulus, extra_modulus, exponent });
hyperelastic_ffi!(gent, Gent { bulk_modulus, shear_modulus, extensibility });
hyperelastic_ffi!(hencky, Hencky { bulk_modulus, shear_modulus });
hyperelastic_ffi!(mooney_rivlin, MooneyRivlin { bulk_modulus, shear_modulus, extra_modulus });
hyperelastic_ffi!(neo_hookean, NeoHookean { bulk_modulus, shear_modulus });
hyperelastic_ffi!(saint_venant_kirchhoff, SaintVenantKirchhoff { bulk_modulus, shear_modulus });
