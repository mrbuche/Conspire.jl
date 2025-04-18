function test_hyperelastic(model)
    a(F) = helmholtz_free_energy_density(model, F)
    P(F) = first_piola_kirchhoff_stress(model, F)
    @test a(I) == 0.0
    for (i, Pᵢ) in enumerate(eachrow(P(I)))
        for (j, Pᵢⱼ) in enumerate(Pᵢ)
            Fd = copy(I)
            Fd[i, j] += ϵ / 2
            da = a(Fd)
            Fd[i, j] -= ϵ
            da -= a(Fd)
            @test abs(Pᵢⱼ - da / ϵ) < ϵ
        end
    end
    @test a(F) > 0.0
    for (i, Pᵢ) in enumerate(eachrow(P(F)))
        for (j, Pᵢⱼ) in enumerate(Pᵢ)
            Fd = copy(F)
            Fd[i, j] += ϵ / 2
            da = a(Fd)
            Fd[i, j] -= ϵ
            da -= a(Fd)
            @test abs(Pᵢⱼ - da / ϵ) < ϵ
        end
    end
    C = first_piola_kirchhoff_tangent_stiffness(model, F)
    for i = 1:3
        for j = 1:3
            for k = 1:3
                for l = 1:3
                    @test C[l, k, j, i] ≈ C[j, i, l, k]
                end
            end
        end
    end
end

@testset "Arruda-Boyce model" begin
    arruda_boyce_model = ArrudaBoyce(κ, μ, N)
    test_elastic(arruda_boyce_model)
    test_hyperelastic(arruda_boyce_model)
end

@testset "Gent model" begin
    gent_model = Gent(κ, μ, Jₘ)
    test_elastic(gent_model)
    test_hyperelastic(gent_model)
end

@testset "Fung model" begin
    fung_model = Fung(κ, μ, μₘ, η)
    test_elastic(fung_model)
    test_hyperelastic(fung_model)
end

@testset "Mooney-Rivlin model" begin
    mooney_rivlin_model = MooneyRivlin(κ, μ, μₘ)
    test_elastic(mooney_rivlin_model)
    test_hyperelastic(mooney_rivlin_model)
end

@testset "Neo-Hookean model" begin
    neo_hookean_model = NeoHookean(κ, μ)
    test_elastic(neo_hookean_model)
    test_hyperelastic(neo_hookean_model)
end

@testset "Saint Venant-Kirchhoff model" begin
    saint_venant_kirchhoff_model = SaintVenantKirchhoff(κ, μ)
    test_elastic(saint_venant_kirchhoff_model)
    test_hyperelastic(saint_venant_kirchhoff_model)
end
