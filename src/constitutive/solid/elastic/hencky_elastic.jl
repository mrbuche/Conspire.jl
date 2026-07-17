using DocStringExtensions

using ....Conspire: PROJECT_ROOT

const HENCKY_ELASTIC_DOC = replace(
    read(joinpath(PROJECT_ROOT, "src/constitutive/solid/elastic/hencky/doc.md"), String),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
)
const HENCKY_ELASTIC_SECOND_PIOLA_KIRCHHOFF_STRESS = read(
    joinpath(
        PROJECT_ROOT,
        "src/constitutive/solid/elastic/hencky/second_piola_kirchhoff_stress.md",
    ),
    String,
)
const HENCKY_ELASTIC_SECOND_PIOLA_KIRCHHOFF_TANGENT_STIFFNESS = read(
    joinpath(
        PROJECT_ROOT,
        "src/constitutive/solid/elastic/hencky/second_piola_kirchhoff_tangent_stiffness.md",
    ),
    String,
)

"""
$(HENCKY_ELASTIC_DOC)
"""
struct HenckyElastic
    κ::Real
    μ::Real
end

"""
$(TYPEDSIGNATURES)
"""
function cauchy_stress(model::HenckyElastic, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:hencky_elastic_cauchy_stress, CONSPIRE_LIB),
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
function cauchy_tangent_stiffness(model::HenckyElastic, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:hencky_elastic_cauchy_tangent_stiffness, CONSPIRE_LIB),
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
function first_piola_kirchhoff_stress(model::HenckyElastic, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:hencky_elastic_first_piola_kirchhoff_stress, CONSPIRE_LIB),
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
function first_piola_kirchhoff_tangent_stiffness(model::HenckyElastic, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:hencky_elastic_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
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
$(HENCKY_ELASTIC_SECOND_PIOLA_KIRCHHOFF_STRESS)
"""
function second_piola_kirchhoff_stress(model::HenckyElastic, F)
    output = zeros(Float64, 3, 3)
    ccall(
        (:hencky_elastic_second_piola_kirchhoff_stress, CONSPIRE_LIB),
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
$(HENCKY_ELASTIC_SECOND_PIOLA_KIRCHHOFF_TANGENT_STIFFNESS)
"""
function second_piola_kirchhoff_tangent_stiffness(model::HenckyElastic, F)
    output = zeros(Float64, 3, 3, 3, 3)
    ccall(
        (:hencky_elastic_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
        Cvoid,
        (Float64, Float64, Ptr{Float64}, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
        output,
    )
    return output
end
