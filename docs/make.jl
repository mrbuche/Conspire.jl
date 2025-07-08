using Conspire, Documenter

# set something up to clone (if not there) based on commit hash in deps/ and .gitignore it

makedocs(
    format = Documenter.HTML(),
    modules = [Conspire],
    pages = [
        "Home" => "index.md",
        "Constitutive" => "constitutive.md",
        "Index" => "genindex.md",
    ],
    sitename = "Conspire.jl",
)
