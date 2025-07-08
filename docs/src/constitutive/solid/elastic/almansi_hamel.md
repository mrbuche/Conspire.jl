# Almansi-Hamel

```@eval
using Markdown
using ....Conspire: PROJECT_ROOT

Markdown.parse(
    replace(
        read(
            joinpath(
                PROJECT_ROOT,
                "conspire.rs/src/constitutive/solid/elastic/almansi_hamel/model.md",
            ), String
        ), "\$\`" => "\`\`", "\`\$" => "\`\`"
    ) * """\n\n**Cauchy stress**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/elastic/almansi_hamel/cauchy_stress.md",
        ), String
    ) * """\n\n**Cauchy tangent stiffness**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/elastic/almansi_hamel/cauchy_tangent_stiffness.md",
        ), String
    )
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
