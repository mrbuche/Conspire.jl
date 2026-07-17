using DocStringExtensions

using ....Conspire: PROJECT_ROOT

const ALMANSI_HAMEL_DOC = replace(
    read(
        joinpath(PROJECT_ROOT, "src/constitutive/solid/elastic/almansi_hamel/doc.md"),
        String,
    ),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
)
const ALMANSI_HAMEL_CAUCHY_STRESS = read(
    joinpath(PROJECT_ROOT, "src/constitutive/solid/elastic/almansi_hamel/cauchy_stress.md"),
    String,
)
const ALMANSI_HAMEL_CAUCHY_TANGENT_STIFFNESS = read(
    joinpath(
        PROJECT_ROOT,
        "src/constitutive/solid/elastic/almansi_hamel/cauchy_tangent_stiffness.md",
    ),
    String,
)

"""
$(ALMANSI_HAMEL_DOC)
"""
struct AlmansiHamel
    κ::Real
    μ::Real
end

"""
$(TYPEDSIGNATURES)
$(ALMANSI_HAMEL_CAUCHY_STRESS)
"""
function cauchy_stress(model::AlmansiHamel, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:almansi_hamel_cauchy_stress, CONSPIRE_LIB),
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
$(ALMANSI_HAMEL_CAUCHY_TANGENT_STIFFNESS)
"""
function cauchy_tangent_stiffness(model::AlmansiHamel, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:almansi_hamel_cauchy_tangent_stiffness, CONSPIRE_LIB),
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
function first_piola_kirchhoff_stress(model::AlmansiHamel, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:almansi_hamel_first_piola_kirchhoff_stress, CONSPIRE_LIB),
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
function first_piola_kirchhoff_tangent_stiffness(model::AlmansiHamel, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:almansi_hamel_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
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
function second_piola_kirchhoff_stress(model::AlmansiHamel, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:almansi_hamel_second_piola_kirchhoff_stress, CONSPIRE_LIB),
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
function second_piola_kirchhoff_tangent_stiffness(model::AlmansiHamel, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:almansi_hamel_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
        output,
    )
    return output
end
