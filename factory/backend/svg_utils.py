"""SVG diagram builders for geometry and graphing problems.

Two classes:
  Diagram — geometry diagrams (polygons, angles, circles, etc.)
  Graph   — function plots on a coordinate grid

Both produce compact, dark-mode-compatible SVG strings using currentColor.
"""

import math

# ---------------------------------------------------------------------------
# Diagram — geometry diagrams
# ---------------------------------------------------------------------------

class Diagram:
    """Build geometry diagrams in math coordinates (y-up).

    Coordinates are auto-scaled to fit the SVG viewBox at render time.

    Example:
        d = Diagram()
        d.polygon([(0,0), (4,0), (2,3)], labels=['A', 'B', 'C'])
        d.angle_arc((0,0), (4,0), (2,3), label='60°')
        svg = d.render()
    """

    def __init__(self, width=300, height=250, padding=40):
        self._width = width
        self._height = height
        self._padding = padding
        self._points = []      # all points for auto-bounds
        self._elements = []    # (render_fn, ...) deferred

    # -- coordinate helpers --------------------------------------------------

    def _register(self, *pts):
        for p in pts:
            self._points.append((float(p[0]), float(p[1])))

    def _bounds(self):
        xs = [p[0] for p in self._points]
        ys = [p[1] for p in self._points]
        return min(xs), max(xs), min(ys), max(ys)

    def _transform(self, px, py, x_min, y_min, sx, sy):
        """Math coords -> SVG coords (y-flip)."""
        svgx = self._padding + (px - x_min) * sx
        svgy = self._padding + (y_min - py + (self._bounds()[3] - self._bounds()[2])) * sy
        return round(svgx, 1), round(svgy, 1)

    # -- public API ----------------------------------------------------------

    def line(self, p1, p2, dashed=False):
        self._register(p1, p2)
        self._elements.append(('line', p1, p2, dashed))

    def polygon(self, points, labels=None, fill=None):
        for p in points:
            self._register(p)
        self._elements.append(('polygon', list(points), labels, fill))

    def circle(self, center, radius, fill=None):
        self._register(center,
                       (center[0] - radius, center[1]),
                       (center[0] + radius, center[1]),
                       (center[0], center[1] - radius),
                       (center[0], center[1] + radius))
        self._elements.append(('circle', center, radius, fill))

    def arc(self, center, radius, start_deg, end_deg):
        # register arc endpoints for bounds
        s = math.radians(start_deg)
        e = math.radians(end_deg)
        self._register(center,
                       (center[0] + radius * math.cos(s), center[1] + radius * math.sin(s)),
                       (center[0] + radius * math.cos(e), center[1] + radius * math.sin(e)))
        self._elements.append(('arc', center, radius, start_deg, end_deg))

    def point(self, x, y, label=None):
        self._register((x, y))
        self._elements.append(('point', (x, y), label))

    def angle_arc(self, vertex, p1, p2, label=None):
        self._register(vertex, p1, p2)
        self._elements.append(('angle_arc', vertex, p1, p2, label))

    def right_angle(self, vertex, p1, p2):
        self._register(vertex, p1, p2)
        self._elements.append(('right_angle', vertex, p1, p2))

    def segment_label(self, p1, p2, text):
        self._register(p1, p2)
        self._elements.append(('segment_label', p1, p2, text))

    def tick_marks(self, p1, p2, count=1):
        self._register(p1, p2)
        self._elements.append(('tick_marks', p1, p2, count))

    def text(self, x, y, text):
        self._register((x, y))
        self._elements.append(('text', (x, y), text))

    # -- render --------------------------------------------------------------

    def render(self):
        if not self._points:
            return '<svg xmlns="http://www.w3.org/2000/svg"></svg>'

        x_min, x_max, y_min, y_max = self._bounds()
        dx = x_max - x_min or 1
        dy = y_max - y_min or 1
        inner_w = self._width - 2 * self._padding
        inner_h = self._height - 2 * self._padding
        scale = min(inner_w / dx, inner_h / dy)
        sx = sy = scale  # uniform

        def tx(px, py):
            return self._transform(px, py, x_min, y_min, sx, sy)

        parts = []
        for el in self._elements:
            kind = el[0]

            if kind == 'line':
                _, p1, p2, dashed = el
                x1, y1 = tx(*p1)
                x2, y2 = tx(*p2)
                dash = ' stroke-dasharray="6,4"' if dashed else ''
                parts.append(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="currentColor" stroke-width="2"{dash}/>')

            elif kind == 'polygon':
                _, points, labels, fill = el
                coords = [tx(*p) for p in points]
                pts_str = " ".join(f"{x},{y}" for x, y in coords)
                fill_attr = f'fill="{fill}" fill-opacity="0.15"' if fill else 'fill="none"'
                parts.append(f'<polygon points="{pts_str}" stroke="currentColor" stroke-width="2" {fill_attr}/>')
                if labels:
                    cx = sum(c[0] for c in coords) / len(coords)
                    cy = sum(c[1] for c in coords) / len(coords)
                    for i, lbl in enumerate(labels):
                        if lbl is None:
                            continue
                        px, py = coords[i]
                        # push label away from centroid by fixed distance
                        dx = px - cx
                        dy = py - cy
                        dist = math.hypot(dx, dy) or 1
                        ox = dx / dist * 18
                        oy = dy / dist * 18
                        parts.append(f'<text x="{round(px + ox, 1)}" y="{round(py + oy + 5, 1)}" '
                                     f'font-size="14" fill="currentColor" font-style="italic" '
                                     f'text-anchor="middle">{lbl}</text>')

            elif kind == 'circle':
                _, center, radius, fill = el
                cx, cy = tx(*center)
                # radius in SVG units
                r = round(radius * sx, 1)
                fill_attr = f'fill="{fill}" fill-opacity="0.15"' if fill else 'fill="none"'
                parts.append(f'<circle cx="{cx}" cy="{cy}" r="{r}" stroke="currentColor" stroke-width="2" {fill_attr}/>')

            elif kind == 'arc':
                _, center, radius, start_deg, end_deg = el
                r = radius * sx
                s_rad = math.radians(start_deg)
                e_rad = math.radians(end_deg)
                # start/end in SVG coords (y-flip means negate angles)
                sx1 = center[0] + radius * math.cos(s_rad)
                sy1 = center[1] + radius * math.sin(s_rad)
                sx2 = center[0] + radius * math.cos(e_rad)
                sy2 = center[1] + radius * math.sin(e_rad)
                x1, y1 = tx(sx1, sy1)
                x2, y2 = tx(sx2, sy2)
                sweep_angle = (end_deg - start_deg) % 360
                large = 1 if sweep_angle > 180 else 0
                parts.append(f'<path d="M{x1},{y1} A{round(r,1)},{round(r,1)} 0 {large} 0 {x2},{y2}" '
                             f'stroke="currentColor" stroke-width="2" fill="none"/>')

            elif kind == 'point':
                _, pos, label = el
                px, py = tx(*pos)
                parts.append(f'<circle cx="{px}" cy="{py}" r="3" fill="currentColor"/>')
                if label:
                    parts.append(f'<text x="{round(px + 6, 1)}" y="{round(py - 6, 1)}" '
                                 f'font-size="13" fill="currentColor">{label}</text>')

            elif kind == 'angle_arc':
                _, vertex, p1, p2, label = el
                vx, vy = tx(*vertex)
                p1x, p1y = tx(*p1)
                p2x, p2y = tx(*p2)
                # angles in SVG coord space from vertex to each point
                a1 = math.atan2(p1y - vy, p1x - vx)
                a2 = math.atan2(p2y - vy, p2x - vx)
                r = 25
                x1 = round(vx + r * math.cos(a1), 1)
                y1 = round(vy + r * math.sin(a1), 1)
                x2 = round(vx + r * math.cos(a2), 1)
                y2 = round(vy + r * math.sin(a2), 1)
                # pick the shorter arc (interior angle)
                cw = (a1 - a2) % (2 * math.pi)
                ccw = (a2 - a1) % (2 * math.pi)
                if ccw <= cw:
                    # shorter path is CCW in SVG math, but SVG y-down
                    # flips visual sense, so use sweep=1
                    sweep = 1
                    mid_a = a1 + ccw / 2
                else:
                    sweep = 0
                    mid_a = a1 - cw / 2
                parts.append(f'<path d="M{x1},{y1} A{r},{r} 0 0 {sweep} {x2},{y2}" '
                             f'stroke="currentColor" stroke-width="1.5" fill="none"/>')
                if label:
                    lx = round(vx + (r + 14) * math.cos(mid_a), 1)
                    ly = round(vy + (r + 14) * math.sin(mid_a), 1)
                    parts.append(f'<text x="{lx}" y="{ly}" font-size="12" fill="currentColor" '
                                 f'text-anchor="middle" dominant-baseline="central">{label}</text>')

            elif kind == 'right_angle':
                _, vertex, p1, p2 = el
                vx, vy = tx(*vertex)
                a1 = math.atan2(tx(*p1)[1] - vy, tx(*p1)[0] - vx)
                a2 = math.atan2(tx(*p2)[1] - vy, tx(*p2)[0] - vx)
                s = 12
                cx1 = round(vx + s * math.cos(a1), 1)
                cy1 = round(vy + s * math.sin(a1), 1)
                cx2 = round(vx + s * math.cos(a2), 1)
                cy2 = round(vy + s * math.sin(a2), 1)
                corner_x = round(vx + s * math.cos(a1) + s * math.cos(a2), 1)
                corner_y = round(vy + s * math.sin(a1) + s * math.sin(a2), 1)
                parts.append(f'<path d="M{cx1},{cy1} L{corner_x},{corner_y} L{cx2},{cy2}" '
                             f'stroke="currentColor" stroke-width="1.5" fill="none"/>')

            elif kind == 'segment_label':
                _, p1, p2, text = el
                x1, y1 = tx(*p1)
                x2, y2 = tx(*p2)
                mx = round((x1 + x2) / 2, 1)
                my = round((y1 + y2) / 2, 1)
                # offset perpendicular to segment
                dx_s = x2 - x1
                dy_s = y2 - y1
                length = math.hypot(dx_s, dy_s) or 1
                nx = -dy_s / length * 14
                ny = dx_s / length * 14
                parts.append(f'<text x="{round(mx + nx, 1)}" y="{round(my + ny, 1)}" '
                             f'font-size="13" fill="currentColor" text-anchor="middle" '
                             f'dominant-baseline="central">{text}</text>')

            elif kind == 'tick_marks':
                _, p1, p2, count = el
                x1, y1 = tx(*p1)
                x2, y2 = tx(*p2)
                mx = (x1 + x2) / 2
                my = (y1 + y2) / 2
                dx_s = x2 - x1
                dy_s = y2 - y1
                length = math.hypot(dx_s, dy_s) or 1
                nx = -dy_s / length
                ny = dx_s / length
                tx_dir = dx_s / length
                ty_dir = dy_s / length
                spacing = 4
                for i in range(count):
                    offset = (i - (count - 1) / 2) * spacing
                    cx = mx + offset * tx_dir
                    cy = my + offset * ty_dir
                    tx1 = round(cx + nx * 5, 1)
                    ty1 = round(cy + ny * 5, 1)
                    tx2 = round(cx - nx * 5, 1)
                    ty2 = round(cy - ny * 5, 1)
                    parts.append(f'<line x1="{tx1}" y1="{ty1}" x2="{tx2}" y2="{ty2}" '
                                 f'stroke="currentColor" stroke-width="1.5"/>')

            elif kind == 'text':
                _, pos, text = el
                px, py = tx(*pos)
                parts.append(f'<text x="{px}" y="{py}" font-size="13" fill="currentColor">{text}</text>')

        vb = f"0 0 {self._width} {self._height}"
        body = "".join(parts)
        return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{vb}" '
                f'style="max-width:{self._width}px" fill="currentColor">{body}</svg>')


# ---------------------------------------------------------------------------
# Graph — function plots on a coordinate grid
# ---------------------------------------------------------------------------

_PLOT_COLORS = ['#3b82f6', '#ef4444', '#10b981', '#f59e0b', '#8b5cf6', '#ec4899']

class Graph:
    """Plot SymPy expressions on a coordinate grid.

    Example:
        g = Graph(x_range=(-5, 5), y_range=(-5, 5))
        g.plot(x**2 - 3, color='blue')
        g.point(2, 1, label='(2, 1)')
        svg = g.render()
    """

    def __init__(self, x_range=(-5, 5), y_range=(-5, 5), width=300, height=300, padding=35):
        self._x_min, self._x_max = x_range
        self._y_min, self._y_max = y_range
        self._width = width
        self._height = height
        self._padding = padding
        self._plots = []
        self._points = []
        self._vlines = []
        self._hlines = []
        self._color_idx = 0

    def _next_color(self):
        c = _PLOT_COLORS[self._color_idx % len(_PLOT_COLORS)]
        self._color_idx += 1
        return c

    def _tx(self, mx, my):
        """Math coords -> SVG coords."""
        inner_w = self._width - 2 * self._padding
        inner_h = self._height - 2 * self._padding
        sx = inner_w / (self._x_max - self._x_min)
        sy = inner_h / (self._y_max - self._y_min)
        svgx = self._padding + (mx - self._x_min) * sx
        svgy = self._padding + (self._y_max - my) * sy
        return round(svgx, 1), round(svgy, 1)

    def plot(self, expr, color=None, dashed=False):
        self._plots.append((expr, color or self._next_color(), dashed))

    def point(self, x, y, label=None):
        self._points.append((x, y, label))

    def vline(self, x, dashed=True):
        self._vlines.append((x, dashed))

    def hline(self, y, dashed=True):
        self._hlines.append((y, dashed))

    def render(self):
        parts = []

        pad = self._padding
        inner_w = self._width - 2 * pad
        inner_h = self._height - 2 * pad
        t = pad + inner_h  # bottom edge
        r = pad + inner_w  # right edge

        # Use CSS classes to cut SVG size
        style = ('<style>'
                 '.g{stroke:currentColor;stroke-opacity:.12;stroke-width:1}'
                 '.a{stroke:currentColor;stroke-width:1.5}'
                 '.t{font-size:10px;fill:currentColor}'
                 '</style>')
        parts.append(style)

        # grid lines
        for xi in range(int(self._x_min), int(self._x_max) + 1):
            x, _ = self._tx(xi, 0)
            parts.append(f'<line x1="{x}" y1="{pad}" x2="{x}" y2="{t}" class="g"/>')
        for yi in range(int(self._y_min), int(self._y_max) + 1):
            _, y = self._tx(0, yi)
            parts.append(f'<line x1="{pad}" y1="{y}" x2="{r}" y2="{y}" class="g"/>')

        # axes
        if self._y_min <= 0 <= self._y_max:
            _, ay = self._tx(0, 0)
            parts.append(f'<line x1="{pad}" y1="{ay}" x2="{r}" y2="{ay}" class="a"/>')
        if self._x_min <= 0 <= self._x_max:
            ax, _ = self._tx(0, 0)
            parts.append(f'<line x1="{ax}" y1="{pad}" x2="{ax}" y2="{t}" class="a"/>')

        # tick labels on x-axis
        for xi in range(int(self._x_min), int(self._x_max) + 1):
            if xi == 0:
                continue
            x, y0 = self._tx(xi, 0)
            yl = min(y0 + 16, t + 16)
            parts.append(f'<text x="{x}" y="{yl}" class="t" text-anchor="middle">{xi}</text>')

        # tick labels on y-axis
        for yi in range(int(self._y_min), int(self._y_max) + 1):
            if yi == 0:
                continue
            x0, y = self._tx(0, yi)
            xl = max(x0 - 8, pad - 20)
            parts.append(f'<text x="{xl}" y="{round(y + 4, 1)}" class="t" text-anchor="end">{yi}</text>')

        # vertical lines
        for vx, dashed in self._vlines:
            x, _ = self._tx(vx, 0)
            dash = ' stroke-dasharray="6,4"' if dashed else ''
            parts.append(f'<line x1="{x}" y1="{pad}" x2="{x}" y2="{t}" '
                         f'stroke="currentColor" stroke-opacity="0.5" stroke-width="1.5"{dash}/>')

        # horizontal lines
        for hy, dashed in self._hlines:
            _, y = self._tx(0, hy)
            dash = ' stroke-dasharray="6,4"' if dashed else ''
            parts.append(f'<line x1="{pad}" y1="{y}" x2="{r}" y2="{y}" '
                         f'stroke="currentColor" stroke-opacity="0.5" stroke-width="1.5"{dash}/>')

        # plot curves
        from sympy import lambdify, Symbol
        sym_x = Symbol('x')
        n_pts = 100
        for expr, color, dashed in self._plots:
            try:
                f = lambdify(sym_x, expr, 'math')
            except Exception:
                continue

            step = (self._x_max - self._x_min) / n_pts
            segments = []
            current_seg = []
            y_span = (self._y_max - self._y_min) * 2

            for i in range(n_pts + 1):
                xv = self._x_min + i * step
                try:
                    yv = float(f(xv))
                except Exception:
                    if current_seg:
                        segments.append(current_seg)
                        current_seg = []
                    continue

                if math.isnan(yv) or math.isinf(yv):
                    if current_seg:
                        segments.append(current_seg)
                        current_seg = []
                    continue

                # discontinuity detection
                if current_seg:
                    prev_y = current_seg[-1][1]
                    if abs(yv - prev_y) > y_span:
                        segments.append(current_seg)
                        current_seg = []

                current_seg.append((xv, yv))

            if current_seg:
                segments.append(current_seg)

            dash_attr = ' stroke-dasharray="6,4"' if dashed else ''
            for seg in segments:
                if len(seg) < 2:
                    continue
                pts_str = " ".join(f"{int(self._tx(px, py)[0])},{int(self._tx(px, py)[1])}" for px, py in seg)
                parts.append(f'<polyline points="{pts_str}" stroke="{color}" '
                             f'stroke-width="2" fill="none" stroke-opacity="0.85"{dash_attr}/>')

        # points
        for px, py, label in self._points:
            sx, sy = self._tx(px, py)
            parts.append(f'<circle cx="{sx}" cy="{sy}" r="3.5" fill="currentColor"/>')
            if label:
                parts.append(f'<text x="{round(sx + 7, 1)}" y="{round(sy - 7, 1)}" '
                             f'font-size="12" fill="currentColor">{label}</text>')

        vb = f"0 0 {self._width} {self._height}"
        body = "".join(parts)
        return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{vb}" '
                f'style="max-width:{self._width}px">{body}</svg>')
