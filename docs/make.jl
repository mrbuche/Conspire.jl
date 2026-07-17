using Conspire, Documenter

makedocs(
    format = Documenter.HTML(),
    modules = [Conspire],
    pages = [
        "Home" => "index.md",
        "Mathematics" => "math.md",
        "Constitutive" => "constitutive.md",
        "Index" => "genindex.md",
    ],
    sitename = "Conspire.jl",
)
