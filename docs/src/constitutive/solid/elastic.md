# Elastic

Elastic constitutive models cannot be defined by a Helmholtz free energy density but still depend on only the deformation gradient.
These constitutive models are therefore defined by a relation for some stress measure as a function of the deformation gradient.
Consequently, the tangent stiffness associated with the first Piola-Kirchhoff stress is not symmetric for elastic constitutive models.

```math
\mathcal{C}_{iJkL} \neq \mathcal{C}_{kLiJ}
```

* [Almansi-Hamel](elastic/almansi_hamel.md) -- The Almansi-Hamel elastic constitutive model.

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