using DocStringExtensions

using ....Conspire: PROJECT_ROOT

const FUNG_DOC = replace(
    read(joinpath(PROJECT_ROOT, "src/constitutive/solid/hyperelastic/fung/doc.md"), String),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
    "[Neo-Hookean model](super::NeoHookean)" => "[Neo-Hookean model](@ref Neo-Hookean) model",
)
const FUNG_CAUCHY_STRESS = read(
    joinpath(PROJECT_ROOT, "src/constitutive/solid/hyperelastic/fung/cauchy_stress.md"),
    String,
)
const FUNG_CAUCHY_TANGENT_STIFFNESS = read(
    joinpath(
        PROJECT_ROOT,
        "src/constitutive/solid/hyperelastic/fung/cauchy_tangent_stiffness.md",
    ),
    String,
)
const FUNG_HELMHOLTZ_FREE_ENERGY_DENSITY = read(
    joinpath(
        PROJECT_ROOT,
        "src/constitutive/solid/hyperelastic/fung/helmholtz_free_energy_density.md",
    ),
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
    output = zeros(Float64, 3, 3)
    ccall(
        (:fung_cauchy_stress, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        model.η,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
$(FUNG_CAUCHY_TANGENT_STIFFNESS)
"""
function cauchy_tangent_stiffness(model::Fung, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:fung_cauchy_tangent_stiffness, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        model.η,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_stress(model::Fung, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:fung_first_piola_kirchhoff_stress, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        model.η,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_tangent_stiffness(model::Fung, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:fung_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        model.η,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_stress(model::Fung, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:fung_second_piola_kirchhoff_stress, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        model.η,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_tangent_stiffness(model::Fung, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:fung_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        model.η,
        F,
        output,
    )
    return output
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
