use crate::constitutive::ffi::hyperelastic_ffi;
use conspire::constitutive::solid::hyperelastic::Gent;

hyperelastic_ffi!(gent, Gent { bulk_modulus, shear_modulus, extensibility });
