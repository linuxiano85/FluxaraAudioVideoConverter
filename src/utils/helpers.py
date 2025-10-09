"""
Utility functions for NovaAudioVideoConverter
"""

import os
import subprocess
from typing import Optional, Tuple
import hashlib


def check_file_exists(file_path: str) -> bool:
    """Check if file exists"""
    return os.path.isfile(file_path)


def get_file_size(file_path: str) -> int:
    """Get file size in bytes"""
    if check_file_exists(file_path):
        return os.path.getsize(file_path)
    return 0


def format_file_size(size_bytes: int) -> str:
    """Format file size in human-readable format"""
    for unit in ['B', 'KB', 'MB', 'GB', 'TB']:
        if size_bytes < 1024.0:
            return f"{size_bytes:.2f} {unit}"
        size_bytes /= 1024.0
    return f"{size_bytes:.2f} PB"


def format_duration(seconds: float) -> str:
    """Format duration in HH:MM:SS format"""
    hours = int(seconds // 3600)
    minutes = int((seconds % 3600) // 60)
    secs = int(seconds % 60)
    return f"{hours:02d}:{minutes:02d}:{secs:02d}"


def parse_time_to_seconds(time_str: str) -> float:
    """Parse time string to seconds"""
    try:
        # Try HH:MM:SS format
        parts = time_str.split(':')
        if len(parts) == 3:
            hours, minutes, seconds = parts
            return int(hours) * 3600 + int(minutes) * 60 + float(seconds)
        elif len(parts) == 2:
            minutes, seconds = parts
            return int(minutes) * 60 + float(seconds)
        else:
            return float(time_str)
    except (ValueError, AttributeError):
        return 0.0


def get_file_hash(file_path: str, algorithm: str = 'md5') -> Optional[str]:
    """Calculate file hash"""
    if not check_file_exists(file_path):
        return None
    
    hash_func = hashlib.new(algorithm)
    with open(file_path, 'rb') as f:
        for chunk in iter(lambda: f.read(4096), b''):
            hash_func.update(chunk)
    
    return hash_func.hexdigest()


def create_directory(directory: str) -> bool:
    """Create directory if it doesn't exist"""
    try:
        os.makedirs(directory, exist_ok=True)
        return True
    except OSError:
        return False


def get_available_disk_space(path: str) -> int:
    """Get available disk space in bytes"""
    try:
        stat = os.statvfs(path)
        return stat.f_bavail * stat.f_frsize
    except (OSError, AttributeError):
        return 0


def is_tool_installed(tool_name: str) -> bool:
    """Check if a command-line tool is installed"""
    try:
        subprocess.run(
            [tool_name, '--version'],
            capture_output=True,
            timeout=5
        )
        return True
    except (subprocess.CalledProcessError, FileNotFoundError, subprocess.TimeoutExpired):
        return False


def get_video_info_quick(file_path: str) -> Optional[dict]:
    """Get quick video information"""
    if not check_file_exists(file_path):
        return None
    
    try:
        cmd = [
            'ffprobe',
            '-v', 'quiet',
            '-print_format', 'json',
            '-show_format',
            '-show_streams',
            file_path
        ]
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
        
        if result.returncode == 0:
            import json
            data = json.loads(result.stdout)
            
            video_info = {}
            for stream in data.get('streams', []):
                if stream.get('codec_type') == 'video':
                    video_info['width'] = stream.get('width')
                    video_info['height'] = stream.get('height')
                    video_info['codec'] = stream.get('codec_name')
                    video_info['fps'] = eval(stream.get('r_frame_rate', '0/1'))
                    break
            
            format_info = data.get('format', {})
            video_info['duration'] = float(format_info.get('duration', 0))
            video_info['size'] = int(format_info.get('size', 0))
            video_info['bitrate'] = int(format_info.get('bit_rate', 0))
            
            return video_info
    except (subprocess.CalledProcessError, subprocess.TimeoutExpired, json.JSONDecodeError):
        return None


def sanitize_filename(filename: str) -> str:
    """Sanitize filename by removing invalid characters"""
    invalid_chars = '<>:"|?*\\/\0'
    for char in invalid_chars:
        filename = filename.replace(char, '_')
    return filename


def get_extension(file_path: str) -> str:
    """Get file extension without dot"""
    return os.path.splitext(file_path)[1].lstrip('.').lower()


def change_extension(file_path: str, new_extension: str) -> str:
    """Change file extension"""
    base = os.path.splitext(file_path)[0]
    if not new_extension.startswith('.'):
        new_extension = '.' + new_extension
    return base + new_extension


def estimate_output_size(
    input_size: int,
    input_duration: float,
    output_bitrate: int
) -> int:
    """Estimate output file size based on bitrate"""
    if input_duration > 0 and output_bitrate > 0:
        # bitrate is in bits per second, convert to bytes
        return int((output_bitrate / 8) * input_duration)
    return input_size


def validate_video_file(file_path: str) -> Tuple[bool, str]:
    """Validate video file"""
    if not check_file_exists(file_path):
        return False, "File does not exist"
    
    if get_file_size(file_path) == 0:
        return False, "File is empty"
    
    info = get_video_info_quick(file_path)
    if not info:
        return False, "Cannot read file information"
    
    if info.get('duration', 0) <= 0:
        return False, "Invalid duration"
    
    return True, "Valid video file"


def get_cpu_count() -> int:
    """Get number of CPU cores"""
    try:
        import multiprocessing
        return multiprocessing.cpu_count()
    except (ImportError, NotImplementedError):
        return 1


def format_bitrate(bitrate: int) -> str:
    """Format bitrate in human-readable format"""
    if bitrate >= 1000000:
        return f"{bitrate / 1000000:.1f} Mbps"
    elif bitrate >= 1000:
        return f"{bitrate / 1000:.1f} Kbps"
    else:
        return f"{bitrate} bps"
