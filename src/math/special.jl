using DocStringExtensions

"""
$(TYPEDSIGNATURES)
```math
y = W_0(x)
```
"""
function lambert_w(x::Float64)
    return ccall((:lambert_w, CONSPIRE_WRAPPER_LIB), Float64, (Float64,), x)
end

"""
$(TYPEDSIGNATURES)
```math
\\mathcal{L}(x) = \\coth(x) - x^{-1}
```
"""
function langevin(x::Float64)
    return ccall((:langevin, CONSPIRE_WRAPPER_LIB), Float64, (Float64,), x)
end

"""
$(TYPEDSIGNATURES)
```math
x = \\mathcal{L}^{-1}(y)
```
"""
function inverse_langevin(y::Float64)
    return ccall((:inverse_langevin, CONSPIRE_WRAPPER_LIB), Float64, (Float64,), y)
end

"""
$(TYPEDSIGNATURES)
```math
f(\\mathbf{x}) = \\sum_{i=1}^{N-1} \\left[\\left(a - x_i\\right)^2 + b\\left(x_{i+1} - x_i^2\\right)^2\\right]
```
"""
function rosenbrock(x, a::Float64, b::Float64)
    return ccall(
        (:rosenbrock, CONSPIRE_WRAPPER_LIB),
        Float64,
        (Ptr{Float64}, UInt, Float64, Float64),
        x,
        length(x),
        a,
        b,
    )
end
