macro_rules! elastic_ffi {
    ($prefix:ident, $model:ident { $($field:ident),+ $(,)? }) => {
        #[unsafe(export_name = concat!(stringify!($prefix), "_cauchy_stress"))]
        unsafe extern "C" fn ffi_cauchy_stress(
            $($field: ::conspire::math::Scalar,)+
            deformation_gradient: *const [[::conspire::math::Scalar; 3]; 3],
            output: *mut [[::conspire::math::Scalar; 3]; 3],
        ) {
            unsafe {
                *output = ::conspire::math::TensorArray::as_array(
                    &::conspire::constitutive::solid::elastic::Elastic::cauchy_stress(
                        &$model { $($field),+ },
                        &::std::slice::from_raw_parts(deformation_gradient, 9)[0].into(),
                    )
                    .unwrap(),
                );
            }
        }

        #[unsafe(export_name = concat!(stringify!($prefix), "_cauchy_tangent_stiffness"))]
        unsafe extern "C" fn ffi_cauchy_tangent_stiffness(
            $($field: ::conspire::math::Scalar,)+
            deformation_gradient: *const [[::conspire::math::Scalar; 3]; 3],
            output: *mut [[[[::conspire::math::Scalar; 3]; 3]; 3]; 3],
        ) {
            unsafe {
                *output = ::conspire::math::TensorArray::as_array(
                    &::conspire::constitutive::solid::elastic::Elastic::cauchy_tangent_stiffness(
                        &$model { $($field),+ },
                        &::std::slice::from_raw_parts(deformation_gradient, 9)[0].into(),
                    )
                    .unwrap(),
                );
            }
        }

        #[unsafe(export_name = concat!(stringify!($prefix), "_first_piola_kirchhoff_stress"))]
        unsafe extern "C" fn ffi_first_piola_kirchhoff_stress(
            $($field: ::conspire::math::Scalar,)+
            deformation_gradient: *const [[::conspire::math::Scalar; 3]; 3],
            output: *mut [[::conspire::math::Scalar; 3]; 3],
        ) {
            unsafe {
                *output = ::conspire::math::TensorArray::as_array(
                    &::conspire::constitutive::solid::elastic::Elastic::first_piola_kirchhoff_stress(
                        &$model { $($field),+ },
                        &::std::slice::from_raw_parts(deformation_gradient, 9)[0].into(),
                    )
                    .unwrap(),
                );
            }
        }

        #[unsafe(export_name = concat!(stringify!($prefix), "_first_piola_kirchhoff_tangent_stiffness"))]
        unsafe extern "C" fn ffi_first_piola_kirchhoff_tangent_stiffness(
            $($field: ::conspire::math::Scalar,)+
            deformation_gradient: *const [[::conspire::math::Scalar; 3]; 3],
            output: *mut [[[[::conspire::math::Scalar; 3]; 3]; 3]; 3],
        ) {
            unsafe {
                *output = ::conspire::math::TensorArray::as_array(
                    &::conspire::constitutive::solid::elastic::Elastic::first_piola_kirchhoff_tangent_stiffness(
                        &$model { $($field),+ },
                        &::std::slice::from_raw_parts(deformation_gradient, 9)[0].into(),
                    )
                    .unwrap(),
                );
            }
        }

        #[unsafe(export_name = concat!(stringify!($prefix), "_second_piola_kirchhoff_stress"))]
        unsafe extern "C" fn ffi_second_piola_kirchhoff_stress(
            $($field: ::conspire::math::Scalar,)+
            deformation_gradient: *const [[::conspire::math::Scalar; 3]; 3],
            output: *mut [[::conspire::math::Scalar; 3]; 3],
        ) {
            unsafe {
                *output = ::conspire::math::TensorArray::as_array(
                    &::conspire::constitutive::solid::elastic::Elastic::second_piola_kirchhoff_stress(
                        &$model { $($field),+ },
                        &::std::slice::from_raw_parts(deformation_gradient, 9)[0].into(),
                    )
                    .unwrap(),
                );
            }
        }

        #[unsafe(export_name = concat!(stringify!($prefix), "_second_piola_kirchhoff_tangent_stiffness"))]
        unsafe extern "C" fn ffi_second_piola_kirchhoff_tangent_stiffness(
            $($field: ::conspire::math::Scalar,)+
            deformation_gradient: *const [[::conspire::math::Scalar; 3]; 3],
            output: *mut [[[[::conspire::math::Scalar; 3]; 3]; 3]; 3],
        ) {
            unsafe {
                *output = ::conspire::math::TensorArray::as_array(
                    &::conspire::constitutive::solid::elastic::Elastic::second_piola_kirchhoff_tangent_stiffness(
                        &$model { $($field),+ },
                        &::std::slice::from_raw_parts(deformation_gradient, 9)[0].into(),
                    )
                    .unwrap(),
                );
            }
        }
    };
}

macro_rules! hyperelastic_ffi {
    ($prefix:ident, $model:ident { $($field:ident),+ $(,)? }) => {
        $crate::constitutive::ffi::elastic_ffi!($prefix, $model { $($field),+ });

        #[unsafe(export_name = concat!(stringify!($prefix), "_helmholtz_free_energy_density"))]
        unsafe extern "C" fn ffi_helmholtz_free_energy_density(
            $($field: ::conspire::math::Scalar,)+
            deformation_gradient: *const [[::conspire::math::Scalar; 3]; 3],
        ) -> ::conspire::math::Scalar {
            unsafe {
                ::conspire::constitutive::solid::hyperelastic::Hyperelastic::helmholtz_free_energy_density(
                    &$model { $($field),+ },
                    &::std::slice::from_raw_parts(deformation_gradient, 9)[0].into(),
                )
                .unwrap()
            }
        }
    };
}

pub(crate) use elastic_ffi;
pub(crate) use hyperelastic_ffi;
