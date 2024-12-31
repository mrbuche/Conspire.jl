using DocStringExtensions

"""
The Arruda-Boyce hyperelastic constitutive model.[^1]

[^1]: E.M. Arruda and M.C. Boyce, [J. Mech. Phys. Solids **41**, 389 (1993)](https://doi.org/10.1016/0022-5096(93)90013-6).

**Parameters**
- The bulk modulus ``\\kappa``.
- The shear modulus ``\\mu``.
- The number of links ``N_b``.

**External variables**
- The deformation gradient ``\\mathbf{F}``.

**Internal variables**
- None.

**Notes**
- The nondimensional end-to-end length per link of the chains is ``\\gamma=\\sqrt{\\mathrm{tr}(\\mathbf{B}^*)/3N_b}``.
- The nondimensional force is given by the inverse Langevin function as ``\\eta=\\mathcal{L}^{-1}(\\gamma)``.
- The initial values are given by ``\\gamma_0=\\sqrt{1/3N_b}`` and ``\\eta_0=\\mathcal{L}^{-1}(\\gamma_0)``.
- The Arruda-Boyce model reduces to the [`NeoHookean`](@ref) model when ``N_b\\to\\infty``.
"""
struct ArrudaBoyce
    κ::Real
    μ::Real
    N::Real
end

"""
$(TYPEDSIGNATURES)
```math
\\boldsymbol{\\sigma}(\\mathbf{F}) = \\frac{\\mu\\gamma_0\\eta}{J\\gamma\\eta_0}\\,{\\mathbf{B}^*}' + \\frac{\\kappa}{2}\\left(J - \\frac{1}{J}\\right)\\mathbf{1}
```
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
```math
\\mathcal{T}_{ijkL}(\\mathbf{F}) = \\frac{\\mu\\gamma_0\\eta}{J^{5/3}\\gamma\\eta_0}\\left(\\delta_{ik}F_{jL} + \\delta_{jk}F_{iL} - \\frac{2}{3}\\,\\delta_{ij}F_{kL}- \\frac{5}{3} \\, B_{ij}'F_{kL}^{-T} \\right) + \\frac{\\mu\\gamma_0\\eta}{3J^{7/3}N_b\\gamma^2\\eta_0}\\left(\\frac{1}{\\eta\\mathcal{L}'(\\eta)} - \\frac{1}{\\gamma}\\right)B_{ij}'B_{km}'F_{mL}^{-T} + \\frac{\\kappa}{2} \\left(J + \\frac{1}{J}\\right)\\delta_{ij}F_{kL}^{-T}
```
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
function first_piola_kirchoff_stress(model::ArrudaBoyce, F)
    raw = ccall(
        (:arruda_boyce_first_piola_kirchoff_stress, CONSPIRE_WRAPPER_LIB),
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
function first_piola_kirchoff_tangent_stiffness(model::ArrudaBoyce, F)
    raw = ccall(
        (:arruda_boyce_first_piola_kirchoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
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
function second_piola_kirchoff_stress(model::ArrudaBoyce, F)
    raw = ccall(
        (:arruda_boyce_second_piola_kirchoff_stress, CONSPIRE_WRAPPER_LIB),
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
function second_piola_kirchoff_tangent_stiffness(model::ArrudaBoyce, F)
    raw = ccall(
        (:arruda_boyce_second_piola_kirchoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
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
```math
a(\\mathbf{F}) = \\frac{3\\mu N_b\\gamma_0}{\\eta_0}\\left[\\gamma\\eta - \\gamma_0\\eta_0 - \\ln\\left(\\frac{\\eta_0\\sinh\\eta}{\\eta\\sinh\\eta_0}\\right) \\right] + \\frac{\\kappa}{2}\\left[\\frac{1}{2}\\left(J^2 - 1\\right) - \\ln J\\right]
```
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
