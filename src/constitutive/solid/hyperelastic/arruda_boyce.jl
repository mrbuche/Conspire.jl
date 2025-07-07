using DocStringExtensions

const MODEL = replace(
    replace(
        read(
            "conspire.rs/src/constitutive/solid/hyperelastic/arruda_boyce/model.md",
            String,
        ),
        "\$\`" => "\`\`",
        "\`\$" => "\`\`",
    ),
    "[Neo-Hookean model](NeoHookean)" => "[Neo-Hookean model](@ref Neo-Hookean) model",
)
const CAUCHYSTRESS = read(
    "conspire.rs/src/constitutive/solid/hyperelastic/arruda_boyce/cauchy_stress.md",
    String,
)
const CAUCHYTANGENTSTIFFNESS = read(
    "conspire.rs/src/constitutive/solid/hyperelastic/arruda_boyce/cauchy_tangent_stiffness.md",
    String,
)
const HELMHOLTZFREEENERGYDENSITY = read(
    "conspire.rs/src/constitutive/solid/hyperelastic/arruda_boyce/helmholtz_free_energy_density.md",
    String,
)

"""
$(MODEL)
"""
struct ArrudaBoyce
    κ::Real
    μ::Real
    N::Real
end

"""
$(TYPEDSIGNATURES)
$(CAUCHYSTRESS)
"""
function cauchy_stress(model::ArrudaBoyce, F)
    raw = ccall(
        (:arruda_boyce_cauchy_stress, CONSPIRE_WRAPPER_LIB),
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
$(CAUCHYTANGENTSTIFFNESS)
"""
function cauchy_tangent_stiffness(model::ArrudaBoyce, F)
    raw = ccall(
        (:arruda_boyce_cauchy_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
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
        (:arruda_boyce_first_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
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
        (:arruda_boyce_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
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
        (:arruda_boyce_second_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
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
        (:arruda_boyce_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
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
$(HELMHOLTZFREEENERGYDENSITY)
"""
function helmholtz_free_energy_density(model::ArrudaBoyce, F)
    return ccall(
        (:arruda_boyce_helmholtz_free_energy_density, CONSPIRE_WRAPPER_LIB),
        Float64,
        (Float64, Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        model.N,
        F,
    )
end
