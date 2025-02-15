using DocStringExtensions

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
