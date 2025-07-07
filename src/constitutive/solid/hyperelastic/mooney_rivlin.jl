using DocStringExtensions

const MOONEYRIVLINMODEL = replace(
    read("conspire.rs/src/constitutive/solid/hyperelastic/mooney_rivlin/model.md", String),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
    "[Neo-Hookean model](super::NeoHookean)" => "[Neo-Hookean model](@ref Neo-Hookean) model",
    "<sup>,</sup>" => "",
)
const MOONEYRIVLINCAUCHYSTRESS = read(
    "conspire.rs/src/constitutive/solid/hyperelastic/mooney_rivlin/cauchy_stress.md",
    String,
)
const MOONEYRIVLINCAUCHYTANGENTSTIFFNESS = read(
    "conspire.rs/src/constitutive/solid/hyperelastic/mooney_rivlin/cauchy_tangent_stiffness.md",
    String,
)
const MOONEYRIVLINHELMHOLTZFREEENERGYDENSITY = read(
    "conspire.rs/src/constitutive/solid/hyperelastic/mooney_rivlin/helmholtz_free_energy_density.md",
    String,
)

"""
$(MOONEYRIVLINMODEL)
"""
struct MooneyRivlin
    κ::Real
    μ::Real
    μₘ::Real
end

"""
$(TYPEDSIGNATURES)
$(MOONEYRIVLINCAUCHYSTRESS)
"""
function cauchy_stress(model::MooneyRivlin, F)
    raw = ccall(
        (:mooney_rivlin_cauchy_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
$(MOONEYRIVLINCAUCHYTANGENTSTIFFNESS)
"""
function cauchy_tangent_stiffness(model::MooneyRivlin, F)
    raw = ccall(
        (:mooney_rivlin_cauchy_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_stress(model::MooneyRivlin, F)
    raw = ccall(
        (:mooney_rivlin_first_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_tangent_stiffness(model::MooneyRivlin, F)
    raw = ccall(
        (:mooney_rivlin_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_stress(model::MooneyRivlin, F)
    raw = ccall(
        (:mooney_rivlin_second_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_tangent_stiffness(model::MooneyRivlin, F)
    raw = ccall(
        (:mooney_rivlin_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
$(MOONEYRIVLINHELMHOLTZFREEENERGYDENSITY)
"""
function helmholtz_free_energy_density(model::MooneyRivlin, F)
    return ccall(
        (:mooney_rivlin_helmholtz_free_energy_density, CONSPIRE_WRAPPER_LIB),
        Float64,
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₘ,
        F,
    )
end
