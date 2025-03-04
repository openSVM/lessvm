# IDELess Screenshots Directory

This directory contains screenshots used in the IDELess documentation, README, and other project materials.

## Purpose

The screenshots in this directory serve several purposes:

1. **Documentation**: Illustrate features and workflows in user guides and documentation
2. **README**: Provide visual examples in the project README
3. **Website**: Supply images for the project website
4. **Marketing**: Offer visual assets for presentations and promotional materials

## Screenshot Guidelines

When adding screenshots to this directory, please follow these guidelines:

### Naming Convention

Use descriptive, lowercase names with hyphens separating words:

- `main-interface.png`: Overview of the main IDELess interface
- `editor-syntax-highlighting.png`: Example of syntax highlighting in the editor
- `debugger-breakpoints.png`: Debugger with breakpoints set
- `simulator-gas-analysis.png`: Gas usage analysis in the simulator
- `deployment-configuration.png`: Deployment configuration panel

### Resolution and Format

- **Resolution**: Capture at 1920x1080 or higher resolution
- **Format**: Use PNG for interface screenshots (preferred) or JPEG for photos
- **Size**: Optimize images to keep file sizes reasonable (< 500KB when possible)
- **Aspect Ratio**: Maintain the original aspect ratio; do not stretch images

### Content Guidelines

- **Clean Interface**: Ensure the interface is clean and focused on the feature being showcased
- **Sample Data**: Use professional, appropriate sample data (no placeholder text like "foo", "bar")
- **Personal Information**: Remove any personal information, API keys, or sensitive data
- **Clarity**: Make sure text is readable and important elements are clearly visible
- **Consistency**: Maintain consistent theme, zoom level, and window size across related screenshots

### Accessibility

- **Contrast**: Ensure sufficient contrast between text and background
- **Size**: Make sure important elements are large enough to be clearly visible
- **Focus**: Highlight the relevant area if the screenshot contains many elements

## Directory Structure

```
screenshots/
├── interface/             # General interface screenshots
│   ├── main-interface.png
│   ├── dark-theme.png
│   └── light-theme.png
├── editor/                # Code editor screenshots
│   ├── syntax-highlighting.png
│   ├── code-completion.png
│   └── error-checking.png
├── debugger/              # Debugger screenshots
│   ├── breakpoints.png
│   ├── variable-inspection.png
│   └── call-stack.png
├── simulator/             # Simulator screenshots
│   ├── execution-visualization.png
│   ├── gas-analysis.png
│   └── performance-metrics.png
├── deployment/            # Deployment screenshots
│   ├── network-selection.png
│   ├── configuration.png
│   └── monitoring.png
├── ai-assistant/          # AI Assistant screenshots
│   ├── code-suggestions.png
│   ├── error-resolution.png
│   └── learning-resources.png
└── workflows/             # Workflow examples
    ├── create-project.png
    ├── debug-session.png
    └── deployment-process.png
```

## Taking Screenshots

### Windows

- **Windows Snipping Tool**: Press `Windows + Shift + S` to open the snipping tool, select the area, and save
- **Print Screen**: Press `Print Screen` to capture the entire screen, or `Alt + Print Screen` for the active window
- **Windows Game Bar**: Press `Windows + G` to open Game Bar, then use the capture button

### macOS

- **Screenshot Utility**: Press `Command + Shift + 5` to open the screenshot utility
- **Capture Entire Screen**: Press `Command + Shift + 3`
- **Capture Selected Area**: Press `Command + Shift + 4`
- **Capture Specific Window**: Press `Command + Shift + 4`, then press `Space`

### Linux

- **GNOME Screenshot**: Press `Print Screen` to capture the entire screen
- **Selected Area**: Press `Shift + Print Screen` to capture a selected area
- **Specific Window**: Press `Alt + Print Screen` to capture the active window
- **Screenshot Tool**: Use the screenshot tool included with your desktop environment

## Updating Screenshots

When updating screenshots due to UI changes or feature updates:

1. Try to match the same content and context as the original screenshot
2. Use the same naming convention to replace the old file
3. Update all screenshots affected by the change for consistency
4. Document significant UI changes in the commit message

## Using Screenshots in Documentation

When referencing screenshots in documentation:

```markdown
![Description of the image](./screenshots/category/image-name.png)
```

For the main README and other documents at the root level:

```markdown
![Description of the image](./screenshots/category/image-name.png)
```

## Placeholder Images

This directory includes some placeholder images that should be replaced with actual screenshots as the UI is developed:

- `interface/main-interface.png`: Replace with actual main interface
- `editor/syntax-highlighting.png`: Replace with actual syntax highlighting example
- `debugger/breakpoints.png`: Replace with actual debugger with breakpoints
- `simulator/gas-analysis.png`: Replace with actual gas analysis visualization
- `deployment/configuration.png`: Replace with actual deployment configuration panel

## Contributing Screenshots

If you're contributing screenshots to the project:

1. Follow the naming and quality guidelines above
2. Place screenshots in the appropriate subdirectory
3. Optimize images before committing
4. Include screenshot updates in the same PR as related code changes
5. Mention updated or added screenshots in your PR description

## License

All screenshots in this directory are subject to the same license as the IDELess project itself. By contributing screenshots, you agree to license them under the project's license terms.