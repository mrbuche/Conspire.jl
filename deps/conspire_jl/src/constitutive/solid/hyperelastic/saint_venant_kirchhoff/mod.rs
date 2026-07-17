use crate::constitutive::ffi::hyperelastic_ffi;
use conspire::constitutive::solid::hyperelastic::SaintVenantKirchhoff;

hyperelastic_ffi!(saint_venant_kirchhoff, SaintVenantKirchhoff { bulk_modulus, shear_modulus });
