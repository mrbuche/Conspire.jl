using DocStringExtensions

using ....Conspire: PROJECT_ROOT

const GENTMODEL = replace(
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/gent/model.md",
        ),
        String,
    ),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
    "[Neo-Hookean model](super::NeoHookean)" => "[Neo-Hookean model](@ref Neo-Hookean) model",
)
const GENTCAUCHYSTRESS = read(
    joinpath(
        PROJECT_ROOT,
        "conspire.rs/src/constitutive/solid/hyperelastic/gent/cauchy_stress.md",
    ),
    String,
)
const GENTCAUCHYTANGENTSTIFFNESS = read(
    joinpath(
        PROJECT_ROOT,
        "conspire.rs/src/constitutive/solid/hyperelastic/gent/cauchy_tangent_stiffness.md",
    ),
    String,
)
const GENTHELMHOLTZFREEENERGYDENSITY = read(
    joinpath(
        PROJECT_ROOT,
        "conspire.rs/src/constitutive/solid/hyperelastic/gent/helmholtz_free_energy_density.md",
    ),
    String,
)

"""
$(GENTMODEL)
"""
struct Gent
    κ::Real
    μ::Real
    Jₘ::Real
end

"""
$(TYPEDSIGNATURES)
$(GENTCAUCHYSTRESS)
"""
function cauchy_stress(model::Gent, F)
    raw = ccall(
        (:gent_cauchy_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.Jₘ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
$(GENTCAUCHYTANGENTSTIFFNESS)
"""
function cauchy_tangent_stiffness(model::Gent, F)
    raw = ccall(
        (:gent_cauchy_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.Jₘ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_stress(model::Gent, F)
    raw = ccall(
        (:gent_first_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.Jₘ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_tangent_stiffness(model::Gent, F)
    raw = ccall(
        (:gent_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.Jₘ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_stress(model::Gent, F)
    raw = ccall(
        (:gent_second_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.Jₘ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_tangent_stiffness(model::Gent, F)
    raw = ccall(
        (:gent_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.Jₘ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
$(GENTHELMHOLTZFREEENERGYDENSITY)
"""
function helmholtz_free_energy_density(model::Gent, F)
    return ccall(
        (:gent_helmholtz_free_energy_density, CONSPIRE_WRAPPER_LIB),
        Float64,
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.Jₘ,
        F,
    )
end
