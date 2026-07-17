use crate::constitutive::ffi::hyperelastic_ffi;
use conspire::constitutive::solid::hyperelastic::Fung;

hyperelastic_ffi!(fung, Fung { bulk_modulus, shear_modulus, extra_modulus, exponent });
