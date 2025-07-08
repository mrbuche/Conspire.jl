# Arruda-Boyce

```@eval
using Markdown
using ....Conspire: PROJECT_ROOT

Markdown.parse(
    replace(
        read(
            joinpath(
                PROJECT_ROOT,
                "conspire.rs/src/constitutive/solid/hyperelastic/arruda_boyce/model.md",
            ), String
        ), "\$\`" => "\`\`", "\`\$" => "\`\`"
    ) * """\n\n**Helmholtz free energy density**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/arruda_boyce/helmholtz_free_energy_density.md",
        ), String
    ) * """\n\n**Cauchy stress**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/arruda_boyce/cauchy_stress.md",
        ), String
    ) * """\n\n**Cauchy tangent stiffness**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/arruda_boyce/cauchy_tangent_stiffness.md",
        ), String
    )
)
```

```@docs
ArrudaBoyce
```

## Methods

```@docs
cauchy_stress(model::ArrudaBoyce, F)
```

```@docs
cauchy_tangent_stiffness(model::ArrudaBoyce, F)
```

```@docs
first_piola_kirchhoff_stress(model::ArrudaBoyce, F)
```

```@docs
first_piola_kirchhoff_tangent_stiffness(model::ArrudaBoyce, F)
```

```@docs
second_piola_kirchhoff_stress(model::ArrudaBoyce, F)
```

```@docs
second_piola_kirchhoff_tangent_stiffness(model::ArrudaBoyce, F)
```

```@docs
helmholtz_free_energy_density(model::ArrudaBoyce, F)
```
