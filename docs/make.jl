using Conspire, Documenter

# set something up to shallow clone (if not there) based on commit hash in deps/

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
