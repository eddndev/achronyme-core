# Contributing to Achronyme Core

Thank you for your interest in contributing to Achronyme Core! This document provides guidelines and instructions for contributing.

## 🚀 Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Emscripten SDK** (latest version)
  ```bash
  git clone https://github.com/emscripten-core/emsdk.git
  cd emsdk
  ./emsdk install latest
  ./emsdk activate latest
  source ./emsdk_env.sh
  ```

- **CMake** >= 3.20
- **Node.js** >= 18
- **C++20** compatible compiler (GCC 11+, Clang 13+, or MSVC 2022+)
- **Git**

### Setting Up Your Development Environment

1. **Fork the repository** on GitHub

2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/achronyme-core.git
   cd achronyme-core
   ```

3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/eddndev/achronyme-core.git
   ```

4. **Install dependencies**:
   ```bash
   npm install
   ```

5. **Build the project**:
   ```bash
   npm run build
   ```

6. **Run tests**:
   ```bash
   npm test
   ```

---

## 📋 Development Workflow

### 1. Create a Branch

Always create a new branch for your work:

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-123
```

### 2. Make Your Changes

- Write clean, readable code
- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run TypeScript tests
npm test

# Run C++ tests (native build)
npm run test:cpp

# Build WASM
npm run build:wasm
```

### 4. Commit Your Changes

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
git commit -m "feat: add support for sin/cos functions"
git commit -m "fix: resolve parser precedence issue"
git commit -m "docs: update API documentation"
git commit -m "test: add tests for complex numbers"
```

**Commit types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Build process or tooling changes

### 5. Push and Create a Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub with:
- Clear title and description
- Reference any related issues
- List of changes made
- Test results

---

## 🏗️ Project Structure

```
achronyme-core/
├── wasm/               # C++ source code
│   ├── src/
│   │   ├── core/       # Core types (Value, Complex, Vector)
│   │   ├── parser/     # Lexer, Parser, Evaluator
│   │   ├── dsp/        # Digital Signal Processing (Phase 5+)
│   │   ├── linalg/     # Linear Algebra (Phase 5+)
│   │   └── bindings/   # Emscripten bindings
│   └── tests/          # C++ tests (Google Test)
│
├── js/                 # TypeScript/JavaScript wrapper
│   ├── src/            # Source files
│   └── __tests__/      # TypeScript tests (Vitest)
│
├── scripts/            # Build scripts
├── docs/               # Documentation
└── dist/               # Build output (gitignored)
```

---

## 🎨 Code Style

### C++ Style

- **Indentation**: 4 spaces
- **Naming**:
  - Classes: `PascalCase` (e.g., `SuperiorOrderCalculator`)
  - Functions: `camelCase` (e.g., `evaluateExpression`)
  - Variables: `snake_case` (e.g., `token_count`)
  - Constants: `UPPER_SNAKE_CASE` (e.g., `MAX_TOKENS`)
- **Headers**: Use `#pragma once` or include guards
- **Namespace**: Always use `namespace achronyme`

Example:
```cpp
namespace achronyme {
namespace parser {

class Lexer {
public:
    std::vector<Token> tokenize();

private:
    size_t current_position_;
};

} // namespace parser
} // namespace achronyme
```

### TypeScript Style

- **Indentation**: 2 spaces
- **Naming**: Follow TypeScript conventions
- **Types**: Use TypeScript types, avoid `any`
- **Exports**: Use named exports

Example:
```typescript
export class SOC {
  private module: AchronymeModule | null = null;

  async init(): Promise<void> {
    // Implementation
  }
}
```

---

## 🧪 Testing Guidelines

### Writing C++ Tests

Use Google Test framework:

```cpp
#include <gtest/gtest.h>
#include "../src/parser/evaluator.hpp"

TEST(EvaluatorTest, BasicAddition) {
    EXPECT_DOUBLE_EQ(eval("2 + 3"), 5.0);
}
```

### Writing TypeScript Tests

Use Vitest:

```typescript
import { describe, it, expect } from 'vitest';
import { SOC } from '../src/index';

describe('SOC', () => {
  it('should evaluate addition', () => {
    expect(soc.eval('2 + 3')).toBe(5);
  });
});
```

---

## 📚 Documentation

When adding new features:

1. **Update README.md** if it affects the public API
2. **Add JSDoc comments** to TypeScript code
3. **Add Doxygen comments** to C++ code
4. **Update CHANGELOG.md**

Example documentation:

```cpp
/**
 * Evaluate a mathematical expression
 *
 * @param expression String expression to evaluate
 * @return Evaluated result as Value
 *
 * @example
 *   evaluate("2 + 3 * 4")  // Returns 14
 */
core::Value evaluate(const std::string& expression);
```

---

## 🐛 Reporting Bugs

When reporting bugs, please include:

1. **Description**: Clear description of the issue
2. **Steps to reproduce**: Minimal code example
3. **Expected behavior**: What should happen
4. **Actual behavior**: What actually happens
5. **Environment**:
   - OS and version
   - Node.js version
   - Emscripten version (if applicable)
   - Browser (if applicable)

---

## 💡 Feature Requests

We welcome feature requests! Please:

1. Check if the feature is already planned (see [Roadmap](README.md#roadmap))
2. Open an issue with the `enhancement` label
3. Describe the feature and its use case
4. Discuss implementation approach if applicable

---

## 🎯 Areas for Contribution

### High Priority
- **Phase 2: Mathematical Functions** - Implement sin, cos, exp, log, etc.
- **Error Handling** - Better error messages and recovery
- **Performance Optimization** - WASM optimizations, SIMD support

### Medium Priority
- **Phase 3: Complex Numbers** - Implement complex number type
- **Phase 3: Vectors** - Implement vector type and operations
- **Documentation** - Improve API docs and examples

### Low Priority
- **Benchmarking** - Add performance benchmarks
- **CI/CD** - Improve GitHub Actions workflows
- **Tooling** - Better development tools

---

## 📜 Code of Conduct

Be respectful, inclusive, and professional. We're all here to learn and build something great together.

---

## ❓ Questions?

If you have questions:

- Open a GitHub Discussion
- Check existing issues
- Contact: contacto@eddndev.com

---

Thank you for contributing to Achronyme Core! 🚀
