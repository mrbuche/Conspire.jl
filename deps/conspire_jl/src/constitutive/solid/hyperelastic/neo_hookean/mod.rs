use crate::constitutive::ffi::hyperelastic_ffi;
use conspire::constitutive::solid::hyperelastic::NeoHookean;

hyperelastic_ffi!(neo_hookean, NeoHookean { bulk_modulus, shear_modulus });
