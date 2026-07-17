use crate::constitutive::ffi::elastic_ffi;
use conspire::constitutive::solid::elastic::AlmansiHamel;

elastic_ffi!(almansi_hamel, AlmansiHamel { bulk_modulus, shear_modulus });
