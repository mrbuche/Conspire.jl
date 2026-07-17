using DocStringExtensions

using ....Conspire: PROJECT_ROOT

const HENCKY_DOC = replace(
    read(
        joinpath(PROJECT_ROOT, "src/constitutive/solid/hyperelastic/hencky/doc.md"),
        String,
    ),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
)
const HENCKY_CAUCHY_STRESS = read(
    joinpath(PROJECT_ROOT, "src/constitutive/solid/hyperelastic/hencky/cauchy_stress.md"),
    String,
)
const HENCKY_CAUCHY_TANGENT_STIFFNESS = read(
    joinpath(
        PROJECT_ROOT,
        "src/constitutive/solid/hyperelastic/hencky/cauchy_tangent_stiffness.md",
    ),
    String,
)
const HENCKY_HELMHOLTZ_FREE_ENERGY_DENSITY = read(
    joinpath(
        PROJECT_ROOT,
        "src/constitutive/solid/hyperelastic/hencky/helmholtz_free_energy_density.md",
    ),
    String,
)

"""
$(HENCKY_DOC)
"""
struct Hencky
    κ::Real
    μ::Real
end

"""
$(TYPEDSIGNATURES)
$(HENCKY_CAUCHY_STRESS)
"""
function cauchy_stress(model::Hencky, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:hencky_cauchy_stress, CONSPIRE_LIB),
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
$(HENCKY_CAUCHY_TANGENT_STIFFNESS)
"""
function cauchy_tangent_stiffness(model::Hencky, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:hencky_cauchy_tangent_stiffness, CONSPIRE_LIB),
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
function first_piola_kirchhoff_stress(model::Hencky, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:hencky_first_piola_kirchhoff_stress, CONSPIRE_LIB),
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
function first_piola_kirchhoff_tangent_stiffness(model::Hencky, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:hencky_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
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
function second_piola_kirchhoff_stress(model::Hencky, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:hencky_second_piola_kirchhoff_stress, CONSPIRE_LIB),
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
function second_piola_kirchhoff_tangent_stiffness(model::Hencky, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:hencky_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
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
$(HENCKY_HELMHOLTZ_FREE_ENERGY_DENSITY)
"""
function helmholtz_free_energy_density(model::Hencky, F)
    return ccall(
        (:hencky_helmholtz_free_energy_density, CONSPIRE_LIB),
        Float64,
        (Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
    )
end
