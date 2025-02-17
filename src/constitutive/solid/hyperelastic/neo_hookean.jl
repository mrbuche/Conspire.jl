using DocStringExtensions

"""
The Neo-Hookean hyperelastic constitutive model.[^1]

[^1]: R.S. Rivlin, [Philos. Trans. R. Soc. London, Ser. A **240**, 459 (1948)](https://doi.org/10.1098/rsta.1948.0002).

**Parameters**
- The bulk modulus ``\\kappa``.
- The shear modulus ``\\mu``.

**External variables**
- The deformation gradient ``\\mathbf{F}``.

**Internal variables**
- None.
"""
struct NeoHookean
    κ::Real
    μ::Real
end

"""
$(TYPEDSIGNATURES)
```math
\\boldsymbol{\\sigma}(\\mathbf{F}) = \\frac{\\mu}{J}\\,{\\mathbf{B}^*}' + \\frac{\\kappa}{2}\\left(J - \\frac{1}{J}\\right)\\mathbf{1}
```
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
```math
\\mathcal{T}_{ijkL}(\\mathbf{F}) = \\frac{\\mu}{J^{5/3}}\\left(\\delta_{ik}F_{jL} + \\delta_{jk}F_{iL} - \\frac{2}{3}\\,\\delta_{ij}F_{kL} - \\frac{5}{3} \\, B_{ij}'F_{kL}^{-T} \\right) + \\frac{\\kappa}{2} \\left(J + \\frac{1}{J}\\right)\\delta_{ij}F_{kL}^{-T}
```
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
```math
a(\\mathbf{F}) = \\frac{\\mu}{2}\\left[\\mathrm{tr}(\\mathbf{B}^*) - 3\\right] + \\frac{\\kappa}{2}\\left[\\frac{1}{2}\\left(J^2 - 1\\right) - \\ln J\\right]
```
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
