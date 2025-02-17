using DocStringExtensions

"""
The Mooney-Rivlin hyperelastic constitutive model.[^1][^2]

[^1]: M. Mooney, [J. Appl. Phys. **11**, 582 (1940)](https://doi.org/10.1063/1.1712836).
[^2]: R.S. Rivlin, [Philos. Trans. R. Soc. London, Ser. A **241**, 379 (1948)](https://doi.org/10.1098/rsta.1948.0024).

**Parameters**
- The bulk modulus ``\\kappa``.
- The shear modulus ``\\mu``.
- The extra modulus ``\\mu_m``.

**External variables**
- The deformation gradient ``\\mathbf{F}``.

**Internal variables**
- None.

**Notes**
- The Mooney-Rivlin model reduces to the [`NeoHookean`](@ref) model when ``\\mu_m\\to 0``.
"""
struct MooneyRivlin
    κ::Real
    μ::Real
    μₘ::Real
end

"""
$(TYPEDSIGNATURES)
```math
\\boldsymbol{\\sigma}(\\mathbf{F}) = \\frac{\\mu - \\mu_m}{J}\\, {\\mathbf{B}^* }' - \\frac{\\mu_m}{J}\\left(\\mathbf{B}^{* -1}\\right)' + \\frac{\\kappa}{2}\\left(J - \\frac{1}{J}\\right)\\mathbf{1}
```
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
```math
\\mathcal{T}_{ijkL}(\\mathbf{F}) = \\frac{\\mu-\\mu_m}{J^{5/3}}\\left(\\delta_{ik}F_{jL} + \\delta_{jk}F_{iL} - \\frac{2}{3}\\,\\delta_{ij}F_{kL}- \\frac{5}{3} \\, B_{ij}'F_{kL}^{-T} \\right) - \\frac{\\mu_m}{J}\\left[ \\frac{2}{3}\\,B_{ij}^{* -1}F_{kL}^{-T} - B_{ik}^{* -1}F_{jL}^{-T} - B_{ik}^{* -1}F_{iL}^{-T} + \\frac{2}{3}\\,\\delta_{ij}\\left(B_{km}^{* -1}\\right)'F_{mL}^{-T} - \\left(B_{ij}^{* -1}\\right)'F_{kL}^{-T} \\right] + \\frac{\\kappa}{2} \\left(J + \\frac{1}{J}\\right)\\delta_{ij}F_{kL}^{-T}
```
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
```math
a(\\mathbf{F}) = \\frac{\\mu - \\mu_m}{2}\\left[\\mathrm{tr}(\\mathbf{B}^* ) - 3\\right] + \\frac{\\mu_m}{2}\\left[I_2(\\mathbf{B}^*) - 3\\right] + \\frac{\\kappa}{2}\\left[\\frac{1}{2}\\left(J^2 - 1\\right) - \\ln J\\right]
```
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
