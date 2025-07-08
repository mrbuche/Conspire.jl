# Gent

```@eval
using Markdown
using ....Conspire: PROJECT_ROOT

Markdown.parse(
    replace(
        read(
            joinpath(
                PROJECT_ROOT,
                "conspire.rs/src/constitutive/solid/hyperelastic/gent/model.md",
            ), String
        ), "\$\`" => "\`\`", "\`\$" => "\`\`"
    ) * """\n\n**Helmholtz free energy density**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/gent/helmholtz_free_energy_density.md",
        ), String
    ) * """\n\n**Cauchy stress**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/gent/cauchy_stress.md",
        ), String
    ) * """\n\n**Cauchy tangent stiffness**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/gent/cauchy_tangent_stiffness.md",
        ), String
    )
)
```

```@docs
Gent
```

## Methods

```@docs
cauchy_stress(model::Gent, F)
```

```@docs
cauchy_tangent_stiffness(model::Gent, F)
```

```@docs
first_piola_kirchhoff_stress(model::Gent, F)
```

```@docs
first_piola_kirchhoff_tangent_stiffness(model::Gent, F)
```

```@docs
second_piola_kirchhoff_stress(model::Gent, F)
```

```@docs
second_piola_kirchhoff_tangent_stiffness(model::Gent, F)
```

```@docs
helmholtz_free_energy_density(model::Gent, F)
```
