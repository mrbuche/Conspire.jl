using DocStringExtensions

"""
The Gent hyperelastic constitutive model.[^1]

[^1]: A.N. Gent, [Rubber Chem. Technol. **69**, 59 (1996)](https://doi.org/10.5254/1.3538357).

**Parameters**
- The bulk modulus ``\\kappa``.
- The shear modulus ``\\mu``.
- The extensibility ``J_m``.

**External variables**
- The deformation gradient ``\\mathbf{F}``.

**Internal variables**
- None.

**Notes**
- The Gent model reduces to the [`NeoHookean`](@ref) model when ``J_m\\to\\infty``.
"""
struct Gent
    κ::Real
    μ::Real
    Jₘ::Real
end

"""
$(TYPEDSIGNATURES)
```math
\\boldsymbol{\\sigma}(\\mathbf{F}) = \\frac{J^{-1}\\mu J_m {\\mathbf{B}^* }'}{J_m - \\mathrm{tr}(\\mathbf{B}^* ) + 3} + \\frac{\\kappa}{2}\\left(J - \\frac{1}{J}\\right)\\mathbf{1}
```
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
```math
\\mathcal{T}_{ijkL}(\\mathbf{F}) = \\frac{J^{-5/3}\\mu J_m}{J_m - \\mathrm{tr}(\\mathbf{B}^* ) + 3}\\Bigg[ \\delta_{ik}F_{jL} + \\delta_{jk}F_{iL} - \\frac{2}{3}\\,\\delta_{ij}F_{kL} + \\frac{2{B_{ij}^* }' F_{kL}}{J_m - \\mathrm{tr}(\\mathbf{B}^* ) + 3} - \\left(\\frac{5}{3} + \\frac{2}{3}\\frac{\\mathrm{tr}(\\mathbf{B}^* )}{J_m - \\mathrm{tr}(\\mathbf{B}^* ) + 3}\\right) J^{2/3} {B_{ij}^* }' F_{kL}^{-T} \\Bigg] + \\frac{\\kappa}{2} \\left(J + \\frac{1}{J}\\right)\\delta_{ij}F_{kL}^{-T}
```
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
```math
a(\\mathbf{F}) = -\\frac{\\mu J_m}{2}\\,\\ln\\left[1 - \\frac{\\mathrm{tr}(\\mathbf{B}^* ) - 3}{J_m}\\right] + \\frac{\\kappa}{2}\\left[\\frac{1}{2}\\left(J^2 - 1\\right) - \\ln J\\right]
```
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
