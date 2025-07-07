using Aqua, Conspire, JuliaFormatter, LinearAlgebra, Test

ϵ = 1e-6

include("constitutive.jl")

Aqua.test_all(Conspire)

if !format("../", overwrite = false, verbose = true)
    error("File(s) not properly formatted.")
end
