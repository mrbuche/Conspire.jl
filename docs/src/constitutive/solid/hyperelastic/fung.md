# Fung

```@eval
using Markdown
using ....Conspire: PROJECT_ROOT

Markdown.parse(
    replace(
        read(
            joinpath(
                PROJECT_ROOT,
                "conspire.rs/src/constitutive/solid/hyperelastic/fung/model.md",
            ), String
        ), "\$\`" => "\`\`", "\`\$" => "\`\`"
    ) * """\n\n**Helmholtz free energy density**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/fung/helmholtz_free_energy_density.md",
        ), String
    ) * """\n\n**Cauchy stress**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/fung/cauchy_stress.md",
        ), String
    ) * """\n\n**Cauchy tangent stiffness**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/fung/cauchy_tangent_stiffness.md",
        ), String
    )
)
```

```@docs
Fung
```

## Methods

```@docs
cauchy_stress(model::Fung, F)
```

```@docs
cauchy_tangent_stiffness(model::Fung, F)
```

```@docs
first_piola_kirchhoff_stress(model::Fung, F)
```

```@docs
first_piola_kirchhoff_tangent_stiffness(model::Fung, F)
```

```@docs
second_piola_kirchhoff_stress(model::Fung, F)
```

```@docs
second_piola_kirchhoff_tangent_stiffness(model::Fung, F)
```

```@docs
helmholtz_free_energy_density(model::Fung, F)
```
