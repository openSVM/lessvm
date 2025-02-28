# Contributing to LessVM IDELess

Thank you for your interest in contributing to the LessVM IDELess project! This document provides guidelines and instructions for contributing to the development of IDELess.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Environment](#development-environment)
- [Project Structure](#project-structure)
- [Contribution Workflow](#contribution-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Issue Reporting](#issue-reporting)
- [Pull Requests](#pull-requests)
- [Review Process](#review-process)
- [Community](#community)

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Please read it before contributing.

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- Node.js (v16.x or higher)
- npm (v8.x or higher)
- Git

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```
   git clone https://github.com/YOUR-USERNAME/ideless.git
   cd ideless
   ```
3. Add the original repository as an upstream remote:
   ```
   git remote add upstream https://github.com/lessvm/ideless.git
   ```

## Development Environment

### Setup

1. Install dependencies:
   ```
   npm install
   ```

2. Start the development server:
   ```
   npm run dev
   ```

3. Build for production:
   ```
   npm run build
   ```

### Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run test` - Run tests
- `npm run lint` - Run linter
- `npm run format` - Format code with Prettier
- `npm run storybook` - Start Storybook for component development

## Project Structure

The IDELess project follows this structure:

```
ideless/
├── public/             # Static assets
├── src/                # Source code
│   ├── components/     # React components
│   │   ├── ai/         # AI Assistant components
│   │   ├── common/     # Common UI components
│   │   ├── debugger/   # Debugger components
│   │   ├── editor/     # Code editor components
│   │   ├── layout/     # Layout components
│   │   └── simulator/  # Simulator components
│   ├── hooks/          # Custom React hooks
│   ├── layouts/        # Page layouts
│   ├── pages/          # Page components
│   ├── services/       # Service modules
│   ├── store/          # Redux store
│   │   ├── slices/     # Redux slices
│   │   └── index.ts    # Store configuration
│   ├── styles/         # Global styles
│   ├── types/          # TypeScript type definitions
│   ├── utils/          # Utility functions
│   ├── App.tsx         # Main App component
│   ├── index.css       # Global CSS
│   └── main.tsx        # Entry point
├── tests/              # Test files
├── .eslintrc.js        # ESLint configuration
├── .prettierrc         # Prettier configuration
├── index.html          # HTML template
├── package.json        # Project dependencies
├── tsconfig.json       # TypeScript configuration
└── vite.config.ts      # Vite configuration
```

## Contribution Workflow

### Branching Strategy

We use a feature branch workflow:

1. Ensure your fork is up to date:
   ```
   git fetch upstream
   git checkout main
   git merge upstream/main
   ```

2. Create a new branch for your feature or bugfix:
   ```
   git checkout -b feature/your-feature-name
   ```
   or
   ```
   git checkout -b fix/issue-you-are-fixing
   ```

3. Make your changes, commit them, and push to your fork:
   ```
   git add .
   git commit -m "Your descriptive commit message"
   git push origin feature/your-feature-name
   ```

4. Create a pull request from your branch to the main repository.

### Commit Messages

Follow these guidelines for commit messages:

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line
- Consider starting the commit message with an applicable emoji:
  - ✨ `:sparkles:` for new features
  - 🐛 `:bug:` for bug fixes
  - 📚 `:books:` for documentation changes
  - ♻️ `:recycle:` for refactoring code
  - 🧪 `:test_tube:` for adding tests
  - 🎨 `:art:` for improving structure/format of the code
  - ⚡ `:zap:` for performance improvements

## Coding Standards

### TypeScript

- Use TypeScript for all new code
- Define explicit types (avoid `any` when possible)
- Use interfaces for object shapes
- Use proper type imports/exports

### React

- Use functional components with hooks
- Use React.memo for performance optimization when appropriate
- Follow the React component file structure:
  ```tsx
  // Imports
  import React from 'react';
  import './ComponentName.css';
  
  // Types
  interface ComponentNameProps {
    // props definition
  }
  
  // Component
  export const ComponentName: React.FC<ComponentNameProps> = ({ prop1, prop2 }) => {
    // hooks
    
    // handlers
    
    // render
    return (
      <div className="component-name">
        {/* JSX content */}
      </div>
    );
  };
  ```

### CSS/Styling

- Use Tailwind CSS for styling
- Follow BEM naming convention for custom CSS classes
- Keep component-specific styles in the same directory as the component

### State Management

- Use Redux with Redux Toolkit for global state
- Use React's Context API for component-specific state
- Use local state (useState) for UI-only state

## Testing Guidelines

### Test Structure

- Place tests in the `tests` directory
- Name test files with `.test.ts` or `.test.tsx` extension
- Group related tests with `describe` blocks
- Use clear test descriptions with `it` or `test`

### Testing Requirements

- Write unit tests for all new features
- Ensure tests are deterministic (no random failures)
- Mock external dependencies
- Aim for high test coverage, especially for critical paths

### Running Tests

```
npm run test
```

To run tests in watch mode:

```
npm run test:watch
```

## Documentation

### Code Documentation

- Document all public functions, classes, and interfaces
- Use JSDoc format for documentation
- Explain complex algorithms or business logic
- Include examples for non-obvious usage

### Component Documentation

- Use Storybook for component documentation
- Include examples of different component states
- Document props and their purpose
- Provide usage guidelines

### Running Storybook

```
npm run storybook
```

## Issue Reporting

### Before Submitting an Issue

- Check if the issue already exists
- Determine if it's a bug, feature request, or question
- Gather relevant information about your environment

### Issue Template

When submitting an issue, please include:

1. **Type**: Bug, feature request, or question
2. **Description**: Clear description of the issue
3. **Steps to Reproduce** (for bugs): Detailed steps to reproduce the issue
4. **Expected Behavior**: What you expected to happen
5. **Actual Behavior**: What actually happened
6. **Environment**: Browser, OS, Node.js version, etc.
7. **Screenshots/Logs**: If applicable
8. **Possible Solution**: If you have suggestions

## Pull Requests

### PR Template

When submitting a pull request, please include:

1. **Description**: What the PR accomplishes
2. **Related Issue**: Link to the issue this PR addresses
3. **Type of Change**: New feature, bug fix, documentation, etc.
4. **Testing**: How you tested the changes
5. **Screenshots**: If applicable
6. **Checklist**:
   - [ ] Code follows the style guidelines
   - [ ] Tests for the changes have been added
   - [ ] Documentation has been updated
   - [ ] Changes generate no new warnings

### PR Size

- Keep PRs focused on a single change
- Split large changes into multiple PRs
- Aim for PRs that can be reviewed in under 30 minutes

## Review Process

### Code Review Guidelines

- Be respectful and constructive
- Focus on the code, not the person
- Explain your reasoning
- Suggest alternatives when rejecting a change

### Approval Process

1. All PRs require at least one approval from a maintainer
2. CI checks must pass
3. All review comments must be resolved
4. Documentation must be updated if necessary

## Community

### Communication Channels

- **GitHub Issues**: For bug reports and feature requests
- **Discord**: For real-time discussion and community support
- **Weekly Meetings**: For project planning and discussion

### Recognition

Contributors will be recognized in the following ways:

- Listed in the CONTRIBUTORS.md file
- Mentioned in release notes for significant contributions
- Invited to become maintainers after consistent contributions

## Special Considerations for LessVM Development

### OpenSVM RPC Servers

- Always use OpenSVM RPC servers as the default for all Solana blockchain interactions
- Never change this default to any other RPC provider
- Document any RPC-specific code clearly

### Performance Considerations

- Be mindful of performance implications, especially for large files
- Optimize rendering to prevent UI lag
- Consider the impact on memory usage
- Test with realistic workloads

### Accessibility

- Ensure all UI components are accessible
- Follow WCAG AA standards at minimum
- Test with screen readers
- Support keyboard navigation

## Thank You!

Your contributions help make IDELess better for everyone. We appreciate your time and effort!