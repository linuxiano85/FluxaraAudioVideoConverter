# The Name "Fluxara" – Decision and Rationale

## Why Fluxara?

The name **Fluxara** was chosen to represent the comprehensive suite of analog media restoration and conversion tools, with a particular focus on Linux-first development.

## Etymology

- **Flux**: From Latin "fluxus" meaning "flow" or "flowing"
  - Represents the continuous flow of signal processing
  - Evokes the transformation from analog to digital
  - Suggests fluidity and adaptation
  
- **-ara**: Suffix suggesting capability, instrumentality, or a suite of tools
  - Creates a memorable, pronounceable name
  - Distinguishes from generic "converter" terminology
  - Implies a comprehensive solution

## Previous Name

This project was previously known as **NovaAudioVideoConverter**. The rebrand to Fluxara (repository now at `linuxiano85/FluxaraAudioVideoConverter`) reflects:

1. **Expanded Vision**: Moving beyond generic conversion to specialized analog restoration
2. **Suite Identity**: Positioning for future expansion (fluxara-avc, fluxara-capture, etc.)
3. **Professional Positioning**: Establishing a distinct brand in the open-source multimedia space
4. **Linux-First Focus**: Emphasizing the platform and community we serve

## Naming Scheme

### Current Project

- **Package Name**: `fluxara-avc`
- **Binary Name**: `fluxara-avc`
- **Full Name**: Fluxara AVC (Audio/Video Converter)

### Future Components

The Fluxara suite will follow the pattern: `fluxara-[component]`

Planned components:
- `fluxara-avc` – Audio/Video Converter (current project)
- `fluxara-capture` – Real-time capture interface for V4L2/ALSA
- `fluxara-enhance` – Specialized restoration filters and denoisers
- `fluxara-gui` – Unified graphical interface for all components

## Design Principles

### Linux-First

- Native GTK4/Libadwaita integration
- Following GNOME HIG and Freedesktop standards
- Package formats: Flatpak (primary), AppImage, DEB, RPM
- No Windows/macOS support in core roadmap

### Analog Focus

While Fluxara can handle any media conversion, its special focus is:
- VHS tape digitization
- Audio cassette restoration
- Vinyl/record capture cleanup
- Analog signal processing and enhancement

### FFmpeg Foundation

Built on FFmpeg, Fluxara provides:
- Curated presets for analog media
- Simplified command-line interface
- Future GUI with visual feedback
- Pipeline optimization for Linux hardware

## Rebrand Rollout

### Phase 1: Repository and Core (This PR)

- ✅ Update package and binary names
- ✅ Update all user-facing strings
- ✅ Create new visual identity
- ✅ Document rebrand rationale
- ⏳ Repository rename (post-merge, by owner)

### Phase 2: Package Distribution

- Update Flatpak manifest
- Update DEB package metadata
- Update RPM spec file
- Update AppImage build
- Submit to flathub.org (if applicable)

### Phase 3: Documentation and Web Presence

- Update all documentation
- Create project website (optional)
- Update package manager listings
- Announce rebrand in community channels

## Repository Naming

### Current

- GitHub: `linuxiano85/FluxaraAudioVideoConverter` (previously `linuxiano85/NovaAudioVideoConverter`)
- Will be renamed to: `linuxiano85/fluxara-avc`

### After Rename

GitHub automatically creates redirects, so:
- Old URLs will redirect to new location
- Existing clones will continue to work
- Git remotes will be updated automatically

### URL Update Strategy

After repository rename:
- Minor PR to update hardcoded URLs in documentation
- Most references use relative paths and don't need changes
- CI/CD badges will auto-update

## Brand Communication

### Elevator Pitch

> Fluxara AVC is a Linux-first analog restoration and conversion toolkit built on FFmpeg, specializing in VHS rescue and vintage media digitization.

### Tagline Options

- "Linux-first analog restoration"
- "Transform. Restore. Preserve."
- "Your analog media deserves better"

### Key Messages

1. **Specialized**: Not just another converter—focused on analog media rescue
2. **Linux-Native**: Built for Linux, by Linux users
3. **FFmpeg-Powered**: Standing on the shoulders of the best media framework
4. **Open Source**: MIT licensed, community-driven

## Community Response

We anticipate questions about the rebrand:

**Q: Why change from Nova?**  
A: Nova was too generic. Fluxara better represents our focus on analog restoration and positions us for future expansion as a suite.

**Q: Will my old bookmarks/links work?**  
A: Yes! GitHub creates automatic redirects when repositories are renamed.

**Q: Is Windows support planned?**  
A: No. Fluxara is Linux-first by design. Our resources focus on making the best Linux experience possible.

**Q: What happens to PR #3?**  
A: This rebrand PR is independent of PR #3's functional changes. Both can coexist and merge in any order.

## References

- **Visual Identity**: [IDENTITY.md](IDENTITY.md)
- **GUI Concept**: [../ui/GUI-concept.md](../ui/GUI-concept.md)
- **Original Issue**: GitHub PR for rebrand implementation

---

**Decision Date**: 2025-10  
**Effective Date**: Upon merge of rebrand PR  
**Repository Rename**: Post-merge (by repository owner)  
**Authors**: Fluxara Team
