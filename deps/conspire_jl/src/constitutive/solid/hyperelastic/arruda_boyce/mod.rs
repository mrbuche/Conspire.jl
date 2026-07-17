use crate::constitutive::ffi::hyperelastic_ffi;
use conspire::constitutive::solid::hyperelastic::ArrudaBoyce;

hyperelastic_ffi!(arruda_boyce, ArrudaBoyce { bulk_modulus, shear_modulus, number_of_links });
