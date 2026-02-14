# Leptos Frontend Patterns

Guide to state management and component patterns used in the Locus frontend.

## Overview

Locus uses **Leptos** (CSR mode) for reactive frontend development with:
- **Signals** - Fine-grained reactive state
- **Context** - Global state sharing between components
- **Resources** - Async data fetching
- **Effects** - Side effects from signal changes
- **Callbacks** - Event handlers

**Version:** Leptos 0.7+ (Edition 2024)

## State Management Patterns

### 1. Local Component State with Signals

**Pattern:** Use `signal()` for component-local reactive state.

**Example from Practice page:**
```rust
#[component]
pub fn Practice() -> impl IntoView {
    // Topic selection state
    let (selected_topic, set_selected_topic) = signal(None::<String>);
    let (selected_subtopics, set_selected_subtopics) = signal(Vec::<String>::new());

    // Problem state
    let (problem, set_problem) = signal(None::<ProblemResponse>);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    // Answer state
    let (answer, set_answer) = signal(String::new());
    let (result, set_result) = signal(None::<GradeResult>);
    let (show_answer, set_show_answer) = signal(false);

    // ... component logic
}
```

**When to use:**
- State only needed within single component
- UI state (loading, error, input values)
- Derived state from props

**Signal API:**
```rust
// Create signal
let (read, write) = signal(initial_value);

// Read value
let value = read.get();

// Update value
write.set(new_value);
write.update(|v| *v += 1); // Modify in place

// Use in closures
move || {
    let current = read.get();
    // ...
}
```

### 2. Global State with Context

**Pattern:** Share state across component tree using `provide_context()` and `expect_context()`.

**Example from main.rs:**
```rust
#[derive(Clone, Copy)]
pub struct AuthContext {
    pub is_logged_in: ReadSignal<bool>,
    pub set_logged_in: WriteSignal<bool>,
    pub username: ReadSignal<Option<String>>,
    pub set_username: WriteSignal<Option<String>>,
}

#[component]
fn App() -> impl IntoView {
    // Create global auth state
    let (is_logged_in, set_logged_in) = signal(api::is_logged_in());
    let (username, set_username) = signal(api::get_stored_username());

    // Provide to all children
    provide_context(AuthContext {
        is_logged_in,
        set_logged_in,
        username,
        set_username,
    });

    view! {
        <Router>
            // ... child components can access AuthContext
        </Router>
    }
}

// In child component
#[component]
fn Navbar() -> impl IntoView {
    let auth = expect_context::<AuthContext>();

    view! {
        <nav>
            {move || if auth.is_logged_in.get() {
                view! { <LogoutButton /> }
            } else {
                view! { <LoginButton /> }
            }}
        </nav>
    }
}
```

**When to use:**
- Authentication state
- Theme/dark mode
- User preferences
- Any state needed by many components

**Context API:**
```rust
// Provide context
provide_context(MyContext { /* fields */ });

// Consume context
let ctx = expect_context::<MyContext>();

// Optional context
let ctx = use_context::<MyContext>(); // Returns Option<MyContext>
```

### 3. Async Operations with spawn_local

**Pattern:** Use `spawn_local()` for async API calls and side effects.

**Example from Practice page:**
```rust
let load_problem = move || {
    set_loading.set(true);
    set_error.set(None);

    let topic = selected_topic.get();
    let subtopics = selected_subtopics.get();

    spawn_local(async move {
        match api::get_problem(true, topic.as_deref(), Some(&subtopics)).await {
            Ok(p) => {
                set_problem.set(Some(p));
                set_loading.set(false);
            }
            Err(e) => {
                set_error.set(Some(e.message));
                set_loading.set(false);
            }
        }
    });
};
```

**When to use:**
- API calls
- Async computations
- Timers and delays
- Any Future that needs to run

**Best practices:**
```rust
// Capture signals before async block
let topic = selected_topic.get(); // Clone/copy value
let set_problem = set_problem;     // Clone signal setter

spawn_local(async move {
    // Use captured values in async block
    let result = api::get_problem(topic).await;
    set_problem.set(result);
});
```

### 4. Event Handlers with Callbacks

**Pattern:** Use `Callback::new()` for event handlers that accept parameters.

**Example from Practice page:**
```rust
let on_topic_confirm = Callback::new(move |(topic, subtopics): (String, Vec<String>)| {
    set_selected_topic.set(Some(topic));
    set_selected_subtopics.set(subtopics);
    load_problem();
});

let on_submit = Callback::new(move |_| {
    if let Some(p) = problem.get() {
        let user_input = preprocess_input(&answer.get());
        if let Some(answer_key) = &p.answer_key {
            let grade = check_answer(&user_input, answer_key, p.grading_mode);
            set_result.set(Some(grade));
        }
    }
});
```

**When to use:**
- Passing event handlers to child components
- Handlers that accept specific event types
- Reusable callback logic

**Callback vs. Closure:**
```rust
// Simple closure (inline)
view! {
    <button on:click=move |_| set_count.update(|c| *c += 1)>
        "Increment"
    </button>
}

// Callback (can be passed as prop)
let on_click = Callback::new(move |_| set_count.update(|c| *c += 1));

view! {
    <MyButton on_click=on_click />
}
```

### 5. Derived State with Memos

**Pattern:** Use `memo()` for expensive computations that depend on signals.

**Example:**
```rust
#[component]
pub fn Leaderboard() -> impl IntoView {
    let (users, set_users) = signal(Vec::<User>::new());
    let (filter, set_filter) = signal(String::new());

    // Memo: recomputes only when users or filter changes
    let filtered_users = memo(move || {
        let f = filter.get().to_lowercase();
        users.get()
            .into_iter()
            .filter(|u| u.username.to_lowercase().contains(&f))
            .collect::<Vec<_>>()
    });

    view! {
        <input
            on:input=move |ev| set_filter.set(event_target_value(&ev))
            placeholder="Filter users..."
        />
        <For
            each=move || filtered_users.get()
            key=|u| u.id
            children=|u| view! { <UserRow user=u /> }
        />
    }
}
```

**When to use:**
- Filtering or sorting lists
- Complex calculations from multiple signals
- Avoiding redundant computations

**Memo vs. Signal:**
- **Signal:** Explicit updates with `set()`
- **Memo:** Automatic updates when dependencies change

### 6. Effects for Side Effects

**Pattern:** Use `Effect::new()` to run code when signals change.

**Example:**
```rust
#[component]
pub fn AutoSave() -> impl IntoView {
    let (content, set_content) = signal(String::new());

    // Auto-save to localStorage when content changes
    Effect::new(move || {
        let c = content.get();
        if !c.is_empty() {
            gloo_storage::LocalStorage::set("draft", c).ok();
        }
    });

    view! {
        <textarea
            on:input=move |ev| set_content.set(event_target_value(&ev))
            prop:value=move || content.get()
        />
    }
}
```

**When to use:**
- localStorage sync
- Logging/analytics
- DOM manipulation outside Leptos
- WebSocket subscriptions

**Effect best practices:**
```rust
// Good: Track specific signals
Effect::new(move || {
    let value = my_signal.get();
    console_log!("Value changed: {}", value);
});

// Avoid: Infinite loops
Effect::new(move || {
    let value = my_signal.get();
    set_my_signal.set(value + 1); // INFINITE LOOP!
});
```

## Component Patterns

### 1. Component Props

**Pattern:** Define props as struct fields with `#[component]`.

**Example:**
```rust
#[component]
pub fn UserCard(
    username: String,
    elo: i32,
    #[prop(optional)] avatar: Option<String>,
    #[prop(default = false)] show_stats: bool,
) -> impl IntoView {
    view! {
        <div class="card">
            <h3>{username}</h3>
            <p>"ELO: "{elo}</p>
            {avatar.map(|url| view! { <img src=url /> })}
            {show_stats.then(|| view! { <StatsPanel /> })}
        </div>
    }
}

// Usage
view! {
    <UserCard username="alice".to_string() elo=1600 />
    <UserCard
        username="bob".to_string()
        elo=1500
        avatar=Some("url".to_string())
        show_stats=true
    />
}
```

**Prop attributes:**
- `#[prop(optional)]` - Makes prop optional (must be Option<T>)
- `#[prop(default = value)]` - Provides default value
- `#[prop(into)]` - Auto-converts with `.into()` (e.g., &str to String)

### 2. Children Components

**Pattern:** Accept children with `children: Children`.

**Example:**
```rust
#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="card">
            {children()}
        </div>
    }
}

// Usage
view! {
    <Card>
        <h2>"Title"</h2>
        <p>"Content"</p>
    </Card>
}
```

### 3. Conditional Rendering

**Pattern:** Use Option, bool.then(), and match for conditional views.

**Examples:**
```rust
// Option
view! {
    {error.get().map(|e| view! { <ErrorBanner message=e /> })}
}

// Boolean with .then()
view! {
    {is_loading.get().then(|| view! { <Spinner /> })}
}

// Boolean with if
view! {
    {if is_logged_in.get() {
        view! { <Dashboard /> }
    } else {
        view! { <Login /> }
    }}
}

// Match
view! {
    {match state.get() {
        State::Loading => view! { <Spinner /> },
        State::Error(e) => view! { <ErrorBanner message=e /> },
        State::Success(data) => view! { <DataView data=data /> },
    }}
}
```

### 4. Lists with For Component

**Pattern:** Use `<For>` for efficient list rendering with keys.

**Example:**
```rust
#[component]
pub fn UserList() -> impl IntoView {
    let (users, set_users) = signal(vec![
        User { id: 1, name: "Alice".into() },
        User { id: 2, name: "Bob".into() },
    ]);

    view! {
        <For
            each=move || users.get()
            key=|user| user.id
            children=|user| view! {
                <li>{user.name}</li>
            }
        />
    }
}
```

**Key points:**
- `each` - Function returning collection
- `key` - Unique identifier for each item (enables efficient updates)
- `children` - Function to render each item

## Router Patterns

### 1. Route Definition

**Pattern:** Define routes in App component with `<Router>` and `<Routes>`.

**Example from main.rs:**
```rust
view! {
    <Router>
        <main>
            <Routes fallback=|| view! { <p>"Page not found"</p> }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/practice") view=Practice />
                <Route path=path!("/ranked") view=Ranked />
                <Route path=path!("/login") view=Login />
            </Routes>
        </main>
    </Router>
}
```

### 2. Navigation

**Pattern:** Use `<A>` component for client-side navigation.

**Example:**
```rust
use leptos_router::components::A;

view! {
    <A href="/practice">"Go to Practice"</A>
}
```

**Programmatic navigation:**
```rust
use leptos_router::hooks::use_navigate;

let navigate = use_navigate();

let on_submit = move || {
    // ... logic
    navigate("/dashboard", Default::default());
};
```

### 3. Route Parameters

**Pattern:** Extract route params with `use_params()`.

**Example:**
```rust
use leptos_router::hooks::use_params;

#[component]
pub fn ProblemDetail() -> impl IntoView {
    let params = use_params::<ProblemParams>();

    let id = move || {
        params.get()
            .ok()
            .and_then(|p| p.id.parse::<u32>().ok())
    };

    view! {
        <div>
            <h1>"Problem "{move || id().unwrap_or(0)}</h1>
        </div>
    }
}

#[derive(Params, PartialEq)]
struct ProblemParams {
    id: String,
}

// Route definition
<Route path=path!("/problem/:id") view=ProblemDetail />
```

## Common Patterns

### Form Handling

```rust
#[component]
pub fn LoginForm() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (loading, set_loading) = signal(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        set_loading.set(true);
        set_error.set(None);

        let email_val = email.get();
        let password_val = password.get();

        spawn_local(async move {
            match api::login(&email_val, &password_val).await {
                Ok(token) => {
                    api::store_token(&token);
                    navigate("/dashboard", Default::default());
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    };

    view! {
        <form on:submit=on_submit>
            <input
                type="email"
                on:input=move |ev| set_email.set(event_target_value(&ev))
                prop:value=move || email.get()
            />
            <input
                type="password"
                on:input=move |ev| set_password.set(event_target_value(&ev))
                prop:value=move || password.get()
            />
            <button type="submit" disabled=move || loading.get()>
                {move || if loading.get() { "Loading..." } else { "Login" }}
            </button>
            {error.get().map(|e| view! { <p class="error">{e}</p> })}
        </form>
    }
}
```

### Modal/Dialog Pattern

```rust
#[component]
pub fn ConfirmDialog(
    show: ReadSignal<bool>,
    on_confirm: Callback<()>,
    on_cancel: Callback<()>,
    children: Children,
) -> impl IntoView {
    view! {
        {move || show.get().then(|| view! {
            <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
                <div class="bg-white p-6 rounded">
                    {children()}
                    <div class="flex gap-2 mt-4">
                        <button on:click=move |_| on_confirm.call(())>
                            "Confirm"
                        </button>
                        <button on:click=move |_| on_cancel.call(())>
                            "Cancel"
                        </button>
                    </div>
                </div>
            </div>
        })}
    }
}

// Usage
let (show_dialog, set_show_dialog) = signal(false);

let on_confirm = Callback::new(move |_| {
    // ... confirm logic
    set_show_dialog.set(false);
});

let on_cancel = Callback::new(move |_| {
    set_show_dialog.set(false);
});

view! {
    <button on:click=move |_| set_show_dialog.set(true)>
        "Delete"
    </button>
    <ConfirmDialog show=show_dialog on_confirm=on_confirm on_cancel=on_cancel>
        <p>"Are you sure you want to delete?"</p>
    </ConfirmDialog>
}
```

### Loading States

```rust
#[derive(Clone)]
enum LoadingState<T> {
    Idle,
    Loading,
    Success(T),
    Error(String),
}

#[component]
pub fn DataFetcher() -> impl IntoView {
    let (state, set_state) = signal(LoadingState::Idle);

    let fetch_data = move || {
        set_state.set(LoadingState::Loading);

        spawn_local(async move {
            match api::fetch_data().await {
                Ok(data) => set_state.set(LoadingState::Success(data)),
                Err(e) => set_state.set(LoadingState::Error(e.message)),
            }
        });
    };

    view! {
        <div>
            {move || match state.get() {
                LoadingState::Idle => view! {
                    <button on:click=move |_| fetch_data()>"Load Data"</button>
                }.into_any(),
                LoadingState::Loading => view! {
                    <Spinner />
                }.into_any(),
                LoadingState::Success(data) => view! {
                    <DataView data=data />
                }.into_any(),
                LoadingState::Error(e) => view! {
                    <ErrorBanner message=e />
                    <button on:click=move |_| fetch_data()>"Retry"</button>
                }.into_any(),
            }}
        </div>
    }
}
```

## Performance Tips

### 1. Minimize Signal Reads

```rust
// Bad: Multiple reads
view! {
    <div>
        <p>{expensive_signal.get().len()}</p>
        <p>{expensive_signal.get().first()}</p>
        <p>{expensive_signal.get().last()}</p>
    </div>
}

// Good: Single read
view! {
    {move || {
        let data = expensive_signal.get();
        view! {
            <div>
                <p>{data.len()}</p>
                <p>{data.first()}</p>
                <p>{data.last()}</p>
            </div>
        }
    }}
}
```

### 2. Use Memos for Expensive Computations

```rust
// Bad: Recomputes on every render
view! {
    <For
        each=move || users.get().into_iter().filter(|u| u.active).collect::<Vec<_>>()
        key=|u| u.id
        children=|u| view! { <UserCard user=u /> }
    />
}

// Good: Memo caches result
let active_users = memo(move || {
    users.get().into_iter().filter(|u| u.active).collect::<Vec<_>>()
});

view! {
    <For
        each=move || active_users.get()
        key=|u| u.id
        children=|u| view! { <UserCard user=u /> }
    />
}
```

### 3. Proper Keys in For Loops

```rust
// Bad: Index as key (breaks on reorder)
<For
    each=move || items.get()
    key=|item| item.index
    children=|item| view! { <Item data=item /> }
/>

// Good: Stable ID as key
<For
    each=move || items.get()
    key=|item| item.id
    children=|item| view! { <Item data=item /> }
/>
```

## Common Pitfalls

### 1. Capturing Signals in Async Blocks

**Problem:** Signal not updating from async block.

```rust
// Wrong: Captures read signal, not write signal
let (count, set_count) = signal(0);

spawn_local(async move {
    let data = fetch_data().await;
    count.set(data.len()); // ERROR: count is ReadSignal
});

// Correct: Capture write signal
let (count, set_count) = signal(0);

spawn_local(async move {
    let data = fetch_data().await;
    set_count.set(data.len());
});
```

### 2. Effect Infinite Loops

**Problem:** Effect updates the signal it reads.

```rust
// Wrong: Infinite loop
let (count, set_count) = signal(0);

Effect::new(move || {
    let c = count.get();
    set_count.set(c + 1); // INFINITE LOOP!
});

// Correct: Update different signal
let (count, set_count) = signal(0);
let (doubled, set_doubled) = signal(0);

Effect::new(move || {
    let c = count.get();
    set_doubled.set(c * 2); // OK
});
```

### 3. Forgetting .get() on Signals

**Problem:** Signal not reactive.

```rust
// Wrong: Not reactive
let message = if is_error { "Error!" } else { "OK" };

// Correct: Reactive closure
let message = move || if is_error.get() { "Error!" } else { "OK" };

view! {
    <p>{message}</p>
}
```

## Testing Patterns

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_updates() {
        let (count, set_count) = signal(0);

        assert_eq!(count.get(), 0);

        set_count.set(5);
        assert_eq!(count.get(), 5);

        set_count.update(|c| *c += 1);
        assert_eq!(count.get(), 6);
    }
}
```

## Resources

- [Leptos Book](https://leptos-rs.github.io/leptos/)
- [Leptos Examples](https://github.com/leptos-rs/leptos/tree/main/examples)
- [Leptos Discord](https://discord.gg/leptos)
- [Fine-Grained Reactivity](https://leptos-rs.github.io/leptos/reactivity/)
