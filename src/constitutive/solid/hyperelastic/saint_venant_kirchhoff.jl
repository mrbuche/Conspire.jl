using DocStringExtensions

using ....Conspire: PROJECT_ROOT

const SAINTVENANTKIRCHHOFFMODEL = replace(
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/saint_venant_kirchhoff/model.md",
        ),
        String,
    ),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
)
const SAINTVENANTKIRCHHOFFSECONDPIOLAKIRCHHOFFSTRESS = read(
    joinpath(
        PROJECT_ROOT,
        "conspire.rs/src/constitutive/solid/hyperelastic/saint_venant_kirchhoff/second_piola_kirchhoff_stress.md",
    ),
    String,
)
const SAINTVENANTKIRCHHOFFSECONDPIOLAKIRCHHOFFTANGENTSTIFFNESS = read(
    joinpath(
        PROJECT_ROOT,
        "conspire.rs/src/constitutive/solid/hyperelastic/saint_venant_kirchhoff/second_piola_kirchhoff_tangent_stiffness.md",
    ),
    String,
)
const SAINTVENANTKIRCHHOFFHELMHOLTZFREEENERGYDENSITY = read(
    joinpath(
        PROJECT_ROOT,
        "conspire.rs/src/constitutive/solid/hyperelastic/saint_venant_kirchhoff/helmholtz_free_energy_density.md",
    ),
    String,
)

"""
$(SAINTVENANTKIRCHHOFFMODEL)
"""
struct SaintVenantKirchhoff
    κ::Real
    μ::Real
end

"""
$(TYPEDSIGNATURES)
"""
function cauchy_stress(model::SaintVenantKirchhoff, F)
    raw = ccall(
        (:saint_venant_kirchhoff_cauchy_stress, CONSPIRE_WRAPPER_LIB),
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
function cauchy_tangent_stiffness(model::SaintVenantKirchhoff, F)
    raw = ccall(
        (:saint_venant_kirchhoff_cauchy_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
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
function first_piola_kirchhoff_stress(model::SaintVenantKirchhoff, F)
    raw = ccall(
        (:saint_venant_kirchhoff_first_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
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
function first_piola_kirchhoff_tangent_stiffness(model::SaintVenantKirchhoff, F)
    raw = ccall(
        (
            :saint_venant_kirchhoff_first_piola_kirchhoff_tangent_stiffness,
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
$(SAINTVENANTKIRCHHOFFSECONDPIOLAKIRCHHOFFSTRESS)
"""
function second_piola_kirchhoff_stress(model::SaintVenantKirchhoff, F)
    raw = ccall(
        (:saint_venant_kirchhoff_second_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
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
$(SAINTVENANTKIRCHHOFFSECONDPIOLAKIRCHHOFFTANGENTSTIFFNESS)
"""
function second_piola_kirchhoff_tangent_stiffness(model::SaintVenantKirchhoff, F)
    raw = ccall(
        (
            :saint_venant_kirchhoff_second_piola_kirchhoff_tangent_stiffness,
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
$(SAINTVENANTKIRCHHOFFHELMHOLTZFREEENERGYDENSITY)
"""
function helmholtz_free_energy_density(model::SaintVenantKirchhoff, F)
    return ccall(
        (:saint_venant_kirchhoff_helmholtz_free_energy_density, CONSPIRE_WRAPPER_LIB),
        Float64,
        (Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
    )
end
