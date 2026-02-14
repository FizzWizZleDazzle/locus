# Documentation Contribution Guidelines

## Single Source of Truth Principle

Each piece of information should have ONE authoritative location. Always link instead of repeating.

## Content Ownership Table

| Topic | File | What Goes Here |
|-------|------|----------------|
| System architecture | ARCHITECTURE.md | Diagrams, data flow, high-level component overview |
| API endpoints | API.md | Complete endpoint reference with request/response examples |
| Backend implementation | BACKEND.md | Code, algorithms, handler logic, server-side details |
| Frontend implementation | FRONTEND.md | Components, pages, client logic, UI patterns |
| Database | DATABASE.md | Schema, migrations, indexes, queries |
| Testing | TESTING.md | Test strategy, examples, running tests |
| SymEngine FFI | SYMENGINE_FFI.md | Safety rules, usage patterns, WASM vs native |
| Authentication | AUTHENTICATION.md | Auth flows, OAuth, email verification, security |
| Grading system | GRADING.md | Two-stage equivalence, modes, MathJSON conversion |
| Development setup | DEVELOPMENT.md | Local setup, workflow, tools, debugging |
| Deployment | DEPLOYMENT.md | Production deployment, operations, monitoring |
| Leptos patterns | LEPTOS_PATTERNS.md | Frontend state management, reactivity, signals |
| Factory development | factory/DEVELOPER_GUIDE.md | Extending problem generators |

## Cross-Referencing Format

Use this consistent format:
```
See [Description](FILE.md#section) for [what type of info].
```

Examples:
- See [JWT Implementation](BACKEND.md#jwt-tokens) for code details.
- See [API Authentication](API.md#authentication) for usage examples.
- See [Database Schema](DATABASE.md#users) for table structure.

## Update Checklist

When updating code:
- [ ] Update the primary documentation file (from ownership table)
- [ ] Check for cross-references that need updating
- [ ] Do NOT copy-paste content to multiple docs - use links instead
- [ ] Run link checker before committing (future: add to CI)

## Writing Style

- **Concise:** One topic per file
- **Scannable:** Use headings, tables, code blocks
- **Actionable:** Provide examples and commands
- **Complete:** Include error cases and troubleshooting

## Avoiding Redundancy

### Bad: Duplicating Content
```markdown
<!-- In ARCHITECTURE.md -->
The ELO formula is: E = 1/(1 + 10^((Rb-Ra)/400))
K-factor is 32 for all topics...

<!-- In BACKEND.md -->
The ELO formula is: E = 1/(1 + 10^((Rb-Ra)/400))
K-factor is 32 for all topics...
```

### Good: Single Source with Cross-References
```markdown
<!-- In ARCHITECTURE.md -->
Each user maintains separate ELO ratings for 8 math topics.
See [ELO Implementation](BACKEND.md#elo-system) for formulas and algorithm.

<!-- In BACKEND.md -->
## ELO System
The ELO formula is: E = 1/(1 + 10^((Rb-Ra)/400))
K-factor is 32 for all topics...
```

## Documentation Review Process

Before submitting documentation changes:

1. **Check ownership table** - Are you editing the correct authoritative file?
2. **Search for duplicates** - Does this information exist elsewhere?
3. **Update cross-references** - Did you add/remove sections that are referenced?
4. **Test examples** - Do code examples compile and run?
5. **Check links** - Are all markdown links valid?

## Link Validation

```bash
# Find all documentation links
grep -r "](.*\.md" docs/ | grep -v "http"

# Manually verify each link points to existing file and section
```

## Future Improvements

- [ ] Automated link checker in CI
- [ ] Documentation linter for style consistency
- [ ] Auto-generated API documentation from code
- [ ] Documentation version for each release tag
