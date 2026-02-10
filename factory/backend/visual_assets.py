"""
Visual Asset Generation for Locus Factory

Generates SVG diagrams using DrawSVG and Matplotlib for:
- Geometry problems (triangles, circles, polygons)
- Statistics (histograms, distributions)
- Physics (force diagrams)
"""

import io
import base64
from typing import Optional, Dict, Any
import math

try:
    import matplotlib
    matplotlib.use('Agg')  # Non-interactive backend
    import matplotlib.pyplot as plt
    import numpy as np
    MATPLOTLIB_AVAILABLE = True
except ImportError:
    MATPLOTLIB_AVAILABLE = False
    print("Warning: matplotlib not available. Visual assets will be disabled.")


def create_triangle_svg(side_a: float, side_b: float, side_c: float) -> str:
    """
    Create an SVG of a triangle with labeled sides.
    Sides are scaled proportionally for visual accuracy.
    """
    # Use Heron's formula to find the area and then the height
    s = (side_a + side_b + side_c) / 2
    area = math.sqrt(s * (s - side_a) * (s - side_b) * (s - side_c))

    # Scale factor to make the triangle fit nicely in the SVG
    scale = 150 / max(side_a, side_b, side_c)

    # Position vertices (triangle with base side_c)
    # A at origin, B on the x-axis, C above
    ax, ay = 50, 200
    bx, by = ax + side_c * scale, 200

    # Find C using the constraint that AC = side_b and BC = side_a
    # Using law of cosines
    cos_A = (side_b**2 + side_c**2 - side_a**2) / (2 * side_b * side_c)
    angle_A = math.acos(cos_A)

    cx = ax + side_b * scale * math.cos(angle_A)
    cy = ay - side_b * scale * math.sin(angle_A)

    # Create SVG
    svg = f'''<svg width="300" height="250" xmlns="http://www.w3.org/2000/svg">
  <style>
    .triangle {{ fill: none; stroke: #2563eb; stroke-width: 2; }}
    .label {{ font-family: Arial, sans-serif; font-size: 14px; fill: #1e293b; }}
    .vertex {{ fill: #2563eb; }}
  </style>

  <!-- Triangle -->
  <polygon points="{ax},{ay} {bx},{by} {cx},{cy}" class="triangle"/>

  <!-- Vertices -->
  <circle cx="{ax}" cy="{ay}" r="3" class="vertex"/>
  <circle cx="{bx}" cy="{by}" r="3" class="vertex"/>
  <circle cx="{cx}" cy="{cy}" r="3" class="vertex"/>

  <!-- Side labels -->
  <text x="{(ax+bx)/2}" y="{ay+20}" text-anchor="middle" class="label">{side_c}</text>
  <text x="{(ax+cx)/2-10}" y="{(ay+cy)/2}" text-anchor="middle" class="label">{side_b}</text>
  <text x="{(bx+cx)/2+10}" y="{(by+cy)/2}" text-anchor="middle" class="label">{side_a}</text>

  <!-- Vertex labels -->
  <text x="{ax-15}" y="{ay+5}" class="label">A</text>
  <text x="{bx+10}" y="{by+5}" class="label">B</text>
  <text x="{cx}" y="{cy-10}" text-anchor="middle" class="label">C</text>
</svg>'''

    return svg


def create_circle_svg(radius: float, labeled_angle: Optional[float] = None) -> str:
    """Create an SVG of a circle with optional angle marking"""
    cx, cy = 150, 150
    r = min(radius * 20, 100)  # Scale for visibility

    svg = f'''<svg width="300" height="300" xmlns="http://www.w3.org/2000/svg">
  <style>
    .circle {{ fill: none; stroke: #2563eb; stroke-width: 2; }}
    .radius {{ stroke: #64748b; stroke-width: 1; stroke-dasharray: 3,3; }}
    .label {{ font-family: Arial, sans-serif; font-size: 14px; fill: #1e293b; }}
  </style>

  <circle cx="{cx}" cy="{cy}" r="{r}" class="circle"/>
  <line x1="{cx}" y1="{cy}" x2="{cx+r}" y2="{cy}" class="radius"/>
  <text x="{cx+r/2}" y="{cy-5}" text-anchor="middle" class="label">r = {radius}</text>
  <circle cx="{cx}" cy="{cy}" r="2" fill="#2563eb"/>
</svg>'''

    return svg


def create_histogram_svg(data: list, bins: int = 10) -> Optional[str]:
    """Create a histogram SVG using matplotlib"""
    if not MATPLOTLIB_AVAILABLE:
        return None

    try:
        fig, ax = plt.subplots(figsize=(6, 4))
        ax.hist(data, bins=bins, color='#2563eb', alpha=0.7, edgecolor='black')
        ax.set_xlabel('Value')
        ax.set_ylabel('Frequency')
        ax.grid(True, alpha=0.3)

        # Save to base64
        buf = io.BytesIO()
        plt.savefig(buf, format='svg', bbox_inches='tight')
        plt.close(fig)
        buf.seek(0)

        svg_data = buf.read().decode('utf-8')
        return svg_data

    except Exception as e:
        print(f"Error creating histogram: {e}")
        return None


def create_normal_distribution_svg(mean: float = 0, std: float = 1) -> Optional[str]:
    """Create a normal distribution curve SVG"""
    if not MATPLOTLIB_AVAILABLE:
        return None

    try:
        x = np.linspace(mean - 4*std, mean + 4*std, 200)
        y = (1 / (std * np.sqrt(2 * np.pi))) * np.exp(-0.5 * ((x - mean) / std)**2)

        fig, ax = plt.subplots(figsize=(6, 4))
        ax.plot(x, y, color='#2563eb', linewidth=2)
        ax.fill_between(x, y, alpha=0.3, color='#2563eb')
        ax.set_xlabel('x')
        ax.set_ylabel('Probability Density')
        ax.set_title(f'Normal Distribution (μ={mean}, σ={std})')
        ax.grid(True, alpha=0.3)
        ax.axvline(mean, color='red', linestyle='--', linewidth=1, label=f'Mean = {mean}')
        ax.legend()

        buf = io.BytesIO()
        plt.savefig(buf, format='svg', bbox_inches='tight')
        plt.close(fig)
        buf.seek(0)

        svg_data = buf.read().decode('utf-8')
        return svg_data

    except Exception as e:
        print(f"Error creating distribution: {e}")
        return None


def svg_to_base64(svg_string: str) -> str:
    """Convert SVG string to base64 data URL"""
    svg_bytes = svg_string.encode('utf-8')
    b64 = base64.b64encode(svg_bytes).decode('utf-8')
    return f"data:image/svg+xml;base64,{b64}"


def generate_geometry_asset(problem_type: str, params: Dict[str, Any]) -> Optional[str]:
    """
    Generate a geometry visualization.

    Args:
        problem_type: 'triangle', 'circle', etc.
        params: Dictionary of parameters (e.g., {'a': 3, 'b': 4, 'c': 5})

    Returns:
        Base64-encoded SVG data URL or None
    """
    try:
        if problem_type == 'triangle':
            svg = create_triangle_svg(
                params.get('a', 3),
                params.get('b', 4),
                params.get('c', 5)
            )
            return svg_to_base64(svg)
        elif problem_type == 'circle':
            svg = create_circle_svg(
                params.get('radius', 5),
                params.get('angle', None)
            )
            return svg_to_base64(svg)
        return None
    except Exception as e:
        print(f"Error generating geometry asset: {e}")
        return None


def generate_statistics_asset(chart_type: str, params: Dict[str, Any]) -> Optional[str]:
    """
    Generate a statistics visualization.

    Args:
        chart_type: 'histogram', 'normal_dist', etc.
        params: Dictionary of parameters

    Returns:
        Base64-encoded SVG data URL or None
    """
    if not MATPLOTLIB_AVAILABLE:
        return None

    try:
        if chart_type == 'histogram':
            svg = create_histogram_svg(
                params.get('data', [1, 2, 2, 3, 3, 3, 4, 4, 5]),
                params.get('bins', 10)
            )
            if svg:
                return svg_to_base64(svg)
        elif chart_type == 'normal_dist':
            svg = create_normal_distribution_svg(
                params.get('mean', 0),
                params.get('std', 1)
            )
            if svg:
                return svg_to_base64(svg)
        return None
    except Exception as e:
        print(f"Error generating statistics asset: {e}")
        return None
