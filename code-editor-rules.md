# FavKit Project Rules

## Project Context
FavKit is a modern Rust library and CLI tool for managing macOS Finder favorites, replacing the abandoned `mysides` tool.

## Architecture

1. **Clean Architecture**
   - **Domain Layer** (`finder/`):
     - Pure business logic and types
     - Rich domain model (not anemic)
     - Type-driven development
     - Business rules encoded in types
     - Domain invariants enforced at compile time
   - **Implementation Layer** (`system/`):
     - Technical details and infrastructure
     - Adapters for external systems
     - Implementation details

2. **Testing Strategy**
   - Outside-In TDD approach:
     1. Start with acceptance tests
     2. Use integration tests with mocks for external APIs
     3. Work inward with unit tests
     4. Implement minimal code to pass
     5. Refactor without changing behavior
   - Test Coverage:
     - Unit tests for all modules
     - Mocks only for outermost layer (macOS API)
     - Thorough error case testing
     - Prefer `Result` over `unwrap`

3. **Project Structure**
   ```plaintext
   src/
   ├── finder/                 # Domain layer
   ├── system/                 # Implementation layer
       ├── core_foundation.rs  # CF wrappers
       ├── favorites/          # macOS API wrapper
       └── macos/              # Low-level API calls
   tests/                      # Tests
   ├── mock/                   # Test doubles
   docs/                       # Documentation
   └── adr/                    # ADRs
   ```

## AI Agent Rules

1. **Rule Acknowledgment**
   - State which rule(s) you're following in responses
   - Abbreviate rule descriptions to single words/phrases

2. **Change Management**
   - ALWAYS read the file content before making changes
   - Make only explicitly requested changes
   - Stay focused on the specific task
   - Follow existing patterns
   - Document changes clearly
   - Keep conversation context
   - Don’t revert approved changes

3. **Communication**
   - Propose improvements after requested changes
   - Wait for user approval
   - Provide clear examples
   - Explain rationale
   - Ask for clarification when in doubt
   - Encourage explicit feedback from the user when uncertain.

4. **Architecture Review**
   - Ensure Clean Architecture compliance
   - Follow KISS, DRY, YAGNI, SOLID principles
   - Verify domain isolation
   - Check separation of concerns
   - Review and update ADRs

5. **Response Validation and Reasoning**
   - Validate all responses against `.cursorrules` before presenting them.
   - Include reasoning in the response:
     - **Summary**: What the response addresses.
     - **Validation**: State which rules are adhered to.
     - **Rationale**: Explain why the response is correct or necessary.
   - If a response violates any rules:
     1. Explicitly flag the inconsistency.
     2. Propose fallback or alternative solutions.
     3. Log the issue for refinement.
   - If the model is unsure or lacks information to provide a correct answer:
     - Clearly state: "I do not know the answer."
     - Avoid speculating or guessing.
     - Suggest next steps or alternative approaches to find the answer.

6. **Prompt Handling Best Practices**
   - The AI must interpret user prompts clearly and effectively by:
     1. Identifying unclear or vague requests and asking clarifying questions.
     2. Suggesting structured approaches for multi-step tasks.
     3. Providing outputs in the format requested by the user (e.g., examples, summaries, or detailed explanations).
   - For complex prompts, break the response into logical parts and guide the user step by step.
   - Avoid speculation or guessing; if the task or solution is unclear:
     - Explicitly state: "The requested task is ambiguous" or "I need more information."
     - Suggest steps the user can take to refine their request.

7. **Error Handling in Responses**
   - If validation fails, responses must:
     1. Explicitly describe the error.
     2. Propose fallback or alternative solutions.
     3. Log inconsistencies for future refinement.

8. **Dynamic Updates**
   - Periodically review and update `.cursorrules` based on evolving project needs and real-world usage feedback.

## Technical Requirements

1. **Tech Stack**
   ```toml
   [toolchain]
   rust = "nightly"
   edition = "2024"
   components = [
     "rustc", "rust-std", "rust-src",
     "rust-analyzer", "rust-docs",
     "rustfmt", "cargo", "clippy",
     "llvm-tools-preview"
   ]

   [dependencies]
   core-foundation = "*"  # https://docs.rs/core-foundation/latest/core_foundation/
   core-services = "*"    # https://docs.rs/core-services/latest/core_services/
   thiserror = "*"       # https://docs.rs/thiserror/latest/thiserror/
   dirs = "*"            # https://docs.rs/dirs/latest/dirs/

   [dev-dependencies]
   cargo-nextest = "*"   # https://github.com/nextest-rs/nextest
   cargo-llvm-cov = "*"  # https://github.com/taiki-e/cargo-llvm-cov
   bacon = "*"           # https://github.com/Canop/bacon
   ```

2. **Rust Development**
   - **Type System Usage**:
     - Express domain concepts through types
     - Encode business rules in the type system
     - Prevent invalid states at compile time
     ```rust
     // WRONG: Stringly-typed API
     fn add_favorite(path: String) -> Result<(), Error>
     
     // RIGHT: Domain concepts in types
     struct Target {
         path: ValidPath,
         kind: TargetKind,
     }
     ```
   - **Standard Library Traits**:
     - Avoid custom conversion methods like `convert` or `map`.
     - Use `From`, `TryFrom`, or `AsRef` for idiomatic conversion and reference operations.
     ```rust
     // WRONG: Custom conversion methods
     fn url_to_target() -> Target
     fn as_string() -> String
     fn into_bytes() -> Vec<u8>
     
     // RIGHT: Standard library traits
     impl From<Url> for Target
     impl TryFrom<&str> for Url
     impl AsRef<str> for Type
     impl Into<Vec<u8>> for Type
     ```
   - **Code Organization**:
     - Separate data from behavior
     - Use traits for abstraction
     - Prefer functional style over OOP
     - Use Rust idioms effectively
   - **Error Handling**:
     - `thiserror` for definitions
     - Custom error types per module
     - `Result` for fallible operations
   - **Testing the Code**:
     - Run the `make test` command to ensure the code compiles and all tests pass.
     - If errors or test failures occur:
       1. Validate the output to identify the issue.
       2. Apply necessary fixes and re-run the tests.
       3. Confirm all issues are resolved before presenting the final result.

3. **Documentation**
   - Clear README
   - API examples
   - Up-to-date ADRs
   - Usage examples
   - Error conditions

4. **Git Commits**
   - Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.
   - Format: `<type>[optional scope]: <description>`
     ```plaintext
     feat[scope]: Add new CLI option

     BREAKING CHANGE: The `-x` flag was removed.
     ```
   - When user says "let's commit" or similar:
     - Only propose the commit message
     - Do not execute any git commands
     - Let the user handle the actual commit

## Core Foundation Rules

1. **Memory Management**
   - Safe handling of raw pointers
   - Proper memory management in test doubles
   - Prevent dangling pointers
   - Balance CFRetain/CFRelease calls
   - Use `system/core_foundation.rs` wrappers for additional safety

   Core Foundation Documentation:
   - [Memory Management Guide](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/CFMemoryMgmt.html)
   - [Object Lifecycle](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Articles/lifecycle.html)
   - [Ownership Policy](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html)
   - [Copy Functions](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/CopyFunctions.html)

2. **Core Foundation Types**
   - Type-safe wrappers for:
     - `CFType`
     - `CFString`
     - `CFArray`
     - `CFUrl`

3. **Mocks**
   - Track owned objects in mock structures
   - Retain objects you need to keep
   - Release objects when the mock is dropped
