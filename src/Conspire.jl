module Conspire

const CONSPIRE_LIB = string(dirname(@__FILE__), "/../target/release/libconspire_jl")
const PROJECT_ROOT = string(dirname(@__FILE__), "/../")

include("math.jl")

export lambert_w, langevin, inverse_langevin, rosenbrock

include("constitutive.jl")

export cauchy_stress, cauchy_tangent_stiffness
export first_piola_kirchhoff_stress, first_piola_kirchhoff_tangent_stiffness
export second_piola_kirchhoff_stress, second_piola_kirchhoff_tangent_stiffness
export helmholtz_free_energy_density
export AlmansiHamel
export ArrudaBoyce, Fung, Gent, MooneyRivlin, NeoHookean, SaintVenantKirchhoff

end
