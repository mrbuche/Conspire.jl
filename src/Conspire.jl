module Conspire

const CONSPIRE_WRAPPER = string(dirname(@__FILE__), "/../deps/conspire_wrapper/")
const CONSPIRE_WRAPPER_LIB = string(CONSPIRE_WRAPPER, "target/release/libconspire_wrapper")

include("math.jl")

export lambert_w, langevin, inverse_langevin

include("constitutive.jl")

export cauchy_stress, cauchy_tangent_stiffness
export first_piola_kirchhoff_stress, first_piola_kirchhoff_tangent_stiffness
export second_piola_kirchhoff_stress, second_piola_kirchhoff_tangent_stiffness
export helmholtz_free_energy_density
export AlmansiHamel
export ArrudaBoyce, Fung, Gent, MooneyRivlin, NeoHookean, SaintVenantKirchhoff

end
