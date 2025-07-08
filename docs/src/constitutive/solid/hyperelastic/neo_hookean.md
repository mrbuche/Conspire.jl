# Neo-Hookean

```@eval
using Markdown
using ....Conspire: PROJECT_ROOT

Markdown.parse(
    replace(
        read(
            joinpath(
                PROJECT_ROOT,
                "conspire.rs/src/constitutive/solid/hyperelastic/neo_hookean/model.md",
            ), String
        ), "\$\`" => "\`\`", "\`\$" => "\`\`"
    ) * """\n\n**Helmholtz free energy density**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/neo_hookean/helmholtz_free_energy_density.md",
        ), String
    ) * """\n\n**Cauchy stress**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/neo_hookean/cauchy_stress.md",
        ), String
    ) * """\n\n**Cauchy tangent stiffness**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/neo_hookean/cauchy_tangent_stiffness.md",
        ), String
    )
)
```

```@docs
NeoHookean
```

## Methods

```@docs
cauchy_stress(model::NeoHookean, F)
```

```@docs
cauchy_tangent_stiffness(model::NeoHookean, F)
```

```@docs
first_piola_kirchhoff_stress(model::NeoHookean, F)
```

```@docs
first_piola_kirchhoff_tangent_stiffness(model::NeoHookean, F)
```

```@docs
second_piola_kirchhoff_stress(model::NeoHookean, F)
```

```@docs
second_piola_kirchhoff_tangent_stiffness(model::NeoHookean, F)
```

```@docs
helmholtz_free_energy_density(model::NeoHookean, F)
```
