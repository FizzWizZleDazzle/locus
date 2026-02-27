using PackageCompiler

project_dir = dirname(dirname(@__FILE__))
sysimage_path = joinpath(project_dir, "sysimage.so")
precompile_file = joinpath(project_dir, "build", "precompile_workload.jl")

println("Building sysimage at: $sysimage_path")
println("Using precompile workload: $precompile_file")

create_sysimage(
    [:SymEngine, :Latexify, :JSON, :Random];
    sysimage_path=sysimage_path,
    precompile_statements_file=precompile_file,
)

println("Sysimage built successfully: $sysimage_path")
