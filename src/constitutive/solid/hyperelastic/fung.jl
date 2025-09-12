using DocStringExtensions

using ....Conspire: PROJECT_ROOT

const FUNG_DOC = replace(
    read(joinpath(PROJECT_ROOT, "src/constitutive/solid/hyperelastic/fung/doc.md"), String),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
    "[Neo-Hookean model](super::NeoHookean)" => "[Neo-Hookean model](@ref Neo-Hookean) model",
)
const FUNG_CAUCHY_STRESS =
    read("src/constitutive/solid/hyperelastic/fung/cauchy_stress.md", String)
const FUNG_CAUCHY_TANGENT_STIFFNESS =
    read("src/constitutive/solid/hyperelastic/fung/cauchy_tangent_stiffness.md", String)
const FUNG_HELMHOLTZ_FREE_ENERGY_DENSITY = read(
    "src/constitutive/solid/hyperelastic/fung/helmholtz_free_energy_density.md",
    String,
)

"""
$(FUNG_DOC)
"""
struct Fung
    κ::Real
    μ::Real
    μₘ::Real
    η::Real
end

"""
$(TYPEDSIGNATURES)
$(FUNG_CAUCHY_STRESS)
"""
function cauchy_stress(model::Fung, F)
    raw = ccall(
        (:fung_cauchy_stress, CONSPIRE_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        model.η,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
$(FUNG_CAUCHY_TANGENT_STIFFNESS)
"""
function cauchy_tangent_stiffness(model::Fung, F)
    raw = ccall(
        (:fung_cauchy_tangent_stiffness, CONSPIRE_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        model.η,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_stress(model::Fung, F)
    raw = ccall(
        (:fung_first_piola_kirchhoff_stress, CONSPIRE_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        model.η,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_tangent_stiffness(model::Fung, F)
    raw = ccall(
        (:fung_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        model.η,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_stress(model::Fung, F)
    raw = ccall(
        (:fung_second_piola_kirchhoff_stress, CONSPIRE_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        model.η,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_tangent_stiffness(model::Fung, F)
    raw = ccall(
        (:fung_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        model.η,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
$(FUNG_HELMHOLTZ_FREE_ENERGY_DENSITY)
"""
function helmholtz_free_energy_density(model::Fung, F)
    return ccall(
        (:fung_helmholtz_free_energy_density, CONSPIRE_LIB),
        Float64,
        (Float64, Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        model.η,
        F,
    )
end
