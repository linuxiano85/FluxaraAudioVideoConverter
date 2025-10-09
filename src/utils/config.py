"""
Configuration module for NovaAudioVideoConverter
"""

import os
from pathlib import Path

# Application info
APP_NAME = "NovaAudioVideoConverter"
APP_VERSION = "1.0.0"
APP_AUTHOR = "NovaAudioVideoConverter Team"

# Paths
HOME_DIR = Path.home()
APP_DIR = HOME_DIR / ".novaaudiovideoconverter"
CONFIG_FILE = APP_DIR / "config.json"
PROFILES_DIR = APP_DIR / "profiles"
TEMP_DIR = APP_DIR / "temp"

# Create directories if they don't exist
for directory in [APP_DIR, PROFILES_DIR, TEMP_DIR]:
    directory.mkdir(parents=True, exist_ok=True)

# Default settings
DEFAULT_SETTINGS = {
    'output_directory': str(HOME_DIR / "Videos"),
    'hardware_acceleration': True,
    'preset': 'medium',
    'default_video_codec': 'h264',
    'default_audio_codec': 'aac',
    'default_video_bitrate': '5M',
    'default_audio_bitrate': '192k',
    'preserve_metadata': True,
    'overwrite_files': False,
    'theme': 'light',
    'language': 'en',
    'subtitle_language': 'en',
    'auto_search_subtitles': False,
    'log_level': 'INFO',
}

# FFmpeg settings
FFMPEG_GLOBAL_OPTIONS = [
    '-hide_banner',
    '-loglevel', 'info',
]

# Quality presets
QUALITY_PRESETS = {
    'low': {
        'video_bitrate': '1M',
        'audio_bitrate': '96k',
        'preset': 'fast'
    },
    'medium': {
        'video_bitrate': '3M',
        'audio_bitrate': '128k',
        'preset': 'medium'
    },
    'high': {
        'video_bitrate': '5M',
        'audio_bitrate': '192k',
        'preset': 'medium'
    },
    'ultra': {
        'video_bitrate': '10M',
        'audio_bitrate': '256k',
        'preset': 'slow'
    }
}

# Resolution presets
RESOLUTION_PRESETS = {
    '8K': '7680x4320',
    '4K': '3840x2160',
    '2K': '2560x1440',
    'Full HD': '1920x1080',
    'HD': '1280x720',
    'SD': '854x480',
    'DVD': '720x480',
}

# Supported operations
SUPPORTED_OPERATIONS = [
    'convert',
    'rip_dvd',
    'cut',
    'rotate',
    'crop',
    'merge',
    'enhance_audio',
    'enhance_video',
    'add_subtitle',
    'extract_audio',
    'extract_frames',
]

# AI Enhancement settings
AI_ENHANCEMENT_SETTINGS = {
    'audio_denoise_strength': 0.21,
    'video_denoise_strength': 5.0,
    'sharpen_amount': 1.0,
    'upscale_factor': 2,
    'stabilization_smoothing': 10,
    'color_contrast': 1.15,
    'color_brightness': 0.02,
    'color_saturation': 1.1,
}

# DVD settings
DVD_SETTINGS = {
    'default_title': 1,
    'remove_protection': True,
    'preset': 'Fast 1080p30',
    'backup_full_structure': False,
}

# Subtitle settings
SUBTITLE_SETTINGS = {
    'default_language': 'en',
    'font_size': 24,
    'search_timeout': 30,
    'max_results': 10,
}


def load_settings():
    """Load settings from config file"""
    import json
    
    if CONFIG_FILE.exists():
        try:
            with open(CONFIG_FILE, 'r') as f:
                return {**DEFAULT_SETTINGS, **json.load(f)}
        except (json.JSONDecodeError, IOError):
            return DEFAULT_SETTINGS.copy()
    return DEFAULT_SETTINGS.copy()


def save_settings(settings):
    """Save settings to config file"""
    import json
    
    try:
        with open(CONFIG_FILE, 'w') as f:
            json.dump(settings, f, indent=2)
        return True
    except IOError:
        return False


def get_temp_file(extension='tmp'):
    """Get temporary file path"""
    import uuid
    filename = f"nova_{uuid.uuid4().hex}.{extension}"
    return TEMP_DIR / filename


def cleanup_temp_files():
    """Clean up temporary files"""
    import shutil
    
    if TEMP_DIR.exists():
        shutil.rmtree(TEMP_DIR)
        TEMP_DIR.mkdir(parents=True, exist_ok=True)
