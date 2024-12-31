using DocStringExtensions

"""
**Parameters**
$(FIELDS)
"""
struct Yeoh
    κ::Real
    μ::Real
    μₑ::Vector
end

"""
$(TYPEDSIGNATURES)
"""
function cauchy_stress(model::Yeoh, F)
    raw = ccall(
        (:yeoh_cauchy_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}, UInt8, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₑ,
        length(model.μₑ),
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function cauchy_tangent_stiffness(model::Yeoh, F)
    raw = ccall(
        (:yeoh_cauchy_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}, UInt8, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₑ,
        length(model.μₑ),
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchoff_stress(model::Yeoh, F)
    raw = ccall(
        (:yeoh_first_piola_kirchoff_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}, UInt8, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₑ,
        length(model.μₑ),
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function first_piola_kirchoff_tangent_stiffness(model::Yeoh, F)
    raw = ccall(
        (:yeoh_first_piola_kirchoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}, UInt8, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₑ,
        length(model.μₑ),
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchoff_stress(model::Yeoh, F)
    raw = ccall(
        (:yeoh_second_piola_kirchoff_stress, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}, UInt8, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₑ,
        length(model.μₑ),
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 9, own = false), (3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function second_piola_kirchoff_tangent_stiffness(model::Yeoh, F)
    raw = ccall(
        (:yeoh_second_piola_kirchoff_tangent_stiffness, CONSPIRE_WRAPPER_LIB),
        Ptr{Float64},
        (Float64, Float64, Ptr{Float64}, UInt8, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₑ,
        length(model.μₑ),
        F,
    )
    return reshape(unsafe_wrap(Array{Float64}, raw, 81, own = false), (3, 3, 3, 3))
end

"""
$(TYPEDSIGNATURES)
"""
function helmholtz_free_energy_density(model::Yeoh, F)
    return ccall(
        (:yeoh_helmholtz_free_energy_density, CONSPIRE_WRAPPER_LIB),
        Float64,
        (Float64, Float64, Ptr{Float64}, UInt8, Ptr{Float64}),
        model.κ,
        model.μ,
        model.μₑ,
        length(model.μₑ),
        F,
    )
end
