"""
Shared utilities for Julia problem generation scripts.

Mirrors the API of factory/backend/scripts/problem_utils.py.
Usage: include(joinpath(@__DIR__, "..", "src", "ProblemUtils.jl"))
   or: using ProblemUtils  (when LOAD_PATH is set)
"""
module ProblemUtils

using SymEngine
using Latexify
using JSON
using Random

export @vars,
    # Re-export SymEngine basics
    Basic, diff, expand, factor, subs, free_symbols,
    sin, cos, tan, exp, log, sqrt, abs,
    PI, E, oo,
    # Helpers
    problem, emit, steps, nonzero,
    fmt_set, fmt_tuple, fmt_list, fmt_matrix, fmt_interval, fmt_equation, fmt_multipart,
    compress_svg, decompress_svg, tex,
    # Random re-exports
    rand, randint, choice

# ---------------------------------------------------------------------------
# Random helpers (match Python API)
# ---------------------------------------------------------------------------

"""Random integer in [lo, hi] (inclusive on both ends, like Python's random.randint)."""
randint(lo::Int, hi::Int) = rand(lo:hi)

"""Pick a random element from a collection."""
choice(v) = rand(v)

# ---------------------------------------------------------------------------
# Symbolic constants
# ---------------------------------------------------------------------------

const PI = Basic(pi)
const E  = exp(Basic(1))
const oo = Basic(Inf)

# ---------------------------------------------------------------------------
# tex() helper
# ---------------------------------------------------------------------------

"""Convert a SymEngine expression to LaTeX string."""
function tex(expr)
    s = string(latexify(expr, env=:raw))
    # Latexify wraps in $...$, strip them
    if startswith(s, "\$") && endswith(s, "\$")
        s = s[2:end-1]
    end
    return s
end

# ---------------------------------------------------------------------------
# Small helpers
# ---------------------------------------------------------------------------

"""Random integer in [lo, hi] excluding 0."""
function nonzero(lo::Int, hi::Int)::Int
    while true
        v = rand(lo:hi)
        v != 0 && return v
    end
end

"""Join solution steps with <br> for solution_latex."""
steps(strings::String...) = join(strings, "<br>")

# ---------------------------------------------------------------------------
# Answer formatting helpers
# ---------------------------------------------------------------------------

"""Format elements as a set string: {1, 2, 3}"""
function fmt_set(elements...)
    elems = try
        sort(collect(elements), by=x -> Float64(x))
    catch
        collect(elements)
    end
    return "{" * join(string.(elems), ", ") * "}"
end

"""Format as ordered tuple: (3, 5)"""
fmt_tuple(elements...) = "(" * join(string.(elements), ", ") * ")"

"""Format as list: [-2, 2]"""
fmt_list(elements...) = "[" * join(string.(elements), ", ") * "]"

"""Format a matrix (vector of vectors) as [[r1], [r2], ...]"""
function fmt_matrix(rows)
    row_strs = ["[" * join(string.(row), ", ") * "]" for row in rows]
    return "[" * join(row_strs, ", ") * "]"
end

"""Format interval in DB format: open:1,closed:7"""
function fmt_interval(left, right; left_open::Bool=false, right_open::Bool=false)
    l_type = left_open ? "open" : "closed"
    r_type = right_open ? "open" : "closed"
    return "$(l_type):$(left),$(r_type):$(right)"
end

"""Format as equation string: lhs = rhs"""
fmt_equation(lhs, rhs) = "$(lhs) = $(rhs)"

"""Format multiple answer parts: type:value|||type:value"""
fmt_multipart(parts::String...) = join(parts, "|||")

# ---------------------------------------------------------------------------
# Auto-detection and conversion
# ---------------------------------------------------------------------------

const _VALID_ANSWER_TYPES = Set([
    "expression", "numeric", "set", "tuple", "list",
    "interval", "inequality", "equation", "boolean",
    "word", "matrix", "multi_part",
])
const _VALID_GRADING_MODES = Set(["equivalent", "factor", "expand"])
const _VALID_CALCULATOR = Set(["none", "scientific", "graphing", "cas"])

"""Auto-detect answer_type from Julia value."""
function _detect_answer_type(answer)
    answer isa Bool && return "boolean"
    answer isa Integer && return "numeric"
    answer isa AbstractFloat && return "numeric"
    # SymEngine Basic with no free symbols -> numeric
    if answer isa Basic
        fs = free_symbols(answer)
        if isempty(fs)
            return "numeric"
        end
    end
    # Vector of vectors -> matrix
    if answer isa AbstractVector && !isempty(answer) && first(answer) isa AbstractVector
        return "matrix"
    end
    answer isa Tuple && return "tuple"
    answer isa AbstractVector && return "list"
    answer isa AbstractSet && return "set"
    return "expression"
end

"""Convert answer to string based on detected type."""
function _answer_to_str(answer, answer_type::String)
    answer isa Bool && return answer ? "true" : "false"
    # Matrix (vector of vectors)
    if answer_type == "matrix" && answer isa AbstractVector
        return fmt_matrix(answer)
    end
    if answer_type == "set" && answer isa AbstractSet
        return fmt_set(collect(answer)...)
    end
    answer isa Tuple && return fmt_tuple(answer...)
    if answer_type == "list" && answer isa AbstractVector
        return fmt_list(answer...)
    end
    return string(answer)
end

# ---------------------------------------------------------------------------
# SVG compression
# ---------------------------------------------------------------------------

const _SVG_DICT = [
    ("~X", "xmlns=\"http://www.w3.org/2000/svg\""),
    ("~D", "dominant-baseline=\"central\""),
    ("~s", "stroke=\"currentColor\""),
    ("~f", "fill=\"currentColor\""),
    ("~n", "fill=\"none\""),
    ("~m", "text-anchor=\"middle\""),
    ("~e", "text-anchor=\"end\""),
    ("~d", "stroke-dasharray=\"6,4\""),
    ("~o", "stroke-opacity=\""),
    ("~i", "font-style=\"italic\""),
    ("~F", "fill-opacity=\"0.15\""),
    ("~w", "stroke-width=\""),
    ("~z", "font-size=\""),
    ("~v", "viewBox=\""),
    ("~M", "style=\"max-width:"),
    ("~L", "<line "),
    ("~T", "<text "),
    ("~C", "<circle "),
    ("~P", "<path d=\""),
    ("~Q", "<polyline points=\""),
    ("~G", "<polygon points=\""),
    ("~E", "</text>"),
    ("~g", "class=\"g\""),
    ("~a", "class=\"a\""),
    ("~t", "class=\"t\""),
]

"""Replace common SVG substrings with short tokens, prepend 's1:' prefix."""
function compress_svg(svg::String)
    out = svg
    for (token, expansion) in _SVG_DICT
        out = replace(out, expansion => token)
    end
    return "s1:" * out
end

"""Expand a compressed SVG string (s1:... prefix) back to a full SVG."""
function decompress_svg(s::String)
    isempty(s) && return ""
    body = startswith(s, "s1:") ? s[4:end] : s
    for (token, expansion) in _SVG_DICT
        body = replace(body, token => expansion)
    end
    return body
end

# ---------------------------------------------------------------------------
# Main API
# ---------------------------------------------------------------------------

"""
    problem(; question, answer, difficulty, topic, solution="",
              grading_mode="equivalent", answer_type=nothing, calculator="none",
              image="", time=nothing) -> Dict

Build a complete problem dict with validation.

- `difficulty`: Int ELO or (lo, hi) tuple for random range
- `topic`: "main/sub" format string
"""
function problem(; question, answer, difficulty, topic, solution="",
                   grading_mode="equivalent", answer_type=nothing, calculator="none",
                   image="", time=nothing)
    # Handle difficulty tuple
    if difficulty isa Tuple
        difficulty = rand(difficulty[1]:difficulty[2])
    end

    # Split topic
    parts = split(topic, "/", limit=2)
    length(parts) != 2 && error("topic must be 'main/sub' format, got: $(repr(topic))")
    main_topic, subtopic = parts

    # Auto-detect answer_type
    if answer_type === nothing
        answer_type = _detect_answer_type(answer)
    end

    # Convert answer to string
    answer_key = _answer_to_str(answer, answer_type)

    # Validate
    grading_mode in _VALID_GRADING_MODES || error("Invalid grading_mode: $(repr(grading_mode))")
    answer_type in _VALID_ANSWER_TYPES || error("Invalid answer_type: $(repr(answer_type))")
    calculator in _VALID_CALCULATOR || error("Invalid calculator: $(repr(calculator))")
    if time !== nothing
        time isa Integer || error("time must be an Int, got $(typeof(time))")
        (1 <= time <= 3600) || error("time must be between 1 and 3600, got $(time)")
    end

    return Dict{String,Any}(
        "question_latex" => question,
        "answer_key" => answer_key,
        "difficulty" => difficulty,
        "main_topic" => String(main_topic),
        "subtopic" => String(subtopic),
        "grading_mode" => grading_mode,
        "answer_type" => answer_type,
        "calculator_allowed" => calculator,
        "solution_latex" => solution,
        "question_image" => !isempty(image) ? compress_svg(image) : "",
        "time_limit_seconds" => time,
    )
end

"""Print problem as JSON line to stdout (JSONL format)."""
emit(d::Dict) = println(JSON.json(d))

# ---------------------------------------------------------------------------
# Batch boilerplate helper
# ---------------------------------------------------------------------------

"""
Parse --count N from ARGS and call generate() that many times.
Call this at the end of every script: run_batch(generate)
"""
function run_batch(generate_fn::Function)
    count = 1
    for i in 1:length(ARGS)-1
        if ARGS[i] == "--count"
            count = parse(Int, ARGS[i+1])
        end
    end
    for _ in 1:count
        emit(generate_fn())
    end
end

export run_batch

end # module
