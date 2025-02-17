function test_elastic(model)
    σ(F) = cauchy_stress(model, F)
    P(F) = first_piola_kirchhoff_stress(model, F)
    S(F) = second_piola_kirchhoff_stress(model, F)
    @test σ(I) == Zero
    @test P(I) == Zero
    @test S(I) == Zero
    @test σ(F) ≈ transpose(σ(F))
    @test σ(F) ≈ (transpose(F) * P(F)) / det(F)
    @test σ(F) ≈ (transpose(F) * S(F) * F) / det(F)
    @test abs(σ(SimpleShearSmall)[4] / ϵ - μ) < ϵ
    @test abs(tr(σ(VolumetricSmall)) / 3 / ϵ / κ - 1) < 3 * ϵ
    T = cauchy_tangent_stiffness(model, F)
    for i = 1:3
        for j = 1:3
            for k = 1:3
                for l = 1:3
                    @test T[l, k, j, i] ≈ T[l, k, i, j]
                    Fd = copy(F)
                    Fd[l, k] += ϵ / 2
                    dσᵢⱼ = σ(Fd)[j, i]
                    Fd[l, k] -= ϵ
                    dσᵢⱼ -= σ(Fd)[j, i]
                    @test abs(T[l, k, j, i] - dσᵢⱼ / ϵ) < 1.11 * ϵ
                end
            end
        end
    end
    C = first_piola_kirchhoff_tangent_stiffness(model, F)
    for i = 1:3
        for j = 1:3
            for k = 1:3
                for l = 1:3
                    Fd = copy(F)
                    Fd[l, k] += ϵ / 2
                    dPᵢⱼ = P(Fd)[j, i]
                    Fd[l, k] -= ϵ
                    dPᵢⱼ -= P(Fd)[j, i]
                    @test abs(C[l, k, j, i] - dPᵢⱼ / ϵ) < ϵ
                end
            end
        end
    end
    G = second_piola_kirchhoff_tangent_stiffness(model, F)
    for i = 1:3
        for j = 1:3
            for k = 1:3
                for l = 1:3
                    Fd = copy(F)
                    Fd[l, k] += ϵ / 2
                    dSᵢⱼ = S(Fd)[j, i]
                    Fd[l, k] -= ϵ
                    dSᵢⱼ -= S(Fd)[j, i]
                    @test abs(G[l, k, j, i] - dSᵢⱼ / ϵ) < 2.0 * ϵ
                end
            end
        end
    end
end

@testset "Almansi-Hamel model" begin
    almansi_hamel_model = AlmansiHamel(κ, μ)
    test_elastic(almansi_hamel_model)
end
