# Precompile workload for PackageCompiler sysimage.
# Exercises hot paths so they're AOT-compiled into sysimage.so.

using SymEngine
using Latexify
using JSON
using Random

# SymEngine operations
@vars x y
e1 = x^3 + 2x^2 - x + 1
diff(e1, x)
expand((x + 1)^2)
expand((x + y)^3)
factor(x^2 - 1)
subs(e1, x => Basic(2))
string(e1)

# Latexify
latexify(x^2 + 3x - 1, env=:raw)
latexify(sin(x) / cos(x), env=:raw)

# JSON
JSON.json(Dict("a" => 1, "b" => "hello", "c" => [1, 2, 3]))
JSON.json(Dict{String,Any}(
    "question_latex" => "\\frac{d}{dx}[x^3]",
    "answer_key" => "3*x^2",
    "difficulty" => 1200,
    "main_topic" => "calculus",
    "subtopic" => "derivatives",
    "grading_mode" => "equivalent",
    "answer_type" => "expression",
    "calculator_allowed" => "none",
    "solution_latex" => "",
    "question_image" => "",
    "time_limit_seconds" => nothing,
))

# Random
rand(1:10)
rand([1, 2, 3, 4, 5])

println("Precompile workload complete")
