using DocStringExtensions

using ....Conspire: PROJECT_ROOT

const SAINT_VENANT_KIRCHHOFF_DOC = replace(
    read(
        joinpath(
            PROJECT_ROOT,
            "src/constitutive/solid/hyperelastic/saint_venant_kirchhoff/doc.md",
        ),
        String,
    ),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
)
const SAINT_VENANT_KIRCHHOFF_SECOND_PIOLA_KIRCHHOFF_STRESS = read(
    joinpath(
        PROJECT_ROOT,
        "src/constitutive/solid/hyperelastic/saint_venant_kirchhoff/second_piola_kirchhoff_stress.md",
    ),
    String,
)
const SAINT_VENANT_KIRCHHOFF_SECOND_PIOLA_KIRCHHOFF_TANGENT_STIFFNESS = read(
    joinpath(
        PROJECT_ROOT,
        "src/constitutive/solid/hyperelastic/saint_venant_kirchhoff/second_piola_kirchhoff_tangent_stiffness.md",
    ),
    String,
)
const SAINT_VENANT_KIRCHHOFF_HELMHOLTZ_FREE_ENERGY_DENSITY = read(
    joinpath(
        PROJECT_ROOT,
        "src/constitutive/solid/hyperelastic/saint_venant_kirchhoff/helmholtz_free_energy_density.md",
    ),
    String,
)

"""
$(SAINT_VENANT_KIRCHHOFF_DOC)
"""
struct SaintVenantKirchhoff
    κ::Real
    μ::Real
end

"""
$(TYPEDSIGNATURES)
"""
function cauchy_stress(model::SaintVenantKirchhoff, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:saint_venant_kirchhoff_cauchy_stress, CONSPIRE_LIB),
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
function cauchy_tangent_stiffness(model::SaintVenantKirchhoff, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:saint_venant_kirchhoff_cauchy_tangent_stiffness, CONSPIRE_LIB),
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
function first_piola_kirchhoff_stress(model::SaintVenantKirchhoff, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:saint_venant_kirchhoff_first_piola_kirchhoff_stress, CONSPIRE_LIB),
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
function first_piola_kirchhoff_tangent_stiffness(model::SaintVenantKirchhoff, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:saint_venant_kirchhoff_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
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
$(SAINT_VENANT_KIRCHHOFF_SECOND_PIOLA_KIRCHHOFF_STRESS)
"""
function second_piola_kirchhoff_stress(model::SaintVenantKirchhoff, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:saint_venant_kirchhoff_second_piola_kirchhoff_stress, CONSPIRE_LIB),
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
$(SAINT_VENANT_KIRCHHOFF_SECOND_PIOLA_KIRCHHOFF_TANGENT_STIFFNESS)
"""
function second_piola_kirchhoff_tangent_stiffness(model::SaintVenantKirchhoff, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:saint_venant_kirchhoff_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
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
$(SAINT_VENANT_KIRCHHOFF_HELMHOLTZ_FREE_ENERGY_DENSITY)
"""
function helmholtz_free_energy_density(model::SaintVenantKirchhoff, F)
    return ccall(
        (:saint_venant_kirchhoff_helmholtz_free_energy_density, CONSPIRE_LIB),
        Float64,
        (Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
    )
end
