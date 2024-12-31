# c o n s p i r e

[![stable](https://img.shields.io/badge/docs-stable-blue)](https://mrbuche.github.io/Conspire.jl/stable)
[![latest](https://img.shields.io/badge/docs-latest-blue)](https://mrbuche.github.io/Conspire.jl/latest)
[![license](https://img.shields.io/github/license/mrbuche/Conspire.jl?color=blue)](https://github.com/mrbuche/Conspire.jl?tab=GPL-3.0-1-ov-file#GPL-3.0-1-ov-file)
[![release](https://img.shields.io/github/v/release/mrbuche/Conspire.jl?color=blue)](https://github.com/mrbuche/Conspire.jl)

The Julia interface to [conspire](https://mrbuche.github.io/conspire).

## Installation

Conspire.jl is a Julia wrapper of [conspire](https://github.com/mrbuche/conspire.rs) offering constitutive modeling features.

```julia
pkg> add Conspire
```

## Example

Calculate the Cauchy stress for a Neo-Hookean model under an applied deformation gradient:

```julia
julia> using Conspire, StaticArrays
julia> model = NeoHookean(13.0, 3.0);
julia> F = SMatrix{3,3,Float64}(
           0.63595746, 0.69157849, 0.71520784,
           0.80589604, 0.83687323, 0.19312595,
           0.05387420, 0.86551549, 0.41880244
       );
julia> cauchy_stress(model, F)
3×3 SMatrix{3, 3, Float64, 9} with indices SOneTo(3)×SOneTo(3):
 -13.249    20.5297   15.5694
  20.5297  -13.3679   14.1711
  15.5694   14.1711  -21.044
```
