use crate::constitutive::ffi::hyperelastic_ffi;
use conspire::constitutive::solid::hyperelastic::MooneyRivlin;

hyperelastic_ffi!(mooney_rivlin, MooneyRivlin { bulk_modulus, shear_modulus, extra_modulus });
