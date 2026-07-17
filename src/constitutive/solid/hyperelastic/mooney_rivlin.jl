using DocStringExtensions

using ....Conspire: PROJECT_ROOT

const MOONEY_RIVLIN_DOC = replace(
    read(
        joinpath(PROJECT_ROOT, "src/constitutive/solid/hyperelastic/mooney_rivlin/doc.md"),
        String,
    ),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
    "[Neo-Hookean model](super::NeoHookean)" => "[Neo-Hookean model](@ref Neo-Hookean) model",
)
const MOONEY_RIVLIN_CAUCHY_STRESS =
    read("src/constitutive/solid/hyperelastic/mooney_rivlin/cauchy_stress.md", String)
const MOONEY_RIVLIN_CAUCHY_TANGENT_STIFFNESS = read(
    "src/constitutive/solid/hyperelastic/mooney_rivlin/cauchy_tangent_stiffness.md",
    String,
)
const MOONEY_RIVLIN_HELMHOLTZ_FREE_ENERGY_DENSITY = read(
    "src/constitutive/solid/hyperelastic/mooney_rivlin/helmholtz_free_energy_density.md",
    String,
)

"""
$(MOONEY_RIVLIN_DOC)
"""
struct MooneyRivlin
    κ::Real
    μ::Real
    μₘ::Real
end

"""
$(TYPEDSIGNATURES)
$(MOONEY_RIVLIN_CAUCHY_STRESS)
"""
function cauchy_stress(model::MooneyRivlin, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:mooney_rivlin_cauchy_stress, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
$(MOONEY_RIVLIN_CAUCHY_TANGENT_STIFFNESS)
"""
function cauchy_tangent_stiffness(model::MooneyRivlin, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:mooney_rivlin_cauchy_tangent_stiffness, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_stress(model::MooneyRivlin, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:mooney_rivlin_first_piola_kirchhoff_stress, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_tangent_stiffness(model::MooneyRivlin, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:mooney_rivlin_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_stress(model::MooneyRivlin, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:mooney_rivlin_second_piola_kirchhoff_stress, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_tangent_stiffness(model::MooneyRivlin, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:mooney_rivlin_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
$(MOONEY_RIVLIN_HELMHOLTZ_FREE_ENERGY_DENSITY)
"""
function helmholtz_free_energy_density(model::MooneyRivlin, F)
    return ccall(
        (:mooney_rivlin_helmholtz_free_energy_density, CONSPIRE_LIB),
        Float64,
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        F,
    )
end
