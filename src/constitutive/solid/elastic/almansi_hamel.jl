using DocStringExtensions

"""
The Almansi-Hamel elastic constitutive model.

**Parameters**
- The bulk modulus ``\\kappa``.
- The shear modulus ``\\mu``.

**External variables**
- The deformation gradient ``\\mathbf{F}``.

**Internal variables**
- None.

**Notes**
- The Almansi-Hamel strain measure is given by ``\\mathbf{e}=\\tfrac{1}{2}(\\mathbf{1}-\\mathbf{B}^{-1})``.
"""
struct AlmansiHamel
    κ::Real
    μ::Real
end

"""
$(TYPEDSIGNATURES)
```math
\\boldsymbol{\\sigma}(\\mathbf{F}) = \\frac{2\\mu}{J}\\,\\mathbf{e}' + \\frac{\\kappa}{J}\\,\\mathrm{tr}(\\mathbf{e})\\mathbf{1}
```
"""
function cauchy_stress(model::AlmansiHamel, F)
    raw = ccall(
        (:almansi_hamel_cauchy_stress, CONSPIRE_WRAPPER_LIB),
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
\\mathcal{T}_{ijkL}(\\mathbf{F}) = \\frac{\\mu}{J}\\left[B_{jk}^{-1}F_{iL}^{-T} + B_{ik}^{-1}F_{jL}^{-T} - \\frac{2}{3}\\,\\delta_{ij}B_{km}^{-1}F_{mL}^{-T} - 2e_{ij}'F_{kL}^{-T}\\right] + \\frac{\\kappa}{J}\\left[\\delta_{ij}B_{km}^{-1}F_{mL}^{-T} - \\mathrm{tr}(\\mathbf{e})\\delta_{ij}F_{kL}^{-T}\\right]
```
"""
function cauchy_tangent_stiffness(model::AlmansiHamel, F)
    raw = ccall(
        (:almansi_hamel_cauchy_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
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
function first_piola_kirchoff_stress(model::AlmansiHamel, F)
    raw = ccall(
        (:almansi_hamel_first_piola_kirchoff_stress, CONSPIRE_WRAPPER_LIB),
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
function first_piola_kirchoff_tangent_stiffness(model::AlmansiHamel, F)
    raw = ccall(
        (:almansi_hamel_first_piola_kirchoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
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
function second_piola_kirchoff_stress(model::AlmansiHamel, F)
    raw = ccall(
        (:almansi_hamel_second_piola_kirchoff_stress, CONSPIRE_WRAPPER_LIB),
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
function second_piola_kirchoff_tangent_stiffness(model::AlmansiHamel, F)
    raw = ccall(
        (:almansi_hamel_second_piola_kirchoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end
