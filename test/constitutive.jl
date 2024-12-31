κ = 13.0
μ = 3.0
μₑ = [-1.0, 3e-1, -1e-3, 1e-5]
μₘ = 1.0
Jₘ = 23.0
η = 1.0
N = 8.0

I = [
    1.0 0.0 0.0
    0.0 1.0 0.0
    0.0 0.0 1.0
]
Zero = [
    0.0 0.0 0.0
    0.0 0.0 0.0
    0.0 0.0 0.0
]
SimpleShearSmall = [
    1.0 ϵ 0.0
    0.0 1.0 0.0
    0.0 0.0 1.0
]
VolumetricSmall = I * (1 + ϵ)^(1 / 3)
F = [
    0.63595746 0.69157849 0.71520784
    0.80589604 0.83687323 0.19312595
    0.05387420 0.86551549 0.41880244
]

include("constitutive/elastic.jl")
include("constitutive/hyperelastic.jl")
