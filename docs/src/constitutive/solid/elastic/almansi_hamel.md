# Almansi-Hamel

```@eval
using Markdown
using ....Conspire: PROJECT_ROOT

foo = Markdown.parse(
    replace(
        read(
            joinpath(
                PROJECT_ROOT,
                "conspire.rs/src/constitutive/solid/elastic/almansi_hamel/model.md",
            ), String
        ),
    "\$\`" => "\`\`", "\`\$" => "\`\`")
)
```

```@docs
AlmansiHamel
```

## Methods

```@docs
cauchy_stress(model::AlmansiHamel, F)
```

```@docs
cauchy_tangent_stiffness(model::AlmansiHamel, F)
```

```@docs
first_piola_kirchhoff_stress(model::AlmansiHamel, F)
```

```@docs
first_piola_kirchhoff_tangent_stiffness(model::AlmansiHamel, F)
```

```@docs
second_piola_kirchhoff_stress(model::AlmansiHamel, F)
```

```@docs
second_piola_kirchhoff_tangent_stiffness(model::AlmansiHamel, F)
```
