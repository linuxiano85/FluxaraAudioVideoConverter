# Fluxara Visual Identity

## Brand Overview

**Fluxara** represents a modern Linux-first suite for analog media restoration and conversion. The name evokes the fluid, flowing nature of signal processing and the transformation of analog media into digital formats.

## Color Palette

### Primary Colors

- **Indigo 600** (`#4F46E5`) - Primary brand color, represents trust and technology
- **Violet 600** (`#7C3AED`) - Secondary brand color, represents creativity and transformation

### Gradient

The signature Fluxara gradient flows from Indigo 600 to Violet 600, symbolizing the transformation journey from analog to digital.

```css
background: linear-gradient(135deg, #4F46E5 0%, #7C3AED 100%);
```

### Supporting Colors

- **Slate 900** (`#0F172A`) - Dark backgrounds
- **Slate 700** (`#334155`) - Secondary text
- **Slate 100** (`#F1F5F9`) - Light backgrounds
- **White** (`#FFFFFF`) - Primary text on dark backgrounds

### Accent Colors

- **Cyan 500** (`#06B6D4`) - Success, active states
- **Red 500** (`#EF4444`) - Errors, warnings
- **Amber 500** (`#F59E0B`) - Warnings, attention

## Typography

### Typeface

- **Headings**: Inter, system-ui, sans-serif
- **Body**: Inter, system-ui, sans-serif
- **Monospace**: JetBrains Mono, Fira Code, monospace

### Scale

- **Heading 1**: 2.25rem (36px) / Bold
- **Heading 2**: 1.875rem (30px) / Semibold
- **Heading 3**: 1.5rem (24px) / Semibold
- **Body**: 1rem (16px) / Regular
- **Small**: 0.875rem (14px) / Regular
- **Code**: 0.875rem (14px) / Monospace

## Icon Style

### Design Principles

- **Monoline**: 2px stroke weight for 24×24px icons
- **Rounded**: All caps and line endings use round caps
- **Minimal**: Clean, simple shapes
- **Consistent**: 24×24px grid with 2px padding

### Icon Colors

- Use `stroke="currentColor"` for adaptable icons
- Support both light and dark themes
- Fill only when necessary for emphasis

### Application Icon

The main app icon combines:
- Circular background with the Fluxara gradient
- Waveform symbol (representing audio)
- Film strip element (representing video)
- White accents for contrast

## UI Elements

### Buttons

- **Primary**: Gradient background, white text
- **Secondary**: Transparent with colored border
- **Ghost**: Transparent, colored text only

### Cards

- **Light mode**: White background, subtle shadow
- **Dark mode**: Slate 800 background, border accent

### Progress Indicators

- Use Cyan 500 for active progress
- Use gradient for completion states

## Linux Desktop Integration

### Desktop Entry

- Use the gradient app icon
- Category: AudioVideo;Audio;Video;Multimedia
- Keywords: converter, ffmpeg, analog, vhs, restoration

### Notifications

- Use brand colors for notification badges
- Include appropriate icon for context

## Future GUI Concept

See [docs/ui/GUI-concept.md](../ui/GUI-concept.md) for mockups and detailed interface design.

## Asset Locations

- **SVG Icons**: `/icons/svg/`
- **Application Icon**: `/icons/svg/app-icon.svg`
- **Feature Icons**: `/icons/svg/[feature-name].svg`

## Usage Guidelines

### Do

- ✅ Use the gradient for primary branding
- ✅ Maintain consistent stroke weights in icons
- ✅ Use `currentColor` for theme-adaptive icons
- ✅ Follow Linux desktop conventions

### Don't

- ❌ Don't modify the gradient colors
- ❌ Don't use gradients on text (except logos)
- ❌ Don't mix icon styles
- ❌ Don't use Windows-specific design patterns

## Accessibility

- All color combinations meet WCAG AA standards
- Minimum contrast ratio of 4.5:1 for body text
- Minimum contrast ratio of 3:1 for large text
- Icons include accessible labels in UI

---

**Version**: 1.0  
**Last Updated**: 2025-10  
**Maintainer**: Fluxara Team
