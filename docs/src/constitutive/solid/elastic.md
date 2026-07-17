# Elastic

* [Almansi-Hamel](elastic/almansi_hamel.md) -- The Almansi-Hamel elastic constitutive model.
* [Hencky (elastic)](elastic/hencky_elastic.md) -- The Hencky elastic constitutive model.
* [Saint Venant-Kirchhoff (elastic)](elastic/saint_venant_kirchhoff_elastic.md) -- The Saint Venant-Kirchhoff elastic constitutive model.

## Functions

```@docs
cauchy_stress(::AlmansiHamel, ::Matrix)
```

```@docs
cauchy_tangent_stiffness(::AlmansiHamel, ::Matrix)
```

```@docs
first_piola_kirchhoff_stress(::AlmansiHamel, ::Matrix)
```

```@docs
first_piola_kirchhoff_tangent_stiffness(::AlmansiHamel, ::Matrix)
```

```@docs
second_piola_kirchhoff_stress(::AlmansiHamel, ::Matrix)
```

```@docs
second_piola_kirchhoff_tangent_stiffness(::AlmansiHamel, ::Matrix)
```
