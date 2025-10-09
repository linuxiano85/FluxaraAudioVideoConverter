# Contributing to NovaAudioVideoConverter

Thank you for considering contributing to NovaAudioVideoConverter! We welcome contributions from the community.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue on GitHub with:
- A clear and descriptive title
- Steps to reproduce the bug
- Expected behavior
- Actual behavior
- Your environment (Linux distribution, Python version, FFmpeg version)
- Screenshots if applicable

### Suggesting Features

We welcome feature suggestions! Please create an issue with:
- A clear and descriptive title
- Detailed description of the feature
- Use cases and examples
- Any relevant mockups or diagrams

### Pull Requests

1. **Fork the repository**
   ```bash
   git clone https://github.com/yourusername/NovaAudioVideoConverter.git
   cd NovaAudioVideoConverter
   ```

2. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**
   - Follow the existing code style
   - Add comments for complex logic
   - Update documentation if needed

4. **Test your changes**
   ```bash
   python3 examples.py
   python3 main.py
   ```

5. **Commit your changes**
   ```bash
   git add .
   git commit -m "Add feature: your feature description"
   ```

6. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request**
   - Go to the original repository
   - Click "New Pull Request"
   - Select your branch
   - Describe your changes

## Code Style Guidelines

### Python
- Follow PEP 8 style guide
- Use meaningful variable names
- Add docstrings to functions and classes
- Keep functions focused and small
- Use type hints where appropriate

Example:
```python
def convert_video(
    input_file: str,
    output_file: str,
    codec: str = 'h264'
) -> bool:
    """
    Convert video file to specified codec.
    
    Args:
        input_file: Path to input video file
        output_file: Path to output video file
        codec: Video codec to use (default: h264)
    
    Returns:
        True if conversion successful, False otherwise
    """
    # Implementation
    pass
```

### Documentation
- Update README.md if adding major features
- Update FEATURES.md for new capabilities
- Add examples to examples.py
- Use clear and concise language
- Include code examples

## Development Setup

### Prerequisites
```bash
# Install system dependencies
sudo apt install python3 python3-pip ffmpeg handbrake-cli libdvdcss2

# Install Python dependencies
pip3 install -r requirements.txt
```

### Running Tests
```bash
# Run examples to test functionality
python3 examples.py

# Test GUI
python3 main.py
```

## Areas for Contribution

We especially welcome contributions in these areas:

### High Priority
- [ ] Integration with OpenSubtitles API
- [ ] Advanced AI models for upscaling
- [ ] Better error handling and user feedback
- [ ] Unit tests and integration tests
- [ ] Performance optimizations
- [ ] Translation to other languages

### Medium Priority
- [ ] Custom device profiles UI
- [ ] Batch processing improvements
- [ ] Timeline editor for video editing
- [ ] Preset management system
- [ ] Plugin system for extensions

### Nice to Have
- [ ] Web interface
- [ ] CLI interface improvements
- [ ] Progress estimation improvements
- [ ] Theme customization
- [ ] Keyboard shortcuts

## Adding New Features

### Adding a New Format
Edit `src/core/formats.py`:
```python
VIDEO_FORMATS['new_format'] = {
    'codec': 'codec_name',
    'container': 'container_name',
    'description': 'Format Description'
}
```

### Adding a New Device Profile
Edit `src/profiles/devices.py`:
```python
DEVICE_PROFILES['device_id'] = {
    'name': 'Device Name',
    'resolution': '1920x1080',
    'video_codec': 'h264',
    'audio_codec': 'aac',
    'video_bitrate': '5M',
    'audio_bitrate': '192k',
    'container': 'mp4'
}
```

### Adding a New AI Filter
Create method in `src/ai/audio_enhancer.py` or `src/ai/video_enhancer.py`:
```python
def new_filter(
    self,
    input_file: str,
    output_file: str,
    parameter: float = 1.0
) -> bool:
    """
    Apply new filter to media file.
    
    Args:
        input_file: Input file path
        output_file: Output file path
        parameter: Filter parameter
    
    Returns:
        True if successful, False otherwise
    """
    cmd = [
        self.ffmpeg_path,
        '-i', input_file,
        '-vf', f'filter_name={parameter}',
        '-y', output_file
    ]
    
    try:
        subprocess.run(cmd, check=True, capture_output=True)
        return True
    except subprocess.CalledProcessError:
        return False
```

## Code Review Process

All Pull Requests will be reviewed by maintainers. We look for:
- Code quality and style
- Functionality and correctness
- Documentation completeness
- Test coverage
- Performance impact

## Community Guidelines

- Be respectful and inclusive
- Help others in the community
- Share knowledge and experience
- Give constructive feedback
- Focus on the issue, not the person

## Questions?

If you have questions about contributing:
- Open an issue with the "question" label
- Start a discussion on GitHub Discussions
- Check existing issues and discussions

## License

By contributing to NovaAudioVideoConverter, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to NovaAudioVideoConverter! 🎉
