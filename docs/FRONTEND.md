# Frontend Documentation

Complete guide to the Locus frontend built with Leptos and WASM.

## Overview

**Framework:** Leptos 0.7 (Client-Side Rendering)
**Language:** Rust compiled to WebAssembly
**Build Tool:** Trunk
**Styling:** Tailwind CSS
**Math Rendering:** KaTeX

---

## Project Structure

```
crates/frontend/
├── Cargo.toml          # Dependencies
├── Trunk.toml          # Build configuration
├── index.html          # HTML template
└── src/
    ├── main.rs         # App entry, router, auth context
    ├── api.rs          # HTTP client, localStorage
    ├── grader.rs       # Client-side grading logic
    ├── symengine.rs    # SymEngine bindings (future)
    ├── components/     # Reusable UI components
    │   ├── mod.rs
    │   ├── navbar.rs
    │   ├── math_input.rs
    │   ├── problem_card.rs
    │   └── topic_selector.rs
    └── pages/          # Route pages
        ├── mod.rs
        ├── home.rs
        ├── practice.rs
        ├── ranked.rs
        ├── leaderboard.rs
        ├── login.rs
        └── register.rs
```

---

## Main Application

### main.rs

**File:** `crates/frontend/src/main.rs`

**Responsibilities:**
- App entry point
- Router configuration
- Global auth context
- Error handling

**Key Components:**

#### App Component
```rust
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let auth = create_rw_signal(AuthState::new());
    provide_context(auth);

    view! {
        <Router>
            <main class="min-h-screen bg-gray-50">
                <Navbar/>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/practice" view=Practice/>
                    <Route path="/ranked" view=Ranked/>
                    <Route path="/leaderboard" view=Leaderboard/>
                    <Route path="/login" view=Login/>
                    <Route path="/register" view=Register/>
                </Routes>
            </main>
        </Router>
    }
}
```

#### AuthState
```rust
#[derive(Clone, Debug)]
pub struct AuthState {
    pub token: Option<String>,
    pub username: Option<String>,
}
```

**Auth Functions:**
- `is_authenticated()` - Check if user is logged in
- `login()` - Store token and username
- `logout()` - Clear auth state
- `get_token()` - Retrieve current token

---

## API Client

### api.rs

**File:** `crates/frontend/src/api.rs`

**Responsibilities:**
- HTTP requests to backend
- JWT token management
- localStorage persistence
- Error handling

**Key Functions:**

#### register
```rust
pub async fn register(
    username: &str,
    email: &str,
    password: &str,
) -> Result<AuthResponse, String>
```

Sends POST to `/api/auth/register`.

#### login
```rust
pub async fn login(
    email: &str,
    password: &str,
) -> Result<AuthResponse, String>
```

Sends POST to `/api/auth/login`, stores token in localStorage.

#### get_current_user
```rust
pub async fn get_current_user(
    token: &str,
) -> Result<UserProfile, String>
```

Sends GET to `/api/user/me` with auth header.

#### get_problem
```rust
pub async fn get_problem(
    token: Option<&str>,
    query: &ProblemQuery,
) -> Result<ProblemResponse, String>
```

Sends GET to `/api/problem` with optional topic filters.

#### submit_answer
```rust
pub async fn submit_answer(
    token: &str,
    request: &SubmitRequest,
) -> Result<SubmitResponse, String>
```

Sends POST to `/api/submit` with user answer.

#### get_leaderboard
```rust
pub async fn get_leaderboard(
    topic: &str,
) -> Result<Vec<LeaderboardEntry>, String>
```

Sends GET to `/api/leaderboard?topic={topic}`.

**localStorage Keys:**
- `locus_auth_token` - JWT token
- `locus_username` - Username

---

## Client-Side Grading

### grader.rs

**File:** `crates/frontend/src/grader.rs`

**Responsibilities:**
- Input preprocessing
- Practice mode validation
- Answer normalization

**Key Functions:**

#### preprocess_input
```rust
pub fn preprocess_input(input: &str) -> String
```

Transforms user input for better usability:
- Inserts implicit multiplication: `2x` → `2*x`
- Handles parentheses: `2(x+1)` → `2*(x+1)`
- Preserves fractions: `x/2` → `x/2`
- Handles negative signs: `-x` → `-x`

**Examples:**
```rust
preprocess_input("2x") // → "2*x"
preprocess_input("3(x+1)") // → "3*(x+1)"
preprocess_input("x^2") // → "x^2"
```

**File location:** `crates/frontend/src/grader.rs:23`

#### normalize_answer
```rust
pub fn normalize_answer(input: &str) -> String
```

Normalizes answer for comparison:
- Trim whitespace
- Convert to lowercase
- Remove extra spaces

**File location:** `crates/frontend/src/grader.rs:82`

#### grade_answer
```rust
pub fn grade_answer(user_input: &str, answer_key: &str) -> bool
```

Simple string comparison for practice mode.

**Future:** Will use SymEngine for symbolic comparison.

---

## Pages

### Home Page

**File:** `crates/frontend/src/pages/home.rs`

**Route:** `/`

**Features:**
- Landing page
- Project description
- Links to practice/ranked modes
- Getting started guide

**Layout:**
- Hero section
- Feature cards
- CTA buttons

---

### Practice Page

**File:** `crates/frontend/src/pages/practice.rs`

**Route:** `/practice`

**Features:**
- Topic selection (TopicSelector component)
- Problem display with answer
- Client-side instant grading
- No ELO changes
- No authentication required

**State:**
```rust
let problem = create_rw_signal::<Option<ProblemResponse>>(None);
let user_input = create_rw_signal(String::new());
let is_correct = create_rw_signal::<Option<bool>>(None);
let selected_topic = create_rw_signal::<Option<MainTopic>>(None);
let selected_subtopics = create_rw_signal::<Vec<String>>(vec![]);
```

**Flow:**
1. User selects topic and subtopics
2. Click "Get Problem" → fetch from API with answer
3. User enters answer
4. Click "Check Answer" → client-side grading
5. Instant feedback (no server submission)

**File location:** `crates/frontend/src/pages/practice.rs`

---

### Ranked Page

**File:** `crates/frontend/src/pages/ranked.rs`

**Route:** `/ranked`

**Features:**
- Requires authentication
- Topic selection
- Problem display WITHOUT answer
- Server-side grading
- ELO rating changes
- Submission history

**Protected Route:**
```rust
let auth = use_context::<RwSignal<AuthState>>();
if !auth.is_authenticated() {
    return view! { <Navigate path="/login"/> };
}
```

**State:**
```rust
let problem = create_rw_signal::<Option<ProblemResponse>>(None);
let user_input = create_rw_signal(String::new());
let result = create_rw_signal::<Option<SubmitResponse>>(None);
let selected_topic = create_rw_signal::<Option<MainTopic>>(None);
let selected_subtopics = create_rw_signal::<Vec<String>>(vec![]);
```

**Flow:**
1. User selects topic and subtopics
2. Click "Get Problem" → fetch from API (no answer)
3. User enters answer
4. Click "Submit" → send to server
5. Server grades and updates ELO
6. Display result with ELO change

**File location:** `crates/frontend/src/pages/ranked.rs`

---

### Leaderboard Page

**File:** `crates/frontend/src/pages/leaderboard.rs`

**Route:** `/leaderboard`

**Features:**
- Topic dropdown selector
- Top 100 users by ELO
- Rank, username, ELO display
- Real-time data (no caching currently)

**State:**
```rust
let selected_topic = create_rw_signal(MainTopic::Calculus);
let leaderboard = create_resource(
    move || selected_topic.get(),
    |topic| async move { get_leaderboard(&topic.to_string()).await }
);
```

**Display:**
- Table format
- Highlighted current user (if logged in)
- Responsive design

**File location:** `crates/frontend/src/pages/leaderboard.rs`

---

### Login Page

**File:** `crates/frontend/src/pages/login.rs`

**Route:** `/login`

**Features:**
- Email and password form
- Error handling
- Redirect to home on success
- Link to registration

**State:**
```rust
let email = create_rw_signal(String::new());
let password = create_rw_signal(String::new());
let error = create_rw_signal::<Option<String>>(None);
```

**Submit Handler:**
```rust
let on_submit = move || {
    spawn_local(async move {
        match login(&email.get(), &password.get()).await {
            Ok(response) => {
                auth.login(response.token, response.username);
                navigate("/");
            }
            Err(e) => error.set(Some(e)),
        }
    });
};
```

**File location:** `crates/frontend/src/pages/login.rs`

---

### Register Page

**File:** `crates/frontend/src/pages/register.rs`

**Route:** `/register`

**Features:**
- Username, email, password form
- Client-side validation
- Error handling
- Auto-login on success

**Validation:**
- Username: 3-50 characters
- Email: Valid format
- Password: Minimum 8 characters

**File location:** `crates/frontend/src/pages/register.rs`

---

## Components

### Navbar

**File:** `crates/frontend/src/components/navbar.rs`

**Features:**
- Logo and site title
- Navigation links (Home, Practice, Ranked, Leaderboard)
- Auth state display
- Login/Logout buttons

**Conditional Rendering:**
```rust
{move || {
    if auth.is_authenticated() {
        view! {
            <span>Welcome, {auth.username}</span>
            <button on:click=logout>Logout</button>
        }
    } else {
        view! {
            <a href="/login">Login</a>
            <a href="/register">Register</a>
        }
    }
}}
```

---

### MathInput

**File:** `crates/frontend/src/components/math_input.rs`

**Features:**
- Input field for math expressions
- Real-time preprocessing preview
- Keyboard shortcuts support
- LaTeX rendering of preview

**Props:**
```rust
#[component]
pub fn MathInput(
    value: RwSignal<String>,
    on_submit: Option<Callback<()>>,
) -> impl IntoView
```

**Preview Display:**
Shows preprocessed version of input:
- Input: `2x + 3(y+1)`
- Preview: `2*x + 3*(y+1)`

---

### ProblemCard

**File:** `crates/frontend/src/components/problem_card.rs`

**Features:**
- Displays problem question
- Shows difficulty and topic
- Optional answer display (practice mode only)
- KaTeX rendering

**Props:**
```rust
#[component]
pub fn ProblemCard(
    problem: ProblemResponse,
    show_answer: bool,
) -> impl IntoView
```

**LaTeX Rendering:**
Uses KaTeX to render math in question.

---

### TopicSelector

**File:** `crates/frontend/src/components/topic_selector.rs`

**Features:**
- Two-step selection: topic → subtopics
- Checkbox for each subtopic
- Dynamic subtopic list based on topic
- "Select All" / "Deselect All" buttons

**Props:**
```rust
#[component]
pub fn TopicSelector(
    selected_topic: RwSignal<Option<MainTopic>>,
    selected_subtopics: RwSignal<Vec<String>>,
) -> impl IntoView
```

**Flow:**
1. User selects main topic from dropdown
2. Subtopic checkboxes appear
3. User selects desired subtopics
4. Parent component uses selections for API query

**File location:** `crates/frontend/src/components/topic_selector.rs`

---

## Styling

### Tailwind CSS

**Configuration:** Applied via CDN in `index.html`

**Common Classes:**
- `bg-gray-50` - Background color
- `text-blue-600` - Link colors
- `rounded-lg` - Rounded corners
- `shadow-md` - Drop shadows
- `hover:bg-blue-700` - Hover states

**Responsive Design:**
- Mobile-first approach
- `sm:`, `md:`, `lg:` breakpoints
- Flexbox and Grid layouts

---

## KaTeX Integration

### Configuration

**File:** `crates/frontend/index.html`

**CDN Links:**
```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js"></script>
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js"></script>
```

### Auto-Rendering

**JavaScript:**
```javascript
setInterval(() => {
    renderMathInElement(document.body, {
        delimiters: [
            {left: "$$", right: "$$", display: true},
            {left: "$", right: "$", display: false}
        ],
        throwOnError: false
    });
}, 500);
```

**Renders every 500ms** to catch dynamically added content.

**LaTeX Syntax:**
- Inline: `$x^2 + y^2$`
- Display: `$$\int_{0}^{1} x^2 dx$$`

---

## Build Configuration

### Trunk.toml

**File:** `crates/frontend/Trunk.toml`

```toml
[build]
target = "index.html"
dist = "dist"

[watch]
ignore = ["dist"]

[serve]
address = "127.0.0.1"
port = 8080
open = false

# Proxy API requests to backend
[[proxy]]
backend = "http://127.0.0.1:3000/api"
```

**Proxy Configuration:**
- Frontend: `http://localhost:8080`
- API requests: `http://localhost:8080/api/*`
- Proxied to: `http://localhost:3000/api/*`

---

## Development Workflow

### Running Locally

```bash
cd crates/frontend
trunk serve
```

**Access:** http://localhost:8080

**Hot Reload:**
- Watches for file changes
- Automatically rebuilds and reloads
- Preserves localStorage state

### Building for Production

```bash
cd crates/frontend
trunk build --release
```

**Output:** `crates/frontend/dist/`

**Contains:**
- `index.html` - Entry point
- WASM binary
- JavaScript glue code
- CSS bundles

---

## State Management

### Signals

Leptos uses reactive signals for state:

```rust
// Read-write signal
let count = create_rw_signal(0);
count.set(count.get() + 1);

// Read-only derived signal
let doubled = create_memo(move |_| count.get() * 2);
```

### Resources

For async data fetching:

```rust
let user = create_resource(
    || (),
    |_| async move {
        get_current_user(&token).await
    }
);

// In view
{move || {
    user.get().map(|data| {
        view! { <div>{data.username}</div> }
    })
}}
```

### Context

For global state (like auth):

```rust
// Provide
let auth = create_rw_signal(AuthState::new());
provide_context(auth);

// Consume
let auth = use_context::<RwSignal<AuthState>>();
```

---

## Routing

### Router Configuration

```rust
<Router>
    <Routes>
        <Route path="/" view=Home/>
        <Route path="/practice" view=Practice/>
        <Route path="/ranked" view=Ranked/>
        <Route path="/leaderboard" view=Leaderboard/>
        <Route path="/login" view=Login/>
        <Route path="/register" view=Register/>
    </Routes>
</Router>
```

### Navigation

```rust
// Declarative
<a href="/practice">Practice</a>

// Programmatic
let navigate = use_navigate();
navigate("/ranked");

// With redirect
<Navigate path="/login"/>
```

---

## Error Handling

### API Errors

```rust
match get_problem(token, &query).await {
    Ok(problem) => problem_signal.set(Some(problem)),
    Err(e) => error_signal.set(Some(format!("Error: {}", e))),
}
```

### Display Errors

```rust
{move || {
    error.get().map(|err| {
        view! {
            <div class="bg-red-100 text-red-700">
                {err}
            </div>
        }
    })
}}
```

---

## Performance Optimization

### WASM Binary Size

- Release build optimizations
- `wasm-opt` for further compression
- Code splitting (future)

### Rendering Optimization

- Reactive signals minimize re-renders
- Memoization for expensive computations
- Virtual DOM diffing

### Caching

- localStorage for auth state
- No API response caching currently
- Future: Service worker for offline support

---

## Testing

### Unit Tests

```bash
cargo test -p locus-frontend
```

**Test Files:**
- `crates/frontend/src/grader.rs` - Input preprocessing tests

**Example Test:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_implicit_multiplication() {
        assert_eq!(preprocess_input("2x"), "2*x");
        assert_eq!(preprocess_input("3(x+1)"), "3*(x+1)");
    }
}
```

### Future Testing

- Component testing with `leptos_testing`
- E2E tests with Playwright/Selenium
- Visual regression testing

---

## Accessibility

### Current State

- Semantic HTML elements
- Form labels
- Keyboard navigation

### Future Improvements

- ARIA labels
- Screen reader support
- High contrast mode
- Keyboard shortcuts documentation

---

## Browser Support

**Minimum Requirements:**
- WebAssembly support
- ES6 JavaScript
- LocalStorage
- Fetch API

**Tested Browsers:**
- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

---

## Future Enhancements

- **SymEngine Integration:** Symbolic math grading
- **Real-time Competitions:** WebSocket support
- **Problem Hints:** Progressive hint system
- **Statistics Dashboard:** Personal analytics
- **Dark Mode:** Theme toggle
- **Mobile App:** PWA or native
- **Social Features:** Friends, challenges
- **Achievement System:** Badges and milestones
