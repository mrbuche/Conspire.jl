module Conspire

const CONSPIRE_WRAPPER = string(dirname(@__FILE__), "/../deps/conspire_wrapper/")
const CONSPIRE_WRAPPER_LIB = string(CONSPIRE_WRAPPER, "target/release/libconspire_wrapper")

include("constitutive.jl")

export cauchy_stress, cauchy_tangent_stiffness
export first_piola_kirchoff_stress, first_piola_kirchoff_tangent_stiffness
export second_piola_kirchoff_stress, second_piola_kirchoff_tangent_stiffness
export helmholtz_free_energy_density
export AlmansiHamel
export ArrudaBoyce, Fung, Gent, MooneyRivlin, NeoHookean, SaintVenantKirchoff, Yeoh

end
