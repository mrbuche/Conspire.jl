macro_rules! elastic_ffi_items {
    ($prefix:ident, $model:path { $($field:ident),+ $(,)? }) => {
        type Model = $model;

        #[unsafe(export_name = concat!(stringify!($prefix), "_cauchy_stress"))]
        unsafe extern "C" fn cauchy_stress(
            $($field: ::conspire::math::Scalar,)+
            deformation_gradient: *const [[::conspire::math::Scalar; 3]; 3],
            output: *mut [[::conspire::math::Scalar; 3]; 3],
        ) {
            unsafe {
                *output = ::conspire::math::TensorArray::as_array(
                    &::conspire::constitutive::solid::elastic::Elastic::cauchy_stress(
                        &Model { $($field),+ },
                        &::std::slice::from_raw_parts(deformation_gradient, 9)[0].into(),
                    )
                    .unwrap(),
                );
            }
        }

        #[unsafe(export_name = concat!(stringify!($prefix), "_cauchy_tangent_stiffness"))]
        unsafe extern "C" fn cauchy_tangent_stiffness(
            $($field: ::conspire::math::Scalar,)+
            deformation_gradient: *const [[::conspire::math::Scalar; 3]; 3],
            output: *mut [[[[::conspire::math::Scalar; 3]; 3]; 3]; 3],
        ) {
            unsafe {
                *output = ::conspire::math::TensorArray::as_array(
                    &::conspire::constitutive::solid::elastic::Elastic::cauchy_tangent_stiffness(
                        &Model { $($field),+ },
                        &::std::slice::from_raw_parts(deformation_gradient, 9)[0].into(),
                    )
                    .unwrap(),
                );
            }
        }

        #[unsafe(export_name = concat!(stringify!($prefix), "_first_piola_kirchhoff_stress"))]
        unsafe extern "C" fn first_piola_kirchhoff_stress(
            $($field: ::conspire::math::Scalar,)+
            deformation_gradient: *const [[::conspire::math::Scalar; 3]; 3],
            output: *mut [[::conspire::math::Scalar; 3]; 3],
        ) {
            unsafe {
                *output = ::conspire::math::TensorArray::as_array(
                    &::conspire::constitutive::solid::elastic::Elastic::first_piola_kirchhoff_stress(
                        &Model { $($field),+ },
                        &::std::slice::from_raw_parts(deformation_gradient, 9)[0].into(),
                    )
                    .unwrap(),
                );
            }
        }

        #[unsafe(export_name = concat!(stringify!($prefix), "_first_piola_kirchhoff_tangent_stiffness"))]
        unsafe extern "C" fn first_piola_kirchhoff_tangent_stiffness(
            $($field: ::conspire::math::Scalar,)+
            deformation_gradient: *const [[::conspire::math::Scalar; 3]; 3],
            output: *mut [[[[::conspire::math::Scalar; 3]; 3]; 3]; 3],
        ) {
            unsafe {
                *output = ::conspire::math::TensorArray::as_array(
                    &::conspire::constitutive::solid::elastic::Elastic::first_piola_kirchhoff_tangent_stiffness(
                        &Model { $($field),+ },
                        &::std::slice::from_raw_parts(deformation_gradient, 9)[0].into(),
                    )
                    .unwrap(),
                );
            }
        }

        #[unsafe(export_name = concat!(stringify!($prefix), "_second_piola_kirchhoff_stress"))]
        unsafe extern "C" fn second_piola_kirchhoff_stress(
            $($field: ::conspire::math::Scalar,)+
            deformation_gradient: *const [[::conspire::math::Scalar; 3]; 3],
            output: *mut [[::conspire::math::Scalar; 3]; 3],
        ) {
            unsafe {
                *output = ::conspire::math::TensorArray::as_array(
                    &::conspire::constitutive::solid::elastic::Elastic::second_piola_kirchhoff_stress(
                        &Model { $($field),+ },
                        &::std::slice::from_raw_parts(deformation_gradient, 9)[0].into(),
                    )
                    .unwrap(),
                );
            }
        }

        #[unsafe(export_name = concat!(stringify!($prefix), "_second_piola_kirchhoff_tangent_stiffness"))]
        unsafe extern "C" fn second_piola_kirchhoff_tangent_stiffness(
            $($field: ::conspire::math::Scalar,)+
            deformation_gradient: *const [[::conspire::math::Scalar; 3]; 3],
            output: *mut [[[[::conspire::math::Scalar; 3]; 3]; 3]; 3],
        ) {
            unsafe {
                *output = ::conspire::math::TensorArray::as_array(
                    &::conspire::constitutive::solid::elastic::Elastic::second_piola_kirchhoff_tangent_stiffness(
                        &Model { $($field),+ },
                        &::std::slice::from_raw_parts(deformation_gradient, 9)[0].into(),
                    )
                    .unwrap(),
                );
            }
        }
    };
}

macro_rules! elastic_ffi {
    ($prefix:ident, $model:ident { $($field:ident),+ $(,)? }) => {
        mod $prefix {
            $crate::constitutive::solid::elastic::elastic_ffi_items!(
                $prefix,
                ::conspire::constitutive::solid::elastic::$model { $($field),+ }
            );
        }
    };
}

pub(crate) use elastic_ffi_items;

elastic_ffi!(almansi_hamel, AlmansiHamel { bulk_modulus, shear_modulus });
elastic_ffi!(hencky_elastic, Hencky { bulk_modulus, shear_modulus });
elastic_ffi!(
    saint_venant_kirchhoff_elastic,
    SaintVenantKirchhoff { bulk_modulus, shear_modulus }
);
