"""Automatic LaTeX formatting for factory-generated problems"""

def normalize_latex(latex_str: str) -> str:
    """
    Normalize LaTeX to ensure proper $ delimiters for KaTeX auto-render.
    Called automatically when problems are staged/confirmed.

    Rules:
    1. If already has $, return as-is (already formatted)
    2. If starts with \\, wrap in $ (pure LaTeX)
    3. Otherwise return as-is (plain text)

    This ensures all AI-generated scripts produce consistent output.
    """
    if not latex_str:
        return latex_str

    latex_str = latex_str.strip()

    # Already has delimiters - good!
    if '$' in latex_str:
        return latex_str

    # Starts with LaTeX command - wrap it
    if latex_str.startswith('\\'):
        return f"${latex_str}$"

    # Plain text (word problems, etc.)
    return latex_str
