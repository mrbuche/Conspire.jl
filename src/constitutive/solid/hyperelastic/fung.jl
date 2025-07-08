using DocStringExtensions

using ....Conspire: PROJECT_ROOT

const FUNGMODEL = replace(
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/fung/model.md",
        ),
        String,
    ),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
    "[Neo-Hookean model](super::NeoHookean)" => "[Neo-Hookean model](@ref Neo-Hookean) model",
)
const FUNGCAUCHYSTRESS = read(
    joinpath(
        PROJECT_ROOT,
        "conspire.rs/src/constitutive/solid/hyperelastic/fung/cauchy_stress.md",
    ),
    String,
)
const FUNGCAUCHYTANGENTSTIFFNESS = read(
    joinpath(
        PROJECT_ROOT,
        "conspire.rs/src/constitutive/solid/hyperelastic/fung/cauchy_tangent_stiffness.md",
    ),
    String,
)
const FUNGHELMHOLTZFREEENERGYDENSITY = read(
    joinpath(
        PROJECT_ROOT,
        "conspire.rs/src/constitutive/solid/hyperelastic/fung/helmholtz_free_energy_density.md",
    ),
    String,
)

"""
$(FUNGMODEL)
"""
struct Fung
    κ::Real
    μ::Real
    μₘ::Real
    η::Real
end

"""
$(TYPEDSIGNATURES)
$(FUNGCAUCHYSTRESS)
"""
function cauchy_stress(model::Fung, F)
    raw = ccall(
        (:fung_cauchy_stress, CONSPIRE_WRAPPER_LIB),
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
$(FUNGCAUCHYTANGENTSTIFFNESS)
"""
function cauchy_tangent_stiffness(model::Fung, F)
    raw = ccall(
        (:fung_cauchy_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
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
        (:fung_first_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
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
        (:fung_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
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
        (:fung_second_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
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
        (:fung_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
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
$(FUNGHELMHOLTZFREEENERGYDENSITY)
"""
function helmholtz_free_energy_density(model::Fung, F)
    return ccall(
        (:fung_helmholtz_free_energy_density, CONSPIRE_WRAPPER_LIB),
        Float64,
        (Float64, Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        model.η,
        F,
    )
end
