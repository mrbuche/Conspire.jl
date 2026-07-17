using DocStringExtensions

const SAINT_VENANT_KIRCHHOFF_ELASTIC_DOC = """
The Saint Venant-Kirchhoff elastic solid constitutive model.

**Parameters**
- The bulk modulus \$\\kappa\$.
- The shear modulus \$\\mu\$.

**External variables**
- The deformation gradient \$\\mathbf{F}\$.

**Internal variables**
- None.

**Notes**
- This is the spatial (Eulerian) formulation, built from the left Cauchy-Green deformation \$\\mathbf{B}\$, distinct from the [Saint Venant-Kirchhoff](@ref "Saint Venant-Kirchhoff") hyperelastic model's material (Lagrangian) formulation built from \$\\mathbf{C}\$.
"""

"""
$(SAINT_VENANT_KIRCHHOFF_ELASTIC_DOC)
"""
struct SaintVenantKirchhoffElastic
    κ::Real
    μ::Real
end

"""
$(TYPEDSIGNATURES)
"""
function cauchy_stress(model::SaintVenantKirchhoffElastic, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:saint_venant_kirchhoff_elastic_cauchy_stress, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function cauchy_tangent_stiffness(model::SaintVenantKirchhoffElastic, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:saint_venant_kirchhoff_elastic_cauchy_tangent_stiffness, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_stress(model::SaintVenantKirchhoffElastic, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:saint_venant_kirchhoff_elastic_first_piola_kirchhoff_stress, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchhoff_tangent_stiffness(model::SaintVenantKirchhoffElastic, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (
            :saint_venant_kirchhoff_elastic_first_piola_kirchhoff_tangent_stiffness,
            CONSPIRE_LIB,
        ),
        Cvoid,
        (Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_stress(model::SaintVenantKirchhoffElastic, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:saint_venant_kirchhoff_elastic_second_piola_kirchhoff_stress, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
        output,
    )
    return output
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchhoff_tangent_stiffness(model::SaintVenantKirchhoffElastic, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (
            :saint_venant_kirchhoff_elastic_second_piola_kirchhoff_tangent_stiffness,
            CONSPIRE_LIB,
        ),
        Cvoid,
        (Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
        output,
    )
    return output
end
