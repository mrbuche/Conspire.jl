using DocStringExtensions

"""
The Saint Venant-Kirchoff hyperelastic constitutive model.

**Parameters**
- The bulk modulus ``\\kappa``.
- The shear modulus ``\\mu``.

**External variables**
- The deformation gradient ``\\mathbf{F}``.

**Internal variables**
- None.

**Notes**
- The Green-Saint Venant strain measure is given by ``\\mathbf{E}=\\tfrac{1}{2}(\\mathbf{C}-\\mathbf{1})``.
"""
struct SaintVenantKirchoff
    κ::Real
    μ::Real
end

"""
$(TYPEDSIGNATURES)
"""
function cauchy_stress(model::SaintVenantKirchoff, F)
    raw = ccall(
        (:saint_venant_kirchoff_cauchy_stress, CONSPIRE_WRAPPER_LIB),
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
function cauchy_tangent_stiffness(model::SaintVenantKirchoff, F)
    raw = ccall(
        (:saint_venant_kirchoff_cauchy_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
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
function first_piola_kirchoff_stress(model::SaintVenantKirchoff, F)
    raw = ccall(
        (:saint_venant_kirchoff_first_piola_kirchoff_stress, CONSPIRE_WRAPPER_LIB),
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
function first_piola_kirchoff_tangent_stiffness(model::SaintVenantKirchoff, F)
    raw = ccall(
        (
            :saint_venant_kirchoff_first_piola_kirchoff_tangent_stiffness,
            CONSPIRE_WRAPPER_LIB,
        ),
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
\\mathbf{S}(\\mathbf{F}) = 2\\mu\\mathbf{E}' + \\kappa\\,\\mathrm{tr}(\\mathbf{E})\\mathbf{1}
```
"""
function second_piola_kirchoff_stress(model::SaintVenantKirchoff, F)
    raw = ccall(
        (:saint_venant_kirchoff_second_piola_kirchoff_stress, CONSPIRE_WRAPPER_LIB),
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
\\mathcal{G}_{IJkL}(\\mathbf{F}) = \\mu\\,\\delta_{JL}F_{kI} + \\mu\\,\\delta_{IL}F_{kJ} + \\left(\\kappa - \\frac{2}{3}\\,\\mu\\right)\\delta_{IJ}F_{kL}
```
"""
function second_piola_kirchoff_tangent_stiffness(model::SaintVenantKirchoff, F)
    raw = ccall(
        (
            :saint_venant_kirchoff_second_piola_kirchoff_tangent_stiffness,
            CONSPIRE_WRAPPER_LIB,
        ),
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
a(\\mathbf{F}) = \\mu\\,\\mathrm{tr}(\\mathbf{E}^2) + \\frac{1}{2}\\left(\\kappa - \\frac{2}{3}\\,\\mu\\right)\\mathrm{tr}(\\mathbf{E})^2
```
"""
function helmholtz_free_energy_density(model::SaintVenantKirchoff, F)
    return ccall(
        (:saint_venant_kirchoff_helmholtz_free_energy_density, CONSPIRE_WRAPPER_LIB),
        Float64,
        (Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
    )
end
