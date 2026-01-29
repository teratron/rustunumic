# Project Context

## Project Overview

## Core Principles

### Language Requirements

Code and documentation language:

- All code, comments, documentation, variable names, function names, class names, method names, attribute names, and technical terms must be in English
- Maintain English as the primary language for all technical elements including error messages, log entries, configuration keys, and API responses to ensure readability and maintainability
- Technical documentation, inline comments, docstrings, and README files must be written in English
- All commit messages, pull request descriptions, and issue titles related to code changes should be in English

Communication style:

- Explanations and discussions in the chat interface should be in Russian
- Use Russian for conversational responses, clarifications, project planning, and non-technical interactions
- Project management communications, feature discussions, and strategic decisions should be conducted in Russian
- Code review comments and technical discussions during development can be in Russian unless collaborating with English-speaking developers

### Performance Benchmarks

All components must meet defined performance benchmarks: Maximum response times for user interactions (sub-200ms for simple operations), Efficient resource utilization (memory, CPU, network), Scalability under expected load conditions, Optimized algorithms and data structures for performance-critical paths.

Performance requirements also include: Regular performance profiling to identify bottlenecks, Establishing baseline metrics for key operations, Monitoring resource usage under various load conditions, Implementation of caching strategies where appropriate, Continuous performance testing in CI/CD pipeline

### Quality Maintenance

Quality must be maintained throughout the development lifecycle: Automated quality checks on all commits, Regular refactoring to maintain code health, Continuous monitoring of performance metrics, Regular security assessments and updates.

### OOP Principle

"Object-Oriented Programming" - All code must follow OOP principles: Encapsulation to hide internal state and implementation details, Inheritance to promote code reuse and create hierarchical relationships, Polymorphism to allow objects of different types to be treated uniformly, and Abstraction to focus on behavior rather than implementation details. This ensures maintainable and scalable code design.

### SOLID Principles

Classes, methods, functions and modules must follow the SOLID principles: Single Responsibility Principle (each class/module has one reason to change), Open/Closed Principle (software entities should be open for extension but closed for modification), Liskov Substitution Principle (objects should be replaceable with instances of their subtypes), Interface Segregation Principle (clients should not be forced to depend on interfaces they don't use), Dependency Inversion Principle (high-level modules should not depend on low-level modules, both should depend on abstractions).

### DRY Principle

"Do not Repeat Yourself" - Code duplication must be eliminated and each piece of knowledge must have a single authoritative representation in the system. All shared functionality must be extracted into reusable components, functions, or modules to ensure a single source of truth and reduce maintenance overhead.

### KISS Principle

"Keep It Simple, Stupid" - Code and architectural solutions must maintain simplicity and avoid unnecessary complexity. Before implementing complex solutions, evaluate if a simpler approach would be equally effective. Simple code is easier to understand, maintain, test, and debug.

### YAGNI Principle

"You Ain't Gonna Need It" - Only implement functionality that is currently needed, not anticipated future needs. Avoid adding features or infrastructure for potential future use cases that are not immediately required. This prevents code bloat and reduces maintenance burden.

### Code Style Guidelines

## Architecture & Core Components

### Main Structure

```text

```

## Building and Running

## Documentation Requirements

## Versioning Requirements

Each significant functional change must update the project version according to the Semantic Versioning (SemVer) convention. Changes must be applied to all files where the version is mentioned: `Cargo.toml`, `README.md`, `docs/` (where applicable) and any other relevant files. When updating versions, dependencies from external libraries, documentation updates, API changes, and backward compatibility must be taken into account. Versioning must strictly follow the major.minor.patch rules with corresponding updates in changelog and release tags. Major version updates indicate backward incompatible changes, minor version updates indicate new functionality in a backward compatible manner, and patch version updates indicate backward compatible bug fixes.

## Template File Requirements

## Rule Validation

All rules and principles in this constitution must be systematically checked and validated during development. The AI agent and development team must ensure compliance with every rule before implementation proceeds. Regular validation of rule adherence must occur to maintain consistency and quality across all project components.
