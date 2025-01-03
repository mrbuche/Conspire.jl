# Hyperelastic

Hyperelastic constitutive models are completely defined by a Helmholtz free energy density function of the deformation gradient.

```math
\mathbf{P}:\dot{\mathbf{F}} - \dot{a}(\mathbf{F}) \geq 0
```
Satisfying the second law of thermodynamics (here, equivalent to extremized or zero dissipation) yields a relation for the stress.

```math
\mathbf{P} = \frac{\partial a}{\partial\mathbf{F}}
```
Consequently, the tangent stiffness associated with the first Piola-Kirchoff stress is symmetric for hyperelastic constitutive models.

```math
\mathcal{C}_{iJkL} = \mathcal{C}_{kLiJ}
```

* [Arruda-Boyce](hyperelastic/arruda_boyce.md) -- The Arruda-Boyce hyperelastic constitutive model.
* [Fung](hyperelastic/fung.md) -- The Fung hyperelastic constitutive model.
* [Gent](hyperelastic/gent.md) -- The Gent hyperelastic constitutive model.
* [Mooney-Rivlin](hyperelastic/mooney_rivlin.md) -- The Mooney-Rivlin hyperelastic constitutive model.
* [Neo-Hookean](hyperelastic/neo_hookean.md) -- The Neo-Hookean hyperelastic constitutive model.
* [Saint Venant-Kirchoff](hyperelastic/saint_venant_kirchoff.md) - The Saint Venant-Kirchoff hyperelastic constitutive model.
* [Yeoh](hyperelastic/yeoh.md) -- The Yeoh hyperelastic constitutive model.

## Functions

```@docs
helmholtz_free_energy_density
```
