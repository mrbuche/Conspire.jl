using DocStringExtensions

using ....Conspire: PROJECT_ROOT

const ARRUDA_BOYCE_DOC = replace(
    read(
        joinpath(PROJECT_ROOT, "src/constitutive/solid/hyperelastic/arruda_boyce/doc.md"),
        String,
    ),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
    "[Neo-Hookean model](super::NeoHookean)" => "[Neo-Hookean model](@ref Neo-Hookean) model",
)
const ARRUDA_BOYCE_CAUCHY_STRESS =
    read("src/constitutive/solid/hyperelastic/arruda_boyce/cauchy_stress.md", String)
const ARRUDA_BOYCE_CAUCHY_TANGENT_STIFFNESS = read(
    "src/constitutive/solid/hyperelastic/arruda_boyce/cauchy_tangent_stiffness.md",
    String,
)
const ARRUDA_BOYCE_HELMHOLTZ_FREE_ENERGY_DENSITY = read(
    "src/constitutive/solid/hyperelastic/arruda_boyce/helmholtz_free_energy_density.md",
    String,
)

"""
$(ARRUDA_BOYCE_DOC)
"""
struct ArrudaBoyce
    κ::Real
    μ::Real
    N::Real
end

"""
$(TYPEDSIGNATURES)
$(ARRUDA_BOYCE_CAUCHY_STRESS)
"""
function cauchy_stress(model::ArrudaBoyce, F)
    raw = ccall(
        (:arruda_boyce_cauchy_stress, CONSPIRE_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.N,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
$(ARRUDA_BOYCE_CAUCHY_TANGENT_STIFFNESS)
"""
function cauchy_tangent_stiffness(model::ArrudaBoyce, F)
    raw = ccall(
        (:arruda_boyce_cauchy_tangent_stiffness, CONSPIRE_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.N,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_stress(model::ArrudaBoyce, F)
    raw = ccall(
        (:arruda_boyce_first_piola_kirchhoff_stress, CONSPIRE_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.N,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_tangent_stiffness(model::ArrudaBoyce, F)
    raw = ccall(
        (:arruda_boyce_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.N,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_stress(model::ArrudaBoyce, F)
    raw = ccall(
        (:arruda_boyce_second_piola_kirchhoff_stress, CONSPIRE_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.N,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_tangent_stiffness(model::ArrudaBoyce, F)
    raw = ccall(
        (:arruda_boyce_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.N,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
$(ARRUDA_BOYCE_HELMHOLTZ_FREE_ENERGY_DENSITY)
"""
function helmholtz_free_energy_density(model::ArrudaBoyce, F)
    return ccall(
        (:arruda_boyce_helmholtz_free_energy_density, CONSPIRE_LIB),
        Float64,
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.N,
        F,
    )
end
