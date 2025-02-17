using DocStringExtensions

"""
The Fung hyperelastic constitutive model.[^1]

[^1]: Y.C. Fung, [Am. J. Physiol. **213**, 1532 (1967)](https://doi.org/10.1152/ajplegacy.1967.213.6.1532).

**Parameters**
- The bulk modulus ``\\kappa``.
- The shear modulus ``\\mu``.
- The extra modulus ``\\mu_m``.
- The exponent ``c``.

**External variables**
- The deformation gradient ``\\mathbf{F}``.

**Internal variables**
- None.

**Notes**
- The Fung model reduces to the [`NeoHookean`](@ref) model when ``\\mu_m\\to 0`` or ``c\\to 0``.
"""
struct Fung
    κ::Real
    μ::Real
    μₘ::Real
    η::Real
end

"""
$(TYPEDSIGNATURES)
```math
\\boldsymbol{\\sigma}(\\mathbf{F}) = \\frac{1}{J}\\left[\\mu + \\mu_m\\left(e^{c[\\mathrm{tr}(\\mathbf{B}^* ) - 3]} - 1\\right)\\right]{\\mathbf{B}^* }' + \\frac{\\kappa}{2}\\left(J - \\frac{1}{J}\\right)\\mathbf{1}
```
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
```math
\\mathcal{T}_{ijkL}(\\mathbf{F}) = \\frac{1}{J^{5/3}}\\left[\\mu + \\mu_m\\left(e^{c[\\mathrm{tr}(\\mathbf{B}^* ) - 3]} - 1\\right)\\right]\\left(\\delta_{ik}F_{jL} + \\delta_{jk}F_{iL} - \\frac{2}{3}\\,\\delta_{ij}F_{kL} - \\frac{5}{3} \\, B_{ij}'F_{kL}^{-T} \\right) + \\frac{2c\\mu_m}{J^{7/3}}\\,e^{c[\\mathrm{tr}(\\mathbf{B}^* ) - 3]}B_{ij}'B_{km}'F_{mL}^{-T} + \\frac{\\kappa}{2} \\left(J + \\frac{1}{J}\\right)\\delta_{ij}F_{kL}^{-T}
```
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
```math
a(\\mathbf{F}) = \\frac{\\mu - \\mu_m}{2}\\left[\\mathrm{tr}(\\mathbf{B}^* ) - 3\\right] + \\frac{\\mu_m}{2c}\\left(e^{c[\\mathrm{tr}(\\mathbf{B}^* ) - 3]} - 1\\right)
```
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
