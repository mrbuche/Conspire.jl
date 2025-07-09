using DocStringExtensions

using ....Conspire: PROJECT_ROOT

const NEO_HOOKEAN_DOC = replace(
    read(
        joinpath(PROJECT_ROOT, "src/constitutive/solid/hyperelastic/neo_hookean/doc.md"),
        String,
    ),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
)
const NEO_HOOKEAN_CAUCHY_STRESS =
    read("src/constitutive/solid/hyperelastic/neo_hookean/cauchy_stress.md", String)
const NEO_HOOKEAN_CAUCHY_TANGENT_STIFFNESS = read(
    "src/constitutive/solid/hyperelastic/neo_hookean/cauchy_tangent_stiffness.md",
    String,
)
const NEO_HOOKEAN_HELMHOLTZ_FREE_ENERGY_DENSITY = read(
    "src/constitutive/solid/hyperelastic/neo_hookean/helmholtz_free_energy_density.md",
    String,
)

"""
$(NEO_HOOKEAN_DOC)
"""
struct NeoHookean
    κ::Real
    μ::Real
end

"""
$(TYPEDSIGNATURES)
$(NEO_HOOKEAN_CAUCHY_STRESS)
"""
function cauchy_stress(model::NeoHookean, F)
    raw = ccall(
        (:neo_hookean_cauchy_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
$(NEO_HOOKEAN_CAUCHY_TANGENT_STIFFNESS)
"""
function cauchy_tangent_stiffness(model::NeoHookean, F)
    raw = ccall(
        (:neo_hookean_cauchy_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_stress(model::NeoHookean, F)
    raw = ccall(
        (:neo_hookean_first_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_tangent_stiffness(model::NeoHookean, F)
    raw = ccall(
        (:neo_hookean_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_stress(model::NeoHookean, F)
    raw = ccall(
        (:neo_hookean_second_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_tangent_stiffness(model::NeoHookean, F)
    raw = ccall(
        (:neo_hookean_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
$(NEO_HOOKEAN_HELMHOLTZ_FREE_ENERGY_DENSITY)
"""
function helmholtz_free_energy_density(model::NeoHookean, F)
    return ccall(
        (:neo_hookean_helmholtz_free_energy_density, CONSPIRE_WRAPPER_LIB),
        Float64,
        (Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
    )
end
