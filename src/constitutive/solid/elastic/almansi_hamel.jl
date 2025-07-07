using DocStringExtensions

const ALMANSIHAMELMODEL = replace(
    read("conspire.rs/src/constitutive/solid/elastic/almansi_hamel/model.md", String),
    "\$\`" => "\`\`",
    "\`\$" => "\`\`",
)
const ALMANSIHAMELCAUCHYSTRESS = read(
    "conspire.rs/src/constitutive/solid/elastic/almansi_hamel/cauchy_stress.md",
    String,
)
const ALMANSIHAMELCAUCHYTANGENTSTIFFNESS = read(
    "conspire.rs/src/constitutive/solid/elastic/almansi_hamel/cauchy_tangent_stiffness.md",
    String,
)

"""
$(ALMANSIHAMELMODEL)
"""
struct AlmansiHamel
    κ::Real
    μ::Real
end

"""
$(TYPEDSIGNATURES)
$(ALMANSIHAMELCAUCHYSTRESS)
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
$(ALMANSIHAMELCAUCHYTANGENTSTIFFNESS)
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
function first_piola_kirchhoff_stress(model::AlmansiHamel, F)
    raw = ccall(
        (:almansi_hamel_first_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
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
        (:almansi_hamel_first_piola_kirchhoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
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
        (:almansi_hamel_second_piola_kirchhoff_stress, CONSPIRE_WRAPPER_LIB),
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
        (:almansi_hamel_second_piola_kirchhoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}),
        model.κ,
        model.μ,
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end
