# Universal AI Coding Agent Ruleset v2.0

> **CRITICAL**: These rules apply to ALL software engineering projects, regardless of language, framework, or infrastructure. Project-specific rules supplement but never override these core principles. When rules conflict, the higher-numbered priority wins (security > performance > convenience).

---

## 0. First Principles (READ FIRST)

Before writing any code, the agent MUST answer these questions:

1. **What problem does this solve?** — If you can't articulate it in one sentence, stop and clarify.
2. **What are the failure modes?** — Every feature has at least one. Identify them before coding.
3. **What is the simplest correct solution?** — Not the cleverest. Not the most abstract. The simplest that is correct, safe, and maintainable.
4. **Does this already exist in the codebase?** — Search before creating. Duplication is a bug.
5. **What breaks if this is wrong?** — Calibrate effort to blast radius. Auth bug > cosmetic bug.

### Hierarchy of Concerns
```
1. Correctness   — Does it do the right thing?
2. Security      — Can it be exploited?
3. Reliability   — Does it handle failure gracefully?
4. Maintainability — Can someone else understand and change it?
5. Performance   — Is it fast enough? (measure, don't guess)
6. Elegance      — Is it clean? (only after 1-5 are satisfied)
```

---

## 1. Code Quality & Standards (MANDATORY)

### Type Safety & Strictness
- **Always use the strictest mode available** — `strict: true` (TypeScript), `#![deny(unsafe_code)]` + `clippy::pedantic` (Rust), `type hints + mypy --strict` (Python), `-Wall -Werror` (C/C++)
- **No implicit types** — Explicitly declare types for all function signatures, return values, and public API boundaries. Internal locals may use inference when the type is obvious from context
- **No `any` / `Object` / `interface{}` escape hatches** — Use proper union types, generics, `unknown`, or constrained type parameters. If you must use an escape hatch, wrap it in a single function with a doc comment explaining why
- **Validate at every trust boundary** — API requests, file reads, user input, environment variables, deserialized data, IPC messages, CLI arguments. Use schema validation (Zod, Pydantic, serde, etc.)
- **Newtype pattern for domain types** — Don't pass raw `string` for UserId, Email, URL. Wrap in branded/newtype to prevent misuse at compile time

### SOLID Principles (Non-negotiable)
- **Single Responsibility** — One module = one reason to change. If a file exceeds 300 lines or a function exceeds 40 lines, refactor
- **Open/Closed** — Design for extension without modification. Use interfaces, traits, abstract classes, and dependency injection
- **Liskov Substitution** — Subtypes must be substitutable for their base types without altering program correctness
- **Interface Segregation** — Many specific interfaces > one general-purpose interface. No client should depend on methods it doesn't use
- **Dependency Inversion** — High-level modules must not depend on low-level modules. Both should depend on abstractions

### DRY & YAGNI (Balanced)
- **Rule of Three** — Tolerate duplication twice. Extract on the third occurrence. Premature abstraction is worse than duplication
- **Centralize configuration** — One source of truth for constants, settings, feature flags, magic numbers
- **Reusable components** — Build generic, composable building blocks with clear contracts
- **YAGNI** — Don't build abstractions for hypothetical future requirements. Build for today, design interfaces that allow tomorrow

### Code Style & Consistency
- **Follow language conventions** — PEP 8 (Python), `rustfmt` defaults (Rust), Prettier + ESLint (JS/TS), `gofmt` (Go), PSR-12 (PHP)
- **Enforce via tooling** — Linters and formatters run on save and in CI. No style debates in code review
- **Naming conventions**:
  - `camelCase` for variables/functions (JS/TS, Java, C#, Go exported)
  - `PascalCase` for types/classes/interfaces/enums
  - `snake_case` for variables/functions (Python, Rust, Ruby, Go unexported)
  - `SCREAMING_SNAKE_CASE` for compile-time constants and environment variable names
  - **Descriptive names** — `userAccountBalance` not `bal`. `fetchActiveOrders` not `getData`. Names should reveal intent
  - **Boolean names** — Prefix with `is`, `has`, `can`, `should`: `isActive`, `hasPermission`, `canRetry`
  - **Function names** — Verb-first: `calculateTotal`, `validateInput`, `sendNotification`
- **Consistent indentation** — Match project convention (2 or 4 spaces), enforce via `.editorconfig`
- **Max line length** — 100-120 characters. Break long lines for readability

### Code Smells to Reject Immediately
- Functions with more than 5 parameters (use an options/config object)
- Nested callbacks/promises deeper than 2 levels (flatten with async/await or combinators)
- Boolean parameters that change function behavior (use separate functions or an enum)
- Comments that explain *what* instead of *why* (rewrite the code to be self-documenting)
- Dead code, commented-out code, or TODO comments without issue references

---

## 2. System Design & Architecture

### Design Before Code
For any non-trivial feature (> 1 file changed), think through:
1. **Data model** — What entities exist? What are their relationships? What are the invariants?
2. **API contract** — What does the interface look like? Request/response shapes, error cases
3. **State management** — Where does state live? Who owns it? How is it synchronized?
4. **Failure modes** — What happens when the database is down? When the network is slow? When input is malformed?
5. **Concurrency** — Can this be called concurrently? Are there race conditions? Do we need locks/transactions?

### Layered Architecture (Mandatory Separation)
```
┌─────────────────────────────────────┐
│  Presentation / API Layer           │  ← HTTP handlers, CLI, UI components
│  (thin: parse request, call service,│     No business logic here
│   format response)                  │
├─────────────────────────────────────┤
│  Application / Service Layer        │  ← Orchestration, use cases, workflows
│  (coordinates domain objects,       │     Transaction boundaries live here
│   enforces business rules)          │
├─────────────────────────────────────┤
│  Domain Layer                       │  ← Core business logic, entities, value objects
│  (pure, no I/O, no framework deps) │     Most testable layer
├─────────────────────────────────────┤
│  Infrastructure Layer               │  ← Database, HTTP clients, file I/O, caches
│  (implements interfaces defined     │     Swappable implementations
│   by domain/service layers)         │
└─────────────────────────────────────┘
```

- **Dependencies point inward** — Infrastructure depends on Domain, never the reverse
- **Domain layer has zero external dependencies** — No ORM, no HTTP, no framework imports
- **Service layer orchestrates** — It calls domain logic and infrastructure, but contains no business rules itself

### Dependency Management
- **Constructor injection** — Pass dependencies as constructor/function parameters. No global singletons, no service locators
- **Interface-based design** — Code against traits/interfaces, not concrete implementations. This enables testing and swapping
- **Avoid circular dependencies** — If A depends on B and B depends on A, extract the shared concern into C
- **Dependency direction** — Always from volatile (UI, API) toward stable (domain, core logic)

### Stateless Design
- **Stateless services** — All persistent state in database/cache, never in application memory (enables horizontal scaling, zero-downtime deploys)
- **Idempotent operations** — Design all write operations to be safely retryable. Use idempotency keys for critical mutations
- **Immutable data** — Prefer immutable data structures. Mutations should be explicit, contained, and documented
- **Event-driven when appropriate** — Use events/messages for cross-boundary communication instead of direct calls

### API Design
- **Contract-first** — Define the API schema before implementing. Use OpenAPI, Protocol Buffers, GraphQL SDL, or equivalent
- **RESTful conventions** — `GET` (read), `POST` (create), `PUT` (full replace), `PATCH` (partial update), `DELETE` (remove). Use proper HTTP status codes
- **Versioning** — URL path (`/v1/`) for breaking changes. Use additive changes (new fields) for non-breaking evolution
- **Pagination** — All list endpoints MUST support pagination. Prefer cursor-based for large/real-time datasets, offset/limit for simple cases
- **Filtering & sorting** — Consistent query param conventions: `?status=active&sort=-created_at&limit=20`
- **Standard error responses** — Consistent structure across all endpoints:
  ```json
  {
    "error": {
      "code": "VALIDATION_ERROR",
      "message": "Human-readable description",
      "details": [{ "field": "email", "message": "Invalid format" }],
      "requestId": "req_abc123"
    }
  }
  ```
- **Rate limiting headers** — Return `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset` on all endpoints
- **HATEOAS for discoverability** — Include `_links` for related resources when it adds value (not mandatory for internal APIs)

---

## 3. Data Modeling & Database Design

### Schema Design Principles
- **Normalize first, denormalize for performance** — Start with 3NF. Denormalize only when you have measured query performance problems
- **Every table needs a primary key** — Prefer UUIDs or ULIDs for distributed systems. Auto-increment integers for single-database systems
- **Timestamps on everything** — `created_at` and `updated_at` on every table. Use `timestamptz` (timezone-aware)
- **Soft deletes when data matters** — Use `deleted_at` column instead of `DELETE` for audit-critical data. Hard delete for ephemeral data
- **Foreign keys are mandatory** — Enforce referential integrity at the database level, not just application level
- **Enums in the database** — Use database-level enums or check constraints, not magic strings

### Migration Discipline
- **Every schema change is a migration** — No manual DDL in production. Ever
- **Migrations must be reversible** — Write both `up` and `down` (or `undo`) for every migration
- **Migrations must be backward-compatible** — Deploy migration first, then code. Never deploy code that requires a migration that hasn't run yet
- **Additive changes only in zero-downtime systems** — Add columns (nullable or with defaults), don't rename or remove until old code is fully retired
- **Test migrations against production-like data** — A migration that works on an empty database may fail on 10M rows

### Query Discipline
- **Index strategy** — Index all foreign keys, all columns used in `WHERE`/`JOIN`/`ORDER BY`. Use composite indexes for multi-column queries. Use `EXPLAIN ANALYZE` to verify
- **No N+1 queries** — Use eager loading, joins, subqueries, or batching (DataLoader pattern). Detect with query logging in development
- **Pagination is mandatory** — Never `SELECT *` without `LIMIT`. Use keyset pagination for large tables
- **Connection pooling** — Always use a connection pool. Size it appropriately (typically 2-4x CPU cores)
- **Parameterized queries only** — Never interpolate user input into SQL strings. Use prepared statements or ORM query builders
- **Transactions for multi-step writes** — If two writes must succeed or fail together, wrap them in a transaction. Use the narrowest isolation level that ensures correctness

---

## 4. Error Handling & Resilience (CRITICAL)

### Error Handling Philosophy
- **Errors are values, not exceptions** — Prefer `Result<T, E>` (Rust), return types with error unions, or typed error classes over throwing generic exceptions
- **Never swallow errors** — Every error must be logged, propagated, or explicitly handled with a documented reason
- **Fail fast at startup** — Validate configuration, database connectivity, and required services on boot. Crash immediately if something is wrong
- **Fail gracefully at runtime** — Degrade functionality rather than crash. Show users a meaningful error, not a stack trace
- **Distinguish recoverable from fatal** — Recoverable: retry, fallback, degrade. Fatal: log, alert, crash cleanly

### Error Classification
```
┌──────────────────┬───────────────────────────────────────────────────┐
│ Category         │ Handling Strategy                                 │
├──────────────────┼───────────────────────────────────────────────────┤
│ Validation       │ Return 400 with field-level errors. Don't log    │
│ Authentication   │ Return 401. Log at WARN level                    │
│ Authorization    │ Return 403. Log at WARN level with user context  │
│ Not Found        │ Return 404. Don't log (unless unexpected)        │
│ Conflict         │ Return 409. Log at INFO level                    │
│ Rate Limited     │ Return 429. Log at WARN with client identifier   │
│ Internal Error   │ Return 500. Log at ERROR with full context       │
│ Upstream Failure │ Return 502/503. Log at ERROR, trigger circuit    │
│ Timeout          │ Return 504. Log at ERROR, consider retry         │
└──────────────────┴───────────────────────────────────────────────────┘
```

### Resilience Patterns
- **Timeouts on everything** — Every network call, database query, and external service call must have a timeout. No infinite waits
- **Retries with exponential backoff** — Retry transient failures (network errors, 503s) with jitter. Max 3 retries. Never retry non-idempotent operations without idempotency keys
- **Circuit breakers** — When an upstream service fails repeatedly, stop calling it. Check periodically. Restore when healthy
- **Bulkheads** — Isolate failure domains. A slow database query shouldn't block unrelated API endpoints
- **Graceful shutdown** — Handle SIGTERM/SIGINT. Drain in-flight requests. Close database connections. Flush logs. Exit cleanly
- **Health checks** — `/health` (liveness: is the process running?) and `/ready` (readiness: can it serve traffic?). Separate concerns

### Structured Logging (MANDATORY)
```json
{
  "level": "ERROR",
  "timestamp": "2025-01-15T10:30:00.000Z",
  "message": "Failed to process payment",
  "service": "payment-service",
  "requestId": "req_abc123",
  "userId": "usr_xyz789",
  "error": {
    "type": "StripeApiError",
    "message": "Card declined",
    "code": "card_declined",
    "stack": "..."
  },
  "context": {
    "orderId": "ord_456",
    "amount": 2999,
    "currency": "USD"
  },
  "duration_ms": 1250
}
```

### Logging Levels
| Level | When | Production? |
|-------|------|-------------|
| `TRACE` | Per-line execution detail | Never |
| `DEBUG` | Diagnostic info for development | Never (unless temporarily enabled for incident) |
| `INFO` | Significant business events (user created, order placed, job completed) | Yes |
| `WARN` | Unexpected but recoverable (deprecated API used, retry succeeded, rate limit approaching) | Yes |
| `ERROR` | Operation failed, requires attention (API call failed, database error, unhandled exception) | Yes + Alert |
| `FATAL` | System is unusable (database unreachable, critical config missing, out of memory) | Yes + Page |

### Logging Rules
- ✅ **Structured JSON format** with consistent field names across all services
- ✅ **Correlation IDs** on every request, propagated across service boundaries
- ✅ **Log at boundaries** — HTTP requests (method, path, status, duration), database queries (query, duration), external API calls
- ✅ **Include error context** — Full error object with stack trace, not just `error.message`
- ✅ **Centralized logger** — One logger instance/factory for the entire application
- ❌ **NEVER log sensitive data** — Passwords, API keys, tokens, PII, credit card numbers, session IDs. Redact or mask
- ❌ **NEVER use `console.log()` / `println!()` / `print()`** — Always use the structured logging framework
- ❌ **NEVER log in tight loops** — Aggregate and log summaries. One log per batch, not per item
- ❌ **NEVER log request/response bodies in production** — Unless explicitly redacted and rate-limited

---

## 5. Security Standards (NON-NEGOTIABLE)

### Threat Modeling (Think Before You Code)
For any feature that handles user data, authentication, or external input:
1. **What are the assets?** — User data, credentials, financial info, system access
2. **Who are the threat actors?** — Unauthenticated users, authenticated users, insiders, automated bots
3. **What are the attack vectors?** — Input injection, authentication bypass, privilege escalation, data exfiltration
4. **What are the mitigations?** — Input validation, access control, encryption, rate limiting, audit logging

### Input Validation & Sanitization
- **Validate all inputs at the boundary** — API requests, form data, URL params, headers, file uploads, WebSocket messages, CLI arguments
- **Schema validation** — Zod (TS), Pydantic (Python), serde + validator (Rust), Joi (Node). Validate shape AND business rules
- **Allowlist, not blocklist** — Define what IS allowed. Reject everything else
- **Context-specific sanitization** — HTML escape for rendering, SQL parameterization for queries, shell escape for commands, URL encoding for redirects
- **File upload validation** — Check MIME type (magic bytes, not just extension), enforce size limits, scan for malware, store outside webroot with randomized names

### Authentication & Authorization
- **Password hashing** — Argon2id (preferred), bcrypt (12+ rounds), scrypt. NEVER plain text, MD5, SHA1, or SHA256 without salt
- **Session management** — HttpOnly + Secure + SameSite=Lax cookies. Rotate session ID on privilege change. Absolute timeout (24h) + idle timeout (30min)
- **Token best practices** — Short-lived access tokens (15 min), refresh token rotation with reuse detection, verify signature AND claims (exp, iss, aud)
- **RBAC/ABAC** — Check permissions on every protected endpoint. Never trust client-side role checks. Enforce at the service layer
- **Principle of least privilege** — Users, services, and database connections get minimum required permissions. Separate read/write database users

### OWASP Top 10 (2021+) Mitigations
- **A01: Broken Access Control** — Deny by default. Verify ownership on every resource access. Log access control failures
- **A02: Cryptographic Failures** — TLS 1.2+ everywhere. Encrypt PII at rest. Use strong algorithms (AES-256-GCM, ChaCha20-Poly1305)
- **A03: Injection** — Parameterized queries only. No string interpolation for SQL, OS commands, LDAP, XPath
- **A04: Insecure Design** — Threat model during design. Use secure design patterns. Limit resource consumption
- **A05: Security Misconfiguration** — Disable debug in production. Remove default credentials. Harden HTTP headers. Automate configuration
- **A06: Vulnerable Components** — Audit dependencies continuously. Pin versions. Update security patches within 48 hours
- **A07: Auth Failures** — Rate limit login attempts. Implement account lockout. Use MFA for sensitive operations
- **A08: Data Integrity Failures** — Verify signatures on updates/deployments. Use SRI for CDN resources. Validate CI/CD pipeline integrity
- **A09: Logging Failures** — Log all auth events, access control failures, input validation failures. Ensure logs can't be tampered with
- **A10: SSRF** — Validate and sanitize all URLs. Blocklist private IP ranges. Use allowlists for external service calls

### API Security Headers (Mandatory for Web)
```
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
Content-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: camera=(), microphone=(), geolocation=()
```

### CORS
- **Never use `*` in production** — Whitelist specific origins
- **Credentials mode** — If using cookies, set `Access-Control-Allow-Credentials: true` with explicit origin (not `*`)
- **Preflight caching** — Set `Access-Control-Max-Age` to reduce OPTIONS requests

---

## 6. Concurrency & Distributed Systems

### Concurrency Fundamentals
- **Shared mutable state is the root of all evil** — Minimize it. Use message passing, immutable data, or synchronization primitives
- **Prefer channels/actors over locks** — Go channels, Rust mpsc, Erlang/Elixir processes, Akka actors
- **If you must use locks** — Lock ordering (prevent deadlocks), minimize critical sections, prefer RwLock over Mutex when reads dominate
- **Async/await discipline** — Never block the async runtime. Use `spawn_blocking` / `task.run_in_executor` for CPU-heavy or blocking I/O work
- **Cancellation safety** — All async operations must handle cancellation gracefully. Clean up resources on cancel

### Distributed Systems Principles
- **CAP theorem awareness** — Know whether your system prioritizes Consistency or Availability during partitions. Document the choice
- **Eventual consistency** — If using eventual consistency, design UIs and APIs that communicate this to users (e.g., "changes may take a few seconds to appear")
- **Idempotency keys** — All mutating API calls in distributed systems must support idempotency keys to handle retries safely
- **Distributed transactions** — Avoid when possible. Use sagas/choreography for multi-service workflows. Compensating transactions for rollback
- **Clock skew** — Never rely on wall-clock ordering across machines. Use logical clocks, vector clocks, or centralized sequence generators
- **At-least-once delivery** — Assume messages/events can be delivered multiple times. Design consumers to be idempotent

### Rate Limiting & Backpressure
- **Rate limit all public endpoints** — Token bucket or sliding window. Return 429 with `Retry-After` header
- **Backpressure propagation** — When overwhelmed, push back on callers rather than buffering unboundedly. Bounded queues, not unbounded
- **Graceful load shedding** — Under extreme load, reject low-priority requests to protect critical paths

---

## 7. Performance & Optimization

### Performance Philosophy
- **Measure first, optimize second** — No optimization without profiling data. Use flame graphs, `EXPLAIN ANALYZE`, browser DevTools, `perf`, `pprof`
- **Optimize the bottleneck** — Identify the actual constraint (CPU, memory, I/O, network, database). Don't optimize what isn't slow
- **Set performance budgets** — Define acceptable latency (P50, P95, P99), throughput, and resource usage before building

### Database Performance
- **Index strategy** — Index all foreign keys, `WHERE` columns, `JOIN` columns, `ORDER BY` columns. Use composite indexes for multi-column queries. Regularly review unused indexes
- **N+1 detection** — Use query logging in development to detect N+1 patterns. Fix with eager loading, joins, or DataLoader
- **Connection pooling** — Always. Size = `(2 * CPU cores) + disk spindles` as a starting point. Monitor pool exhaustion
- **Read replicas** — Route read-heavy queries to replicas. Ensure application handles replication lag
- **Caching layers** — L1: Application-level (in-process, TTL-based). L2: Distributed cache (Redis/Memcached). Invalidation strategy must be defined before caching

### Application Performance
- **Lazy loading** — Load resources only when needed. Code splitting, lazy imports, on-demand initialization
- **Batch operations** — Group database inserts/updates, API calls, event emissions. Amortize overhead
- **Streaming over buffering** — For large datasets, stream results instead of loading everything into memory
- **Object pooling** — Reuse expensive objects (database connections, HTTP clients, thread pools) instead of creating/destroying
- **Memory management** — Clean up event listeners, close connections/handles, avoid closures that capture large scopes. Profile for leaks

### Frontend Performance (Web)
- **Core Web Vitals targets** — LCP < 2.5s, INP < 200ms, CLS < 0.1
- **Code splitting** — Route-based splitting minimum. Component-level for large features. Dynamic `import()` for heavy libraries
- **Image optimization** — WebP/AVIF with fallbacks, lazy loading below the fold, responsive `srcset`, proper `width`/`height` to prevent CLS
- **Bundle analysis** — Run bundle analyzer in CI. Alert on bundle size regression (> 5% increase)
- **Render optimization** — Virtualize long lists (react-virtual, tanstack-virtual). Memoize expensive computations. Avoid unnecessary re-renders
- **Network optimization** — HTTP/2 or HTTP/3, compression (Brotli > gzip), CDN for static assets, preload critical resources

---

## 8. Testing Strategy (MANDATORY)

### Testing Philosophy
- **Tests are a specification** — They document what the system should do. Write them as if someone will read them to understand the feature
- **Test behavior, not implementation** — Test the public API/contract. Don't test private methods or internal state
- **Tests must be deterministic** — No random data, no wall-clock dependencies, no reliance on external services, no test ordering dependencies
- **Tests must be fast** — Unit: < 50ms. Integration: < 2s. E2E: < 30s. If tests are slow, developers won't run them

### Coverage Requirements
| Layer | Target | Focus |
|-------|--------|-------|
| Critical paths (auth, payments, data mutations) | 100% | Every branch, every error case |
| Domain/business logic | 90%+ | Pure functions, state machines, validation |
| Service/application layer | 80%+ | Orchestration, error handling, edge cases |
| API endpoints | 80%+ | Request validation, response shape, auth, errors |
| UI components | 70%+ | User interactions, conditional rendering, accessibility |
| Infrastructure/glue code | 50%+ | Happy path + critical error paths |

### Testing Pyramid
```
              ╱╲
             ╱  ╲        E2E Tests (5-10%)
            ╱    ╲       Critical user journeys only
           ╱──────╲
          ╱        ╲     Integration Tests (20-30%)
         ╱          ╲    API endpoints, service interactions, DB queries
        ╱────────────╲
       ╱              ╲  Unit Tests (60-70%)
      ╱                ╲ Pure functions, domain logic, utilities
     ╱──────────────────╲
```

### Test Structure (AAA Pattern)
```
// Arrange — Set up preconditions and inputs
// Act     — Execute the behavior under test
// Assert  — Verify the expected outcome
```
- **One logical assertion per test** — Test one behavior. Multiple `expect` calls are fine if they verify one concept
- **Descriptive test names** — `should return 404 when user does not exist` not `test user`
- **Test edge cases explicitly** — Empty inputs, null/undefined, boundary values, concurrent access, error conditions

### Test Types & Tools
- **Unit tests** — Pure functions, domain logic (Vitest, Jest, pytest, `#[test]` in Rust, Go `testing`)
- **Integration tests** — API endpoints, database interactions, service layer (Supertest, TestContainers, `sqlx::test`)
- **E2E tests** — Critical user flows only (Playwright strongly preferred, Cypress acceptable)
- **Contract tests** — API compatibility between services (Pact, Dredd)
- **Property-based tests** — For algorithmic code, parsers, serializers (fast-check, Hypothesis, proptest)
- **Regression tests** — Every bug fix MUST include a test that would have caught the bug
- **Load/stress tests** — For performance-critical paths before release (k6, Grafana k6, Artillery)

### What NOT to Test
- Third-party library internals (trust their tests)
- Trivial getters/setters with no logic
- Framework boilerplate (routing config, DI wiring)
- Implementation details that can change without affecting behavior

---

## 9. Version Control & Git Practices

### Commit Standards (Conventional Commits)
```
type(scope): description

feat(auth):     Add JWT refresh token rotation
fix(payments):  Handle race condition in concurrent checkout
docs(readme):   Update deployment instructions
refactor(db):   Extract connection pooling into shared module
perf(search):   Add composite index for full-text queries
test(users):    Add integration tests for user deletion
chore(deps):    Update tokio to 1.35, fix deprecation warnings
ci(pipeline):   Add SAST scanning step
```
- **Atomic commits** — One logical change per commit. Must be revertable without side effects
- **Present tense, imperative mood** — "Add feature" not "Added feature" or "Adds feature"
- **Reference issues** — `fix(auth): Prevent session fixation (#234)`
- **No WIP commits on main** — Squash or rebase before merging

### Branching Strategy
- **`main` is always deployable** — Protected branch. Requires PR + CI passing + review
- **Short-lived feature branches** — `feature/user-auth`, `fix/payment-race-condition`. Merge within 1-3 days
- **Hotfix branches** — `hotfix/critical-xss-patch`. Fast-track review, deploy immediately
- **No direct commits to `main`** — All changes via pull requests. No exceptions

### Pull Request Standards
- **Small PRs** — < 400 lines changed. If larger, split into stacked PRs
- **Self-review first** — Review your own diff before requesting others
- **PR template** (mandatory fields):
  ```markdown
  ## What — One-sentence summary of the change
  ## Why — Link to issue/ticket or business justification
  ## How — Brief technical approach (only if non-obvious)
  ## Testing — How to verify (automated tests + manual steps if needed)
  ## Breaking Changes — List any breaking changes (API, schema, config)
  ```
- **CI must pass** — All tests, linting, type-checking, security scanning green before merge
- **Squash merge by default** — Clean linear history on main. Merge commits only for long-running branches with meaningful individual commits

---

## 10. Project Structure & Organization

### Universal Principles
- **Group by feature/domain, not by file type** — `features/auth/` not `controllers/auth-controller.ts` + `services/auth-service.ts`
- **Flat over nested** — Max 4 levels deep. If you need more, your modules are too granular
- **Colocation** — Tests, types, and utilities live next to the code they support
- **Barrel exports with caution** — Use `index.ts` / `mod.rs` for public API, but don't re-export everything (causes circular deps and bundle bloat)

### Recommended Structure (Feature-Based)
```
src/
├── features/
│   ├── auth/
│   │   ├── auth.service.ts       # Business logic
│   │   ├── auth.controller.ts    # HTTP handler (thin)
│   │   ├── auth.repository.ts    # Data access
│   │   ├── auth.types.ts         # Types/interfaces
│   │   ├── auth.validation.ts    # Input schemas
│   │   └── __tests__/
│   │       ├── auth.service.test.ts
│   │       └── auth.controller.test.ts
│   ├── users/
│   └── payments/
├── shared/
│   ├── errors/          # Error types, error handling utilities
│   ├── middleware/       # Auth, logging, rate limiting
│   ├── types/           # Shared type definitions
│   └── utils/           # Pure utility functions
├── infrastructure/
│   ├── database/        # Connection, migrations, query builder
│   ├── cache/           # Redis/cache abstraction
│   ├── http-client/     # External API client wrapper
│   └── logger/          # Logging setup
└── config/
    ├── index.ts         # Validated config object
    └── schema.ts        # Config validation schema
```

### Configuration Management
- **Environment-based config** — `.env.development`, `.env.production`, `.env.test`
- **Never commit secrets** — `.env` in `.gitignore`. Provide `.env.example` with placeholder values and comments
- **Validate at startup** — Parse and validate all env vars on boot. Crash immediately if required vars are missing or invalid
- **Type-safe config object** — Export a single validated config object. No `process.env.X` scattered throughout the codebase
- **Config hierarchy** — CLI args > Environment vars > `.env` file > defaults

---

## 11. Observability & Monitoring

### Three Pillars of Observability
1. **Logs** — Structured, correlated, centralized (see Section 4)
2. **Metrics** — Quantitative measurements over time (counters, gauges, histograms)
3. **Traces** — Request flow across service boundaries (OpenTelemetry)

### Key Metrics (RED + USE)
**RED Method (request-driven services):**
- **Rate** — Requests per second
- **Errors** — Error rate (% of requests resulting in errors)
- **Duration** — Latency distribution (P50, P95, P99)

**USE Method (infrastructure resources):**
- **Utilization** — % of resource capacity in use (CPU, memory, disk, connections)
- **Saturation** — Queue depth, backlog size (work waiting to be processed)
- **Errors** — Resource-level errors (disk failures, OOM kills, connection timeouts)

### Alerting Strategy
| Severity | Condition | Response Time | Example |
|----------|-----------|---------------|---------|
| **P1 Critical** | Service down, data loss risk | < 15 min | Database unreachable, auth service down |
| **P2 High** | Degraded performance, elevated errors | < 1 hour | Error rate > 1%, P99 > 5s |
| **P3 Medium** | Anomalous but functional | < 4 hours | Memory usage > 80%, disk > 70% |
| **P4 Low** | Informational, trend-based | Next business day | Slow query count increasing, dependency EOL |

### Health Endpoints
- **`GET /health`** — Liveness probe. Returns 200 if process is running. No dependency checks
- **`GET /ready`** — Readiness probe. Returns 200 only if all critical dependencies (database, cache, required services) are reachable
- **`GET /metrics`** — Prometheus-compatible metrics endpoint (if using metrics collection)

---

## 12. Documentation Standards

### Code Documentation
- **Self-documenting code first** — Clear names and structure > comments. Comment *why*, never *what*
- **Doc comments on public APIs** — Every public function, type, and module gets a doc comment explaining purpose, parameters, return value, errors, and usage example
- **No stale comments** — A wrong comment is worse than no comment. Update or delete when code changes

### Project Documentation (Minimum Viable)
- **README.md** — Project description, prerequisites, quick start, running tests, deployment, env vars
- **ARCHITECTURE.md** — System overview, component diagram, data flow, key design decisions
- **CHANGELOG.md** — Keep a Changelog format. Updated with every release
- **ADRs (Architecture Decision Records)** — Document significant technical decisions with context, options considered, and rationale

### API Documentation
- **OpenAPI/Swagger** for REST APIs — Auto-generated from code annotations when possible
- **Examples for every endpoint** — Request/response examples with realistic data
- **Error documentation** — All possible error codes and their meaning

---

## 13. Deployment & Infrastructure

### Container Best Practices
- **Multi-stage builds** — Separate build and runtime stages. Final image should be minimal (distroless, alpine, scratch)
- **Non-root user** — Run as unprivileged user. Set `USER` in Dockerfile
- **No secrets in images** — Use runtime env vars, mounted secrets, or secret managers
- **Health checks** — `HEALTHCHECK` in Dockerfile or orchestrator-level probes
- **Deterministic builds** — Pin base image digests, lock dependency versions. Same input = same image
- **Image scanning** — Scan for vulnerabilities in CI (Trivy, Snyk, Grype)

### CI/CD Pipeline (Minimum)
```
1. Checkout + dependency install (cached)
2. Lint + format check
3. Type checking (strict mode)
4. Unit tests
5. Integration tests
6. Security scanning (SAST + dependency audit)
7. Build artifact/image
8. E2E tests (against built artifact)
9. Deploy to staging (auto on main)
10. Deploy to production (manual approval or tag-based)
```

### Environment Strategy
- **Local** — Docker Compose or native tooling with hot reload. Seed data for development
- **Staging** — Mirrors production config. Auto-deploy on `main` merge. Used for final verification
- **Production** — Tag-based or manual-approval deployment. Blue/green or rolling updates. Instant rollback capability

### Infrastructure as Code
- **All infrastructure is code** — Terraform, Pulumi, CloudFormation, or equivalent. No manual console changes
- **State management** — Remote state with locking (S3 + DynamoDB, Terraform Cloud)
- **Environment parity** — Same IaC modules for staging and production with different variables

---

## 14. Dependencies & Supply Chain Security

### Dependency Hygiene
- **Lock files are mandatory** — Commit `package-lock.json`, `Cargo.lock`, `yarn.lock`, `poetry.lock`, `go.sum`. Reproducible builds
- **Pin versions in production** — Exact versions or patch range (`~1.2.3`). No `*` or `latest`
- **Audit continuously** — `npm audit`, `cargo audit`, `pip-audit`, Dependabot/Renovate. Fix critical/high vulnerabilities within 48 hours
- **Minimize dependencies** — Every dependency is attack surface + maintenance burden. Evaluate: Can we write this in < 50 lines?
- **Vet new dependencies** — Before adding: check maintenance status, download count, license, security history, bundle size impact

### Update Strategy
- **Automated PRs** — Dependabot or Renovate for automated dependency update PRs
- **Security patches** — Apply within 48 hours of disclosure
- **Minor/patch updates** — Weekly or bi-weekly batch
- **Major updates** — Evaluate changelog, test thoroughly, schedule during low-risk windows

---

## 15. Accessibility (a11y) Standards

### WCAG 2.2 AA Compliance (Web Applications)
- **Semantic HTML first** — Use `<button>`, `<nav>`, `<main>`, `<article>`, `<form>` before reaching for ARIA
- **Keyboard navigation** — All interactive elements reachable and operable via keyboard (Tab, Enter, Space, Escape, Arrow keys)
- **Screen reader support** — Meaningful alt text, ARIA labels for non-text content, live regions for dynamic updates
- **Color contrast** — 4.5:1 minimum for normal text, 3:1 for large text. Never convey information by color alone
- **Focus management** — Visible focus indicators. Logical focus order. Focus trapping in modals. Return focus on close
- **Responsive design** — Functional from 320px to 4K. No horizontal scrolling at 320px. Touch targets ≥ 44x44px
- **Motion sensitivity** — Respect `prefers-reduced-motion`. No auto-playing animations without user consent
- **Form accessibility** — Labels on all inputs, error messages linked to fields, clear validation feedback

---

## 16. AI Agent-Specific Directives

### Before Writing Any Code
1. **Read the codebase first** — Check README, architecture docs, existing patterns, recent commits. Understand before changing
2. **Search for existing solutions** — The function you're about to write may already exist. Search the codebase thoroughly
3. **Match existing patterns** — Follow the conventions already established. Don't introduce new patterns without explicit discussion
4. **Understand the dependency graph** — Know what depends on what you're changing. Check callers, not just callees
5. **Consider the blast radius** — A change to a shared utility affects every consumer. A change to a leaf module affects only itself

### Code Generation Standards
- **Production-ready only** — No placeholder code, no `// TODO: implement this`, no stub functions. Every line must be runnable
- **All imports included** — Generated code must compile/run without adding missing imports
- **Error handling included** — Every I/O operation, every fallible call must have error handling. No happy-path-only code
- **Types included** — All function signatures fully typed. No implicit `any` or missing return types
- **Tests included** — When adding a new function or fixing a bug, include or update tests in the same change

### Build Verification (Before Considering Work Complete)
- ✅ Code compiles/transpiles without errors (`cargo build`, `tsc --noEmit`, `npm run build`)
- ✅ Type checking passes in strict mode
- ✅ Linter passes with zero new warnings
- ✅ All existing tests still pass
- ✅ New tests cover the change
- ✅ No new `unsafe` blocks, `any` types, or `unwrap()`/`!` without justification

### Migration & Schema Changes
- ✅ Migration file created (never manual DDL)
- ✅ Migration is reversible (up + down)
- ✅ Migration is backward-compatible with currently deployed code
- ✅ Indexes added for new foreign keys and query patterns
- ✅ Migration tested against realistic data volume

### What the Agent Must NEVER Do
- ❌ Delete or weaken existing tests without explicit instruction
- ❌ Remove error handling to "simplify" code
- ❌ Add `// @ts-ignore`, `#[allow(unused)]`, `# type: ignore` without a comment explaining why
- ❌ Introduce global mutable state
- ❌ Hardcode secrets, API keys, or environment-specific values
- ❌ Add dependencies without evaluating alternatives and justifying the choice
- ❌ Make changes outside the scope of the current task without asking
- ❌ Guess at behavior when documentation or code is ambiguous — ask for clarification

### Communication Style
- **Terse and direct** — No filler, no preamble, no "Great question!"
- **Actionable** — Provide commands, file paths, specific code changes
- **Honest about uncertainty** — "I'm not sure about X" is better than a confident wrong answer
- **Minimal context switching** — Complete one task before starting another

---

## 17. Code Review Checklist

Before approving any change, verify:

### Correctness
- [ ] Logic is correct for all inputs (happy path, edge cases, error cases)
- [ ] State mutations are intentional and documented
- [ ] Concurrent access is handled (if applicable)
- [ ] Database queries are correct and efficient

### Security
- [ ] No hardcoded secrets or sensitive data
- [ ] All inputs validated and sanitized
- [ ] Authentication and authorization enforced
- [ ] No new injection vectors (SQL, XSS, command injection)

### Reliability
- [ ] Error handling is comprehensive (no swallowed errors)
- [ ] Timeouts set on all external calls
- [ ] Graceful degradation for non-critical failures
- [ ] Logging is appropriate (correct level, no PII, includes context)

### Maintainability
- [ ] Code follows existing patterns and conventions
- [ ] No unnecessary complexity or premature abstraction
- [ ] Tests cover new functionality and edge cases
- [ ] Documentation updated (if public API changed)

### Performance
- [ ] No N+1 queries or unbounded fetches
- [ ] No unnecessary allocations in hot paths
- [ ] Indexes added for new query patterns
- [ ] Bundle size impact acceptable (frontend)

---

## Final Principles

1. **Correctness over cleverness** — Clear, boring code that works > elegant code that's fragile
2. **Explicit over implicit** — Make behavior visible. No magic, no hidden side effects
3. **Fail fast, fail loud** — Surface errors immediately. Silent failures are the hardest bugs
4. **Measure before optimizing** — Profile, benchmark, then optimize the actual bottleneck
5. **Security by default** — Secure first, convenient second. Never the reverse
6. **Automate everything repeatable** — Formatting, linting, testing, deployment, dependency updates
7. **Design for change** — Requirements will change. Make it easy to adapt without rewriting
8. **Minimize blast radius** — Small changes, small deploys, small failures. Isolate what can go wrong
9. **Leave it better than you found it** — Boy Scout Rule. Small improvements compound

---

**Last Updated**: 2026-02-20
**Version**: 2.0.0
