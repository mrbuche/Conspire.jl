using Conspire, Documenter, TOML

cargo_toml_path = joinpath(@__DIR__, "..", "deps", "conspire_wrapper", "Cargo.toml")
cargo_data = TOML.parsefile(cargo_toml_path)
repo_url = cargo_data["dependencies"]["conspire"]["git"]
commit_hash = cargo_data["dependencies"]["conspire"]["rev"]
repo_path = joinpath(@__DIR__, "..", "conspire.rs")
if !isdir(repo_path)
    println("Cloning repository...")
    run(`git clone $repo_url $repo_path`)
else
    println("Repository already exists.")
end
println("Checking out commit hash: $commit_hash")
run(`git -C $repo_path checkout $commit_hash`)

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
