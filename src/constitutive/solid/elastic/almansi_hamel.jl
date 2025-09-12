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
const ALMANSI_HAMEL_CAUCHY_STRESS =
    read("src/constitutive/solid/elastic/almansi_hamel/cauchy_stress.md", String)
const ALMANSI_HAMEL_CAUCHY_TANGENT_STIFFNESS =
    read("src/constitutive/solid/elastic/almansi_hamel/cauchy_tangent_stiffness.md", String)

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
    raw = ccall(
        (:almansi_hamel_cauchy_stress, CONSPIRE_LIB),
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
$(ALMANSI_HAMEL_CAUCHY_TANGENT_STIFFNESS)
"""
function cauchy_tangent_stiffness(model::AlmansiHamel, F)
    raw = ccall(
        (:almansi_hamel_cauchy_tangent_stiffness, CONSPIRE_LIB),
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
function first_piola_kirchhoff_stress(model::AlmansiHamel, F)
    raw = ccall(
        (:almansi_hamel_first_piola_kirchhoff_stress, CONSPIRE_LIB),
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
function first_piola_kirchhoff_tangent_stiffness(model::AlmansiHamel, F)
    raw = ccall(
        (:almansi_hamel_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
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
function second_piola_kirchhoff_stress(model::AlmansiHamel, F)
    raw = ccall(
        (:almansi_hamel_second_piola_kirchhoff_stress, CONSPIRE_LIB),
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
function second_piola_kirchhoff_tangent_stiffness(model::AlmansiHamel, F)
    raw = ccall(
        (:almansi_hamel_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end
