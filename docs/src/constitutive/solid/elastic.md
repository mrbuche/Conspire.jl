# Elastic

```@eval
using Markdown
Markdown.parse_file(joinpath("..", "..", "..", "..", "conspire.rs", "src", "constitutive", "solid", "elastic", "doc.md"))
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
