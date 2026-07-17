using DocStringExtensions

using ....Conspire: PROJECT_ROOT

const GENT_DOC = replace(
    read(joinpath(PROJECT_ROOT, "src/constitutive/solid/hyperelastic/gent/doc.md"), String),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
    "[Neo-Hookean model](super::NeoHookean)" => "[Neo-Hookean model](@ref Neo-Hookean) model",
)
const GENT_CAUCHY_STRESS =
    read("src/constitutive/solid/hyperelastic/gent/cauchy_stress.md", String)
const GENT_CAUCHY_TANGENT_STIFFNESS =
    read("src/constitutive/solid/hyperelastic/gent/cauchy_tangent_stiffness.md", String)
const GENT_HELMHOLTZ_FREE_ENERGY_DENSITY = read(
    "src/constitutive/solid/hyperelastic/gent/helmholtz_free_energy_density.md",
    String,
)

"""
$(GENT_DOC)
"""
struct Gent
    κ::Real
    μ::Real
    Jₘ::Real
end

"""
$(TYPEDSIGNATURES)
$(GENT_CAUCHY_STRESS)
"""
function cauchy_stress(model::Gent, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:gent_cauchy_stress, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.Jₘ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
$(GENT_CAUCHY_TANGENT_STIFFNESS)
"""
function cauchy_tangent_stiffness(model::Gent, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:gent_cauchy_tangent_stiffness, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.Jₘ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_stress(model::Gent, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:gent_first_piola_kirchhoff_stress, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.Jₘ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_tangent_stiffness(model::Gent, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:gent_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.Jₘ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_stress(model::Gent, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:gent_second_piola_kirchhoff_stress, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.Jₘ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_tangent_stiffness(model::Gent, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:gent_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.Jₘ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
$(GENT_HELMHOLTZ_FREE_ENERGY_DENSITY)
"""
function helmholtz_free_energy_density(model::Gent, F)
    return ccall(
        (:gent_helmholtz_free_energy_density, CONSPIRE_LIB),
        Float64,
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.Jₘ,
        F,
    )
end
