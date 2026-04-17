-- Seed a sample projectile motion problem for testing

INSERT INTO physics_problems (
    title,
    description_latex,
    difficulty,
    physics_topic,
    physics_subtopic,
    scene_definition,
    parameters,
    challenge_stages,
    what_if_prompts,
    common_errors,
    answer_spec
) VALUES (
    'Ball Launched at an Angle',
    'A ball is launched from the ground at $30°$ above the horizontal with an initial speed of $20\,\text{m/s}$. Ignoring air resistance, find the maximum height reached and the total horizontal range.',
    2,
    'mechanics',
    'projectile_motion',

    -- scene_definition
    '{
        "gravity": [0.0, -9.81],
        "bodies": [
            {
                "id": "ball",
                "label": "Ball",
                "body_type": "dynamic",
                "shape": { "type": "circle", "radius": 0.3 },
                "position": [0.0, 0.3],
                "rotation": 0.0,
                "velocity": [17.32, 10.0],
                "material": { "restitution": 0.5, "static_friction": 0.3, "kinetic_friction": 0.2 },
                "mass": 1.0,
                "fill_color": "#3b82f6",
                "stroke_color": "#1e40af"
            }
        ],
        "constraints": [],
        "boundaries": [
            {
                "id": "ground",
                "start": [-5.0, 0.0],
                "end": [40.0, 0.0],
                "material": { "restitution": 0.0, "static_friction": 0.5, "kinetic_friction": 0.4 }
            }
        ],
        "camera": [15.0, 3.0, 1.0],
        "pixels_per_metre": 20.0
    }'::jsonb,

    -- parameters
    '[
        { "key": "ball.velocity.x", "label": "Horizontal velocity", "default": 17.32, "min": 5.0, "max": 30.0, "step": 0.5, "unit": "m/s", "target": "ball.velocity.x" },
        { "key": "ball.velocity.y", "label": "Vertical velocity", "default": 10.0, "min": 5.0, "max": 30.0, "step": 0.5, "unit": "m/s", "target": "ball.velocity.y" },
        { "key": "ball.mass", "label": "Mass", "default": 1.0, "min": 0.5, "max": 10.0, "step": 0.5, "unit": "kg", "target": "ball.mass" }
    ]'::jsonb,

    -- challenge_stages
    '[
        {
            "id": "identify",
            "title": "Identify quantities",
            "prompt_text": "Before solving, identify which physical quantities are relevant to this projectile motion problem.",
            "hint_text": "Think about what determines how high and how far the ball goes.",
            "success_response": "Correct! The initial velocity, launch angle, and gravity are all you need — mass does not affect projectile motion (without air resistance).",
            "stage_data": {
                "type": "identify_quantities",
                "correct": [
                    { "id": "velocity", "label": "Initial velocity", "symbol_latex": "v_0" },
                    { "id": "angle", "label": "Launch angle", "symbol_latex": "\\theta" },
                    { "id": "gravity", "label": "Gravitational acceleration", "symbol_latex": "g" }
                ],
                "distractors": [
                    { "id": "mass", "label": "Mass", "symbol_latex": "m" },
                    { "id": "friction", "label": "Friction coefficient", "symbol_latex": "\\mu" },
                    { "id": "spring_k", "label": "Spring constant", "symbol_latex": "k" }
                ],
                "explanations": {
                    "velocity": "Yes — the initial velocity determines both the horizontal range and maximum height.",
                    "angle": "Yes — the launch angle splits the velocity into horizontal and vertical components.",
                    "gravity": "Yes — gravity determines how quickly the vertical velocity changes.",
                    "mass": "Mass does not affect projectile motion when air resistance is ignored. All objects fall at the same rate!",
                    "friction": "There is no friction in this problem — the ball is moving through the air.",
                    "spring_k": "There is no spring in this problem."
                }
            }
        },
        {
            "id": "fbd",
            "title": "Free-body diagram",
            "prompt_text": "Draw the free-body diagram for the ball while it is in flight. What forces act on it?",
            "hint_text": "After launch, what is the only force acting on the ball? (We are ignoring air resistance.)",
            "success_response": "Correct! The only force on the ball during flight is gravity, acting straight down.",
            "stage_data": {
                "type": "freebody_diagram",
                "target_body": "ball",
                "expected_forces": [
                    { "id": "gravity", "label": "Weight (mg)", "direction_deg": 270.0, "color": "#ef4444", "label_latex": "mg" }
                ],
                "direction_tolerance_deg": 15.0,
                "per_force_hints": {
                    "gravity": "Gravity always acts straight downward."
                }
            }
        },
        {
            "id": "equations",
            "title": "Set up equations",
            "prompt_text": "Decompose the motion into horizontal and vertical components. What is the vertical velocity component?",
            "hint_text": "Use trigonometry: the vertical component is v₀ sin(θ).",
            "success_response": "Correct! v_y = v₀ sin(30°) = 20 × 0.5 = 10 m/s, and v_x = v₀ cos(30°) = 20 × 0.866 = 17.32 m/s.",
            "stage_data": {
                "type": "equation_builder",
                "axis_label": "vertical (y-axis)",
                "correct_terms": [
                    { "id": "v0_sin", "latex": "v_0 \\sin\\theta", "sign": "+" },
                    { "id": "neg_gt", "latex": "gt", "sign": "-" }
                ],
                "available_terms": [
                    { "id": "v0_sin", "latex": "v_0 \\sin\\theta", "sign": "+" },
                    { "id": "v0_cos", "latex": "v_0 \\cos\\theta", "sign": "+" },
                    { "id": "neg_gt", "latex": "gt", "sign": "-" },
                    { "id": "pos_gt", "latex": "gt", "sign": "+" },
                    { "id": "half_gt2", "latex": "\\frac{1}{2}gt^2", "sign": "-" }
                ],
                "error_feedback": {
                    "v0_cos,neg_gt": "You used cos instead of sin. The vertical component uses sin(θ), not cos(θ)."
                }
            }
        },
        {
            "id": "predict",
            "title": "Predict",
            "prompt_text": "Now calculate: what is the maximum height the ball reaches?",
            "hint_text": "At maximum height, vertical velocity = 0. Use v² = v₀² - 2gh.",
            "success_response": "The simulation confirms your calculation!",
            "stage_data": {
                "type": "prediction",
                "question": "What is the maximum height reached by the ball? (in metres)",
                "answer": 5.1,
                "unit": "m",
                "tolerance_pct": 10.0,
                "sim_runs_after": true,
                "sim_end_time": 3.0
            }
        },
        {
            "id": "reflect",
            "title": "Reflect",
            "prompt_text": "Compare your prediction with the simulation result.",
            "hint_text": null,
            "success_response": "Great self-diagnosis! Understanding where errors come from is the key to not repeating them.",
            "stage_data": {
                "type": "reflection",
                "trigger": "wrong_prediction",
                "diagnostic_options": [
                    { "id": "wrong_component", "label": "I used the wrong velocity component (cos instead of sin)", "is_correct": true },
                    { "id": "forgot_half", "label": "I forgot the 1/2 factor in the kinematic equation", "is_correct": false },
                    { "id": "wrong_g", "label": "I used the wrong value for g", "is_correct": false },
                    { "id": "arithmetic", "label": "I made an arithmetic error", "is_correct": false },
                    { "id": "unsure", "label": "I am not sure what went wrong", "is_correct": false }
                ],
                "micro_lessons": {
                    "wrong_component": {
                        "explanation_latex": "The vertical component of velocity is $v_y = v_0 \\sin\\theta$, not $v_0 \\cos\\theta$. Remember: sine goes with the \\textbf{opposite} side — here the vertical component is opposite to the angle measured from horizontal.",
                        "visual_overlay": "velocity_decomposition"
                    },
                    "unsure": {
                        "explanation_latex": "At max height, $v_y = 0$. Using $v_y^2 = v_{0y}^2 - 2gh$: $0 = (10)^2 - 2(9.81)h$, giving $h = 100/19.62 \\approx 5.1\\,\\text{m}$.",
                        "visual_overlay": null
                    }
                }
            }
        }
    ]'::jsonb,

    -- what_if_prompts
    '[
        {
            "question": "What happens to the maximum height if you double the mass?",
            "parameter_key": "ball.mass",
            "suggested_value": 2.0,
            "expected_insight": "The height does not change! Mass cancels out in projectile motion (without air resistance). This is why a feather and a bowling ball fall at the same rate in a vacuum."
        },
        {
            "question": "What launch angle gives the maximum range?",
            "parameter_key": "ball.velocity.y",
            "suggested_value": 14.14,
            "expected_insight": "45° gives the maximum range for a given launch speed (when launch and landing heights are equal). At 45°, v_x = v_y = v₀/√2."
        }
    ]'::jsonb,

    -- common_errors
    '[
        {
            "id": "wrong_component",
            "description": "Using cos instead of sin for the vertical component",
            "micro_lesson": {
                "explanation_latex": "When the angle is measured from the horizontal, $\\sin\\theta$ gives the vertical component and $\\cos\\theta$ gives the horizontal component.",
                "visual_overlay": "velocity_decomposition"
            }
        },
        {
            "id": "forgot_half",
            "description": "Forgetting the 1/2 in displacement equations",
            "micro_lesson": {
                "explanation_latex": "The kinematic equation $\\Delta y = v_{0y}t - \\frac{1}{2}gt^2$ has a factor of $\\frac{1}{2}$. Without it, you overestimate displacement.",
                "visual_overlay": null
            }
        }
    ]'::jsonb,

    -- answer_spec
    '{
        "parts": [
            { "label": "Maximum height", "unit": "m", "answer": 5.1, "tolerance": 0.3 },
            { "label": "Horizontal range", "unit": "m", "answer": 35.3, "tolerance": 0.5 }
        ]
    }'::jsonb
),

-- Second problem: Block on an inclined plane
(
    'Block on a Frictionless Ramp',
    'A $5\,\text{kg}$ block is placed on a frictionless inclined plane at $30°$. It is released from rest. Find the acceleration of the block down the incline.',
    1,
    'mechanics',
    'inclined_plane',

    -- scene_definition
    '{
        "gravity": [0.0, -9.81],
        "bodies": [
            {
                "id": "block",
                "label": "Block",
                "body_type": "dynamic",
                "shape": { "type": "rectangle", "width": 0.8, "height": 0.6 },
                "position": [-2.0, 3.5],
                "rotation": -0.5236,
                "velocity": [0.0, 0.0],
                "material": { "restitution": 0.0, "static_friction": 0.0, "kinetic_friction": 0.0 },
                "mass": 5.0,
                "fill_color": "#f59e0b",
                "stroke_color": "#92400e"
            },
            {
                "id": "ramp",
                "label": "",
                "body_type": "fixed",
                "shape": { "type": "polygon", "vertices": [[-5.0, 0.0], [5.0, 0.0], [-5.0, 5.77]] },
                "position": [0.0, 0.0],
                "rotation": 0.0,
                "velocity": [0.0, 0.0],
                "material": { "restitution": 0.0, "static_friction": 0.0, "kinetic_friction": 0.0 },
                "mass": 0.0,
                "fill_color": "#d1d5db",
                "stroke_color": "#6b7280"
            }
        ],
        "constraints": [],
        "boundaries": [
            {
                "id": "floor",
                "start": [-6.0, 0.0],
                "end": [8.0, 0.0],
                "material": { "restitution": 0.0, "static_friction": 0.0, "kinetic_friction": 0.0 }
            }
        ],
        "camera": [0.0, 2.0, 1.0],
        "pixels_per_metre": 40.0
    }'::jsonb,

    -- parameters
    '[
        { "key": "block.mass", "label": "Mass", "default": 5.0, "min": 1.0, "max": 20.0, "step": 0.5, "unit": "kg", "target": "block.mass" },
        { "key": "ramp.rotation", "label": "Ramp angle", "default": 30.0, "min": 10.0, "max": 60.0, "step": 1.0, "unit": "deg", "target": "ramp.rotation" }
    ]'::jsonb,

    -- challenge_stages
    '[
        {
            "id": "identify",
            "title": "Identify quantities",
            "prompt_text": "What quantities determine the acceleration of the block down the frictionless incline?",
            "hint_text": "On a frictionless surface, what forces act on the block?",
            "success_response": "Correct! Only the angle and gravity matter. Mass cancels out!",
            "stage_data": {
                "type": "identify_quantities",
                "correct": [
                    { "id": "angle", "label": "Incline angle", "symbol_latex": "\\theta" },
                    { "id": "gravity", "label": "Gravitational acceleration", "symbol_latex": "g" }
                ],
                "distractors": [
                    { "id": "mass", "label": "Mass of block", "symbol_latex": "m" },
                    { "id": "friction", "label": "Friction coefficient", "symbol_latex": "\\mu" }
                ],
                "explanations": {
                    "angle": "Yes — the angle determines what fraction of gravity acts along the incline.",
                    "gravity": "Yes — gravity is the driving force.",
                    "mass": "Mass appears in both the force (mg sin θ) and F=ma, so it cancels! The acceleration is independent of mass.",
                    "friction": "The problem states the surface is frictionless, so μ = 0."
                }
            }
        },
        {
            "id": "fbd",
            "title": "Free-body diagram",
            "prompt_text": "Draw the free-body diagram for the block on the incline. Include all forces.",
            "hint_text": "There are exactly two forces: gravity and the normal force from the surface.",
            "success_response": "Perfect! Gravity pulls straight down, and the normal force pushes perpendicular to the surface.",
            "stage_data": {
                "type": "freebody_diagram",
                "target_body": "block",
                "expected_forces": [
                    { "id": "gravity", "label": "Weight (mg)", "direction_deg": 270.0, "color": "#ef4444", "label_latex": "mg" },
                    { "id": "normal", "label": "Normal force (N)", "direction_deg": 120.0, "color": "#3b82f6", "label_latex": "N" }
                ],
                "direction_tolerance_deg": 20.0,
                "per_force_hints": {
                    "gravity": "Gravity always acts straight downward, regardless of the incline.",
                    "normal": "The normal force is always perpendicular to the contact surface."
                }
            }
        },
        {
            "id": "predict",
            "title": "Predict",
            "prompt_text": "Calculate the acceleration of the block down the incline. Use a = g sin(θ).",
            "hint_text": "g = 9.81 m/s², θ = 30°, sin(30°) = 0.5.",
            "success_response": "Let us see if the simulation agrees with your calculation.",
            "stage_data": {
                "type": "prediction",
                "question": "What is the acceleration of the block down the incline? (in m/s²)",
                "answer": 4.905,
                "unit": "m/s²",
                "tolerance_pct": 5.0,
                "sim_runs_after": true,
                "sim_end_time": 3.0
            }
        }
    ]'::jsonb,

    -- what_if_prompts
    '[
        {
            "question": "What happens to the acceleration if you double the mass of the block?",
            "parameter_key": "block.mass",
            "suggested_value": 10.0,
            "expected_insight": "The acceleration stays the same! On a frictionless incline, a = g sin(θ) — mass cancels out. A heavy block and a light block slide at the same rate."
        },
        {
            "question": "At what angle would the acceleration equal g (free fall)?",
            "parameter_key": "ramp.rotation",
            "suggested_value": 90.0,
            "expected_insight": "At 90°, sin(90°) = 1, so a = g. The incline becomes vertical — the block is in free fall!"
        }
    ]'::jsonb,

    -- common_errors
    '[]'::jsonb,

    -- answer_spec
    '{
        "parts": [
            { "label": "Acceleration", "unit": "m/s²", "answer": 4.905, "tolerance": 0.15 }
        ]
    }'::jsonb
);
