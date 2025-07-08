# Saint Venant-Kirchhoff

```@eval
using Markdown
using ....Conspire: PROJECT_ROOT

Markdown.parse(
    replace(
        read(
            joinpath(
                PROJECT_ROOT,
                "conspire.rs/src/constitutive/solid/hyperelastic/saint_venant_kirchhoff/model.md",
            ), String
        ), "\$\`" => "\`\`", "\`\$" => "\`\`"
    ) * """\n\n**Helmholtz free energy density**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/saint_venant_kirchhoff/helmholtz_free_energy_density.md",
        ), String
    ) * """\n\n**Second Piola-Kirchhoff stress**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/saint_venant_kirchhoff/second_piola_kirchhoff_stress.md",
        ), String
    ) * """\n\n**Second Piola-Kirchhoff tangent stiffness**\n\n""" *
    read(
        joinpath(
            PROJECT_ROOT,
            "conspire.rs/src/constitutive/solid/hyperelastic/saint_venant_kirchhoff/second_piola_kirchhoff_tangent_stiffness.md",
        ), String
    )
)
```

```@docs
SaintVenantKirchhoff
```

## Methods

```@docs
cauchy_stress(model::SaintVenantKirchhoff, F)
```

```@docs
cauchy_tangent_stiffness(model::SaintVenantKirchhoff, F)
```

```@docs
first_piola_kirchhoff_stress(model::SaintVenantKirchhoff, F)
```

```@docs
first_piola_kirchhoff_tangent_stiffness(model::SaintVenantKirchhoff, F)
```

```@docs
second_piola_kirchhoff_stress(model::SaintVenantKirchhoff, F)
```

```@docs
second_piola_kirchhoff_tangent_stiffness(model::SaintVenantKirchhoff, F)
```

```@docs
helmholtz_free_energy_density(model::SaintVenantKirchhoff, F)
```
