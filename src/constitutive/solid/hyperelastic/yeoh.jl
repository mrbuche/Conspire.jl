using DocStringExtensions

"""
The Yeoh hyperelastic constitutive model.[^1]

[^1]: O.H. Yeoh, [Rubber Chem. Technol. **66**, 754 (1993)](https://doi.org/10.5254/1.3538343).

**Parameters**
- The bulk modulus ``\\kappa``.
- The shear modulus ``\\mu``.
- The extra moduli ``\\mu_n`` for ``n=2\\ldots N``.

**External variables**
- The deformation gradient ``\\mathbf{F}``.

**Internal variables**
- None.

**Notes**
- The Yeoh model reduces to the [`NeoHookean`](@ref) model when ``\\mu_n\\to 0`` for ``n=2\\ldots N``.
"""
struct Yeoh
    κ::Real
    μ::Real
    μₑ::Vector
end

"""
$(TYPEDSIGNATURES)
```math
\\boldsymbol{\\sigma}(\\mathbf{F}) = \\sum_{n=1}^N \\frac{n\\mu_n}{J}\\left[\\mathrm{tr}(\\mathbf{B}^* ) - 3\\right]^{n-1}\\,{\\mathbf{B}^*}' + \\frac{\\kappa}{2}\\left(J - \\frac{1}{J}\\right)\\mathbf{1}
```
"""
function cauchy_stress(model::Yeoh, F)
    raw = ccall(
        (:yeoh_cauchy_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}, UInt8, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₑ,
        length(model.μₑ),
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
```math
\\mathcal{T}_{ijkL}(\\mathbf{F}) = \\sum_{n=1}^N \\frac{n\\mu_n}{J^{5/3}}\\left[\\mathrm{tr}(\\mathbf{B}^* ) - 3\\right]^{n-1}\\left(\\delta_{ik}F_{jL} + \\delta_{jk}F_{iL} - \\frac{2}{3}\\,\\delta_{ij}F_{kL}- \\frac{5}{3} \\, B_{ij}'F_{kL}^{-T} \\right) + \\sum_{n=2}^N \\frac{2n(n-1)\\mu_n}{J^{7/3}}\\left[\\mathrm{tr}(\\mathbf{B}^* ) - 3\\right]^{n-2}B_{ij}'B_{km}'F_{mL}^{-T} + \\frac{\\kappa}{2} \\left(J + \\frac{1}{J}\\right)\\delta_{ij}F_{kL}^{-T}
```
"""
function cauchy_tangent_stiffness(model::Yeoh, F)
    raw = ccall(
        (:yeoh_cauchy_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}, UInt8, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₑ,
        length(model.μₑ),
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_stress(model::Yeoh, F)
    raw = ccall(
        (:yeoh_first_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}, UInt8, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₑ,
        length(model.μₑ),
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_tangent_stiffness(model::Yeoh, F)
    raw = ccall(
        (:yeoh_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}, UInt8, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₑ,
        length(model.μₑ),
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_stress(model::Yeoh, F)
    raw = ccall(
        (:yeoh_second_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}, UInt8, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₑ,
        length(model.μₑ),
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_tangent_stiffness(model::Yeoh, F)
    raw = ccall(
        (:yeoh_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}, UInt8, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₑ,
        length(model.μₑ),
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
```math
a(\\mathbf{F}) = \\sum_{n=1}^N \\frac{\\mu_n}{2}\\left[\\mathrm{tr}(\\mathbf{B}^* ) - 3\\right]^n + \\frac{\\kappa}{2}\\left[\\frac{1}{2}\\left(J^2 - 1\\right) - \\ln J\\right]
```
"""
function helmholtz_free_energy_density(model::Yeoh, F)
    return ccall(
        (:yeoh_helmholtz_free_energy_density, CONSPIRE_WRAPPER_LIB),
        Float64,
        (Float64, Float64, Ptr{Float64}, UInt8, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₑ,
        length(model.μₑ),
        F,
    )
end
