# Mooney-Rivlin

```@eval
using Markdown
using ....Conspire: PROJECT_ROOT

Markdown.parse(
    replace(
        read(
            joinpath(
                PROJECT_ROOT,
                "conspire.rs/src/constitutive/solid/hyperelastic/mooney_rivlin/model.md",
            ), String
        ), "\$\`" => "\`\`", "\`\$" => "\`\`"
    ) * """\n\n**Helmholtz free energy density**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/mooney_rivlin/helmholtz_free_energy_density.md",
        ), String
    ) * """\n\n**Cauchy stress**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/mooney_rivlin/cauchy_stress.md",
        ), String
    ) * """\n\n**Cauchy tangent stiffness**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/mooney_rivlin/cauchy_tangent_stiffness.md",
        ), String
    )
)
```

```@docs
MooneyRivlin
```

## Methods

```@docs
cauchy_stress(model::MooneyRivlin, F)
```

```@docs
cauchy_tangent_stiffness(model::MooneyRivlin, F)
```

```@docs
first_piola_kirchhoff_stress(model::MooneyRivlin, F)
```

```@docs
first_piola_kirchhoff_tangent_stiffness(model::MooneyRivlin, F)
```

```@docs
second_piola_kirchhoff_stress(model::MooneyRivlin, F)
```

```@docs
second_piola_kirchhoff_tangent_stiffness(model::MooneyRivlin, F)
```

```@docs
helmholtz_free_energy_density(model::MooneyRivlin, F)
```
