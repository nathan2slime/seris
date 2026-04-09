# Seris Development Roadmap

> Philosophy: *Silence over noise, clarity over excess*

This roadmap outlines planned improvements for the Seris Discord bot, prioritized by impact and effort.

---

## 🎯 Current State Summary

**Strengths:**
- Clean modular architecture (commands/embeds/services)
- Comprehensive deployment options (Docker, local install, GitHub releases)
- Modern Rust 2021 edition with minimal attack surface

**Key Areas for Improvement:**
- Error handling reliability
- Test coverage
- Code documentation
- Production hardening

---

## Phase 1: Foundation & Reliability (High Priority)

### 1.1 Error Handling Improvements
**Goal:** Eliminate panics and improve fault tolerance

- [ ] Replace all `.unwrap()` calls with proper error handling
  - Priority: `main.rs` lines 52-53 (client startup)
  - Priority: All service layer API calls
- [ ] Implement structured error types with `thiserror`
  - Create `SerisError` enum with variants for each failure mode
  - Add error context for debugging
- [ ] Fix silent error swallowing in `commands/clear.rs`
  - Log errors even when not showing to user
- [ ] Add input validation
  - Validate API keys are non-empty at startup
  - Validate Discord permissions before operations

**Estimated Effort:** 2-3 days  
**Impact:** ⭐⭐⭐⭐⭐

---

### 1.2 Logging Standardization
**Goal:** Consistent, structured logging across the codebase

- [ ] Replace all `println!()` with `log`/`env_logger`
  - `commands/anime.rs`
  - `commands/manga.rs`
- [ ] Add structured logging with `tracing` crate (optional upgrade)
- [ ] Implement log levels consistently
  - `error!` for failures
  - `warn!` for recoverable issues
  - `info!` for significant events
  - `debug!` for development

**Estimated Effort:** 1 day  
**Impact:** ⭐⭐⭐⭐

---

### 1.3 Code Documentation
**Goal:** Self-documenting code with Rust doc comments

- [ ] Add `///` doc comments to all public functions
- [ ] Document all types and structs
- [ ] Add module-level documentation (`//!`)
- [ ] Include usage examples where applicable
- [ ] Run `cargo doc` to verify no warnings

**Estimated Effort:** 2 days  
**Impact:** ⭐⭐⭐⭐

---

## Phase 2: Quality Assurance (High Priority)

### 2.1 Unit Testing
**Goal:** Achieve 80%+ code coverage

- [ ] Set up test infrastructure
  - Add `tokio-test` for async tests
  - Configure test utilities module
- [ ] Write tests for service layer
  - `services/jikan.rs` - mock API responses
  - `services/nasa.rs` - mock API responses
- [ ] Write tests for command logic
  - Test embed builders in `embeds/`
  - Test validation logic
- [ ] Add CI test job to GitHub Actions

**Estimated Effort:** 4-5 days  
**Impact:** ⭐⭐⭐⭐⭐

---

### 2.2 Integration Testing
**Goal:** End-to-end testing for critical paths

- [ ] Create integration test suite
  - Test full command execution flow
  - Test error scenarios
- [ ] Mock Discord API interactions
- [ ] Test configuration loading edge cases

**Estimated Effort:** 2-3 days  
**Impact:** ⭐⭐⭐⭐

---

### 2.3 Linting & Formatting
**Goal:** Consistent code style and catch common issues

- [ ] Add `rustfmt.toml` configuration
- [ ] Add `clippy.toml` with project-specific lints
- [ ] Configure CI to run `cargo fmt --check` and `cargo clippy`
- [ ] Fix existing clippy warnings
- [ ] Consider adding `deny` lints for critical issues

**Estimated Effort:** 1 day  
**Impact:** ⭐⭐⭐

---

## Phase 3: Production Hardening (Medium Priority)

### 3.1 Graceful Shutdown
**Goal:** Clean shutdown without data loss or orphaned resources

- [ ] Implement proper signal handling
- [ ] Flush logs before exit
- [ ] Close HTTP connections cleanly
- [ ] Disconnect from Discord gracefully
- [ ] Add shutdown timeout

**Estimated Effort:** 1-2 days  
**Impact:** ⭐⭐⭐⭐

---

### 3.2 Configuration & Secrets
**Goal:** Secure and flexible configuration

- [ ] Add environment variable fallbacks for secrets
  - `SERIS_DISCORD_TOKEN`
  - `SERIS_NASA_API_KEY`
- [ ] Validate configuration at startup with clear error messages
- [ ] Add `.env` to `.gitignore`
- [ ] Document all configuration options in README
- [ ] Consider adding config schema validation

**Estimated Effort:** 1-2 days  
**Impact:** ⭐⭐⭐⭐

---

### 3.3 API Resilience
**Goal:** Handle external API failures gracefully

- [ ] Add HTTP request timeouts (currently missing)
- [ ] Implement retry logic with exponential backoff
- [ ] Add circuit breaker pattern for failing APIs
- [ ] Cache responses where appropriate (reduce API calls)
- [ ] Add rate limiting awareness

**Estimated Effort:** 2-3 days  
**Impact:** ⭐⭐⭐⭐

---

### 3.4 Security Hardening
**Goal:** Reduce attack surface

- [ ] Audit all external command execution (`cli.rs`)
  - Sanitize all paths and inputs
  - Consider using `std::process::Command` with explicit args
- [ ] Review permission requirements for all commands
- [ ] Add rate limiting for commands (prevent abuse)
- [ ] Consider adding audit logging for admin actions

**Estimated Effort:** 2 days  
**Impact:** ⭐⭐⭐⭐

---

## Phase 4: Developer Experience (Medium Priority)

### 4.1 Build & CI Improvements
**Goal:** Faster, more reliable builds and deployments

- [ ] Remove unnecessary workspace configuration
- [ ] Add feature flags for optional components
  - `cli` - admin CLI tools
  - `bot` - Discord bot functionality
- [ ] Optimize Docker build (multi-stage already good)
- [ ] Add automated release changelog generation
- [ ] Consider adding benchmark tests

**Estimated Effort:** 1-2 days  
**Impact:** ⭐⭐⭐

---

### 4.2 Health Checks & Monitoring
**Goal:** Observability for production deployments

- [ ] Implement HTTP health check endpoint (Docker port 8080)
  - `/health` - basic liveness
  - `/ready` - Discord connection status
- [ ] Add metrics collection (optional: `prometheus`)
- [ ] Add Docker healthcheck configuration
- [ ] Document monitoring recommendations

**Estimated Effort:** 1-2 days  
**Impact:** ⭐⭐⭐

---

### 4.3 Documentation Improvements
**Goal:** Better onboarding and user experience

- [ ] Add CONTRIBUTING.md
- [ ] Add architecture diagram
- [ ] Document API endpoints used
- [ ] Add troubleshooting guide to README
- [ ] Create deployment guide for different platforms

**Estimated Effort:** 1-2 days  
**Impact:** ⭐⭐⭐

---

## Phase 5: Future Enhancements (Low Priority)

### 5.1 Feature Additions
**Goal:** Expand bot capabilities (optional)

- [ ] Add more utility commands
- [ ] Implement command cooldowns
- [ ] Add database support for persistence
- [x] Consider plugin architecture
- [ ] Add admin dashboard (web UI)

**Estimated Effort:** Variable  
**Impact:** Depends on features

---

### 5.2 Performance Optimization
**Goal:** Reduce resource usage

- [x] Profile memory usage
- [x] Optimize API call patterns
- [x] Consider connection pooling
- [x] Benchmark critical paths

**Estimated Effort:** 2-3 days  
**Impact:** ⭐⭐ (current performance likely adequate)

---

## Quick Reference: Files Needing Attention

| File | Issue | Priority |
|------|-------|----------|
| `src/main.rs:52-53` | `.unwrap()` on client startup | Critical |
| `src/commands/clear.rs:22` | Magic number (40) | High |
| `src/commands/clear.rs:36` | Silent error swallowing | High |
| `src/commands/anime.rs` | `println!()` instead of logging | Medium |
| `src/commands/manga.rs` | `println!()` instead of logging | Medium |
| `src/cli.rs:12` | Hardcoded system path | Medium |
| `src/services/jikan.rs` | Redundant error handling | Low |
| `src/services/nasa.rs` | Redundant error handling | Low |
| `Cargo.toml` | Unnecessary workspace config | Low |

---

## Recommended Tooling Additions

```toml
[dev-dependencies]
tokio-test = "0.4"  # Async testing

[dependencies]
thiserror = "1.0"   # Error type derivation
tracing = "0.1"     # Structured logging (optional)
tracing-subscriber = "0.3"  # Log formatting
```

---

## Success Metrics

- [ ] Zero `.unwrap()` calls in production code
- [ ] 80%+ test coverage
- [ ] All public APIs documented
- [ ] CI passes with fmt, clippy, and tests
- [ ] Graceful shutdown completes in < 5 seconds
- [ ] No panic logs in production (30-day period)

---

*Last updated: 2026-04-02*
