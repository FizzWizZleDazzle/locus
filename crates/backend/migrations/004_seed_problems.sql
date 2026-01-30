-- Seed initial problems for testing
-- Now using main_topic and subtopic structure

-- Algebra 1 problems (factoring and polynomials)
INSERT INTO problems (question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode) VALUES
('Simplify: $2x + 3x$', '5*x', 1200, 'algebra1', 'polynomials', 'equivalent'),
('Simplify: $x^2 + 2x + x^2$', '2*x^2+2*x', 1300, 'algebra1', 'polynomials', 'equivalent'),
('Expand: $(x + 1)(x + 2)$', 'x^2+3*x+2', 1400, 'algebra1', 'polynomials', 'equivalent'),
('Expand: $(x - 3)^2$', 'x^2-6*x+9', 1450, 'algebra1', 'polynomials', 'equivalent'),
('Factor: $x^2 - 4$', '(x-2)*(x+2)', 1500, 'algebra1', 'factoring', 'factor'),
('Factor: $x^2 + 5x + 6$', '(x+2)*(x+3)', 1550, 'algebra1', 'factoring', 'factor'),
('Factor: $x^2 - 7x + 12$', '(x-3)*(x-4)', 1600, 'algebra1', 'factoring', 'factor'),
('Factor: $2x^2 + 7x + 3$', '(2*x+1)*(x+3)', 1700, 'algebra1', 'factoring', 'factor'),
('Simplify: $\frac{x^2 - 9}{x + 3}$', 'x-3', 1500, 'algebra1', 'polynomials', 'equivalent'),
('Expand: $(x + y)^2$', 'x^2+2*x*y+y^2', 1400, 'algebra1', 'polynomials', 'equivalent'),
('Simplify: $(x^3)^2$', 'x^6', 1300, 'algebra1', 'exponents', 'equivalent'),
('Expand: $(a - b)(a + b)$', 'a^2-b^2', 1350, 'algebra1', 'polynomials', 'equivalent'),
('Simplify: $\frac{x^2 - 1}{x - 1}$', 'x+1', 1500, 'algebra1', 'polynomials', 'equivalent');

-- Calculus problems (derivatives)
INSERT INTO problems (question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode) VALUES
('Find $\frac{d}{dx}[x^2]$', '2*x', 1400, 'calculus', 'derivatives', 'equivalent'),
('Find $\frac{d}{dx}[x^3]$', '3*x^2', 1450, 'calculus', 'derivatives', 'equivalent'),
('Find $\frac{d}{dx}[5x^2 + 3x]$', '10*x+3', 1500, 'calculus', 'derivatives', 'equivalent'),
('Find $\frac{d}{dx}[x^4 - 2x^2 + 1]$', '4*x^3-4*x', 1550, 'calculus', 'derivatives', 'equivalent'),
('Find $\frac{d}{dx}[\sin(x)]$', 'cos(x)', 1500, 'calculus', 'derivatives', 'equivalent'),
('Find $\frac{d}{dx}[\cos(x)]$', '-sin(x)', 1500, 'calculus', 'derivatives', 'equivalent'),
('Find $\frac{d}{dx}[e^x]$', 'exp(x)', 1450, 'calculus', 'derivatives', 'equivalent'),
('Find $\frac{d}{dx}[\ln(x)]$', '1/x', 1550, 'calculus', 'derivatives', 'equivalent'),
('Find $\frac{d}{dx}[x \cdot e^x]$', 'exp(x)+x*exp(x)', 1650, 'calculus', 'derivatives', 'equivalent'),
('Find $\frac{d}{dx}[\sin(x^2)]$', '2*x*cos(x^2)', 1700, 'calculus', 'derivatives', 'equivalent'),
('Find $\frac{d}{dx}[\tan(x)]$', 'sec(x)^2', 1600, 'calculus', 'derivatives', 'equivalent'),
('Find $\frac{d}{dx}[\ln(x^2)]$', '2/x', 1550, 'calculus', 'derivatives', 'equivalent'),
('Find $\frac{d}{dx}[e^{2x}]$', '2*exp(2*x)', 1600, 'calculus', 'derivatives', 'equivalent'),
('Find $\frac{d}{dx}[\sqrt{x}]$', '1/(2*sqrt(x))', 1500, 'calculus', 'derivatives', 'equivalent'),
('Find $\frac{d}{dx}[x^x]$ (hint: use logarithmic differentiation)', 'x^x*(ln(x)+1)', 1800, 'calculus', 'derivatives', 'equivalent');

-- Linear algebra problems
INSERT INTO problems (question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode) VALUES
('If $A = \begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}$, find $\det(A)$', '-2', 1500, 'linear_algebra', 'determinants', 'equivalent'),
('If $A = \begin{pmatrix} 2 & 0 \\ 0 & 3 \end{pmatrix}$, find $\det(A)$', '6', 1400, 'linear_algebra', 'determinants', 'equivalent'),
('Find the trace of $\begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}$', '5', 1350, 'linear_algebra', 'matrix_operations', 'equivalent'),
('If $\vec{u} = (1, 2)$ and $\vec{v} = (3, 4)$, find $\vec{u} \cdot \vec{v}$', '11', 1400, 'linear_algebra', 'vector_spaces', 'equivalent'),
('Find $|\vec{v}|$ where $\vec{v} = (3, 4)$', '5', 1450, 'linear_algebra', 'vector_spaces', 'equivalent');

-- Algebra 2 problems (radicals and logarithms)
INSERT INTO problems (question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode) VALUES
('Simplify: $\sqrt{x^4}$', 'x^2', 1400, 'algebra2', 'radical_expressions', 'equivalent'),
('Simplify: $\log_2(8)$', '3', 1300, 'algebra2', 'logarithms', 'equivalent'),
('Simplify: $\log_3(27)$', '3', 1300, 'algebra2', 'logarithms', 'equivalent'),
('Simplify: $\ln(e^5)$', '5', 1250, 'algebra2', 'logarithms', 'equivalent');
