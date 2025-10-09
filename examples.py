#!/usr/bin/env python3
"""
NovaAudioVideoConverter - Example Usage
This script demonstrates various features of the converter
"""

import sys
import os

# Add src to path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from src.core.converter import Converter
from src.core.dvd_ripper import DVDRipper
from src.core.formats import get_supported_formats, is_video_format, is_audio_format
from src.ai.audio_enhancer import AudioEnhancer
from src.ai.video_enhancer import VideoEnhancer
from src.profiles.devices import get_all_device_profiles, list_device_categories
from src.utils.subtitle_search import SubtitleSearcher
from src.utils.helpers import format_file_size, format_duration, is_tool_installed


def print_section(title):
    """Print section header"""
    print("\n" + "=" * 60)
    print(f"  {title}")
    print("=" * 60)


def example_format_info():
    """Display supported formats information"""
    print_section("Supported Formats (180+)")
    
    formats = get_supported_formats()
    print(f"\nTotal formats supported: {len(formats)}")
    
    video_formats = [f for f in formats if is_video_format(f)]
    audio_formats = [f for f in formats if is_audio_format(f)]
    
    print(f"Video formats: {len(video_formats)}")
    print(f"Audio formats: {len(audio_formats)}")
    
    print(f"\nSample video formats: {', '.join(video_formats[:20])}")
    print(f"Sample audio formats: {', '.join(audio_formats[:20])}")


def example_device_profiles():
    """Display device profiles information"""
    print_section("Device Profiles (200+)")
    
    profiles = get_all_device_profiles()
    print(f"\nTotal device profiles: {len(profiles)}")
    
    categories = list_device_categories()
    for category, devices in categories.items():
        if devices:
            print(f"\n{category} ({len(devices)} profiles):")
            for device_id, name in devices[:5]:
                print(f"  - {name}")
            if len(devices) > 5:
                print(f"  ... and {len(devices) - 5} more")


def example_conversion():
    """Example: Basic conversion"""
    print_section("Example: Video Conversion")
    
    converter = Converter()
    
    # Check if FFmpeg is installed
    if not converter.check_ffmpeg_installed():
        print("\n❌ FFmpeg is not installed!")
        print("Install FFmpeg to use conversion features:")
        print("  Ubuntu/Debian: sudo apt install ffmpeg")
        print("  Fedora: sudo dnf install ffmpeg")
        print("  Arch Linux: sudo pacman -S ffmpeg")
        return
    
    print("\n✅ FFmpeg is installed and ready")
    print("\nExample conversion command:")
    print("""
    converter = Converter()
    converter.convert(
        input_file='input.avi',
        output_file='output.mp4',
        video_codec='h264',
        audio_codec='aac',
        video_bitrate='5M',
        audio_bitrate='192k',
        resolution='1920x1080',
        preset='medium',
        hwaccel=True
    )
    """)


def example_dvd_ripper():
    """Example: DVD ripping"""
    print_section("Example: DVD Ripping")
    
    ripper = DVDRipper()
    tools = ripper.check_tools_installed()
    
    print("\nDVD Tools Status:")
    for tool, installed in tools.items():
        status = "✅ Installed" if installed else "❌ Not installed"
        print(f"  {tool}: {status}")
    
    print("\nExample DVD ripping command:")
    print("""
    ripper = DVDRipper()
    ripper.rip_dvd(
        dvd_path='/dev/dvd',  # or path to ISO file
        output_file='movie.mp4',
        title=1,
        preset='Fast 1080p30',
        remove_protection=True
    )
    """)


def example_video_editing():
    """Example: Video editing"""
    print_section("Example: Video Editing")
    
    print("\n1. Cut/Trim Video:")
    print("""
    converter.cut_video(
        input_file='input.mp4',
        output_file='output.mp4',
        start_time='00:01:30',
        duration='00:05:00',
        lossless=True
    )
    """)
    
    print("\n2. Rotate Video:")
    print("""
    converter.rotate_video(
        input_file='input.mp4',
        output_file='output.mp4',
        rotation=90  # 90, 180, or 270 degrees
    )
    """)
    
    print("\n3. Crop Video:")
    print("""
    converter.crop_video(
        input_file='input.mp4',
        output_file='output.mp4',
        width=1280,
        height=720,
        x=0,
        y=0
    )
    """)
    
    print("\n4. Merge Videos (Lossless):")
    print("""
    converter.merge_files(
        input_files=['part1.mp4', 'part2.mp4', 'part3.mp4'],
        output_file='merged.mp4',
        lossless=True
    )
    """)


def example_ai_enhancement():
    """Example: AI enhancement"""
    print_section("Example: AI Enhancement")
    
    print("\nAudio Enhancement:")
    print("""
    audio_enhancer = AudioEnhancer()
    
    # Remove noise
    audio_enhancer.denoise_audio('input.mp3', 'denoised.mp3')
    
    # Enhance voice
    audio_enhancer.enhance_voice('input.mp3', 'enhanced.mp3')
    
    # Comprehensive repair
    audio_enhancer.repair_damaged_audio('damaged.mp3', 'repaired.mp3')
    
    # Normalize levels
    audio_enhancer.normalize_audio('input.mp3', 'normalized.mp3')
    """)
    
    print("\nVideo Enhancement:")
    print("""
    video_enhancer = VideoEnhancer()
    
    # Remove noise
    video_enhancer.denoise_video('input.mp4', 'denoised.mp4')
    
    # Sharpen
    video_enhancer.sharpen_video('input.mp4', 'sharp.mp4')
    
    # Stabilize
    video_enhancer.stabilize_video('shaky.mp4', 'stable.mp4')
    
    # Upscale with AI
    video_enhancer.upscale_video('input.mp4', 'upscaled.mp4', scale_factor=2)
    
    # Comprehensive repair
    video_enhancer.repair_damaged_video('damaged.mp4', 'repaired.mp4')
    """)


def example_subtitles():
    """Example: Subtitle management"""
    print_section("Example: Subtitle Management")
    
    print("\nSubtitle Search and Download:")
    print("""
    searcher = SubtitleSearcher()
    
    # Search for subtitles
    results = searcher.search_subtitles(
        movie_name='Movie Title',
        language='en',
        year=2023
    )
    
    # Download subtitle
    searcher.download_subtitle(
        subtitle_url='https://example.com/subtitle.srt',
        output_path='subtitle.srt'
    )
    """)
    
    print("\nAdd Subtitles to Video:")
    print("""
    # Hard-sub (burned into video)
    searcher.burn_subtitle_into_video(
        video_file='movie.mp4',
        subtitle_file='subtitle.srt',
        output_file='movie_with_subs.mp4'
    )
    
    # Soft-sub (separate stream)
    searcher.add_soft_subtitle(
        video_file='movie.mp4',
        subtitle_file='subtitle.srt',
        output_file='movie_with_subs.mp4',
        language='eng'
    )
    """)
    
    print("\nSupported Languages:")
    searcher = SubtitleSearcher()
    languages = searcher.get_available_languages()
    lang_names = [f"{lang['name']} ({lang['code']})" for lang in languages[:10]]
    print(f"  {', '.join(lang_names)}, ...")


def example_system_check():
    """Check system requirements"""
    print_section("System Requirements Check")
    
    tools = {
        'python3': 'Python 3',
        'ffmpeg': 'FFmpeg',
        'ffprobe': 'FFprobe',
        'HandBrakeCLI': 'HandBrake CLI',
    }
    
    print("\nChecking installed tools:")
    for cmd, name in tools.items():
        installed = is_tool_installed(cmd)
        status = "✅ Installed" if installed else "❌ Not installed"
        print(f"  {name}: {status}")


def main():
    """Main function"""
    print("\n" + "=" * 60)
    print("  NovaAudioVideoConverter - Feature Demonstration")
    print("=" * 60)
    
    # System check
    example_system_check()
    
    # Display capabilities
    example_format_info()
    example_device_profiles()
    
    # Show usage examples
    example_conversion()
    example_dvd_ripper()
    example_video_editing()
    example_ai_enhancement()
    example_subtitles()
    
    print("\n" + "=" * 60)
    print("  For GUI interface, run: python3 main.py")
    print("  For full documentation, see: README.md")
    print("=" * 60 + "\n")


if __name__ == "__main__":
    main()
