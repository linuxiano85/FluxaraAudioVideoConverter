"""
Core converter module using FFmpeg
Handles audio and video conversion with ultra-fast processing
"""

import subprocess
import os
import json
from typing import Optional, Dict, List, Callable


class Converter:
    """Main converter class using FFmpeg"""
    
    def __init__(self):
        self.ffmpeg_path = 'ffmpeg'
        self.ffprobe_path = 'ffprobe'
        
    def check_ffmpeg_installed(self) -> bool:
        """Check if FFmpeg is installed"""
        try:
            subprocess.run(
                [self.ffmpeg_path, '-version'],
                capture_output=True,
                check=True
            )
            return True
        except (subprocess.CalledProcessError, FileNotFoundError):
            return False
    
    def get_media_info(self, input_file: str) -> Optional[Dict]:
        """Get media file information using ffprobe"""
        try:
            cmd = [
                self.ffprobe_path,
                '-v', 'quiet',
                '-print_format', 'json',
                '-show_format',
                '-show_streams',
                input_file
            ]
            result = subprocess.run(cmd, capture_output=True, text=True, check=True)
            return json.loads(result.stdout)
        except (subprocess.CalledProcessError, json.JSONDecodeError, FileNotFoundError):
            return None
    
    def convert(
        self,
        input_file: str,
        output_file: str,
        video_codec: Optional[str] = None,
        audio_codec: Optional[str] = None,
        video_bitrate: Optional[str] = None,
        audio_bitrate: Optional[str] = None,
        resolution: Optional[str] = None,
        fps: Optional[int] = None,
        preset: str = 'medium',
        hwaccel: bool = False,
        progress_callback: Optional[Callable] = None
    ) -> bool:
        """
        Convert media file with specified parameters
        
        Args:
            input_file: Input file path
            output_file: Output file path
            video_codec: Video codec (e.g., 'h264', 'hevc')
            audio_codec: Audio codec (e.g., 'aac', 'mp3')
            video_bitrate: Video bitrate (e.g., '2M')
            audio_bitrate: Audio bitrate (e.g., '192k')
            resolution: Output resolution (e.g., '1920x1080')
            fps: Output frames per second
            preset: Encoding preset for speed/quality tradeoff
            hwaccel: Use hardware acceleration
            progress_callback: Callback function for progress updates
        
        Returns:
            True if conversion successful, False otherwise
        """
        cmd = [self.ffmpeg_path, '-i', input_file]
        
        # Hardware acceleration
        if hwaccel:
            cmd.extend(['-hwaccel', 'auto'])
        
        # Video codec
        if video_codec:
            cmd.extend(['-c:v', video_codec])
            if video_codec in ['libx264', 'libx265', 'h264_nvenc', 'hevc_nvenc']:
                cmd.extend(['-preset', preset])
        
        # Audio codec
        if audio_codec:
            cmd.extend(['-c:a', audio_codec])
        
        # Video bitrate
        if video_bitrate:
            cmd.extend(['-b:v', video_bitrate])
        
        # Audio bitrate
        if audio_bitrate:
            cmd.extend(['-b:a', audio_bitrate])
        
        # Resolution
        if resolution:
            cmd.extend(['-s', resolution])
        
        # FPS
        if fps:
            cmd.extend(['-r', str(fps)])
        
        # Progress monitoring
        cmd.extend(['-progress', 'pipe:1'])
        
        # Output file
        cmd.extend(['-y', output_file])
        
        try:
            process = subprocess.Popen(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                universal_newlines=True
            )
            
            # Monitor progress
            if progress_callback:
                for line in process.stdout:
                    if line.startswith('out_time_ms='):
                        time_ms = int(line.split('=')[1])
                        progress_callback(time_ms)
            
            process.wait()
            return process.returncode == 0
            
        except Exception as e:
            print(f"Conversion error: {e}")
            return False
    
    def merge_files(
        self,
        input_files: List[str],
        output_file: str,
        lossless: bool = True
    ) -> bool:
        """
        Merge multiple files without re-encoding (lossless)
        
        Args:
            input_files: List of input file paths
            output_file: Output file path
            lossless: If True, use stream copy (no re-encoding)
        
        Returns:
            True if merge successful, False otherwise
        """
        # Create concat file list
        concat_file = '/tmp/concat_list.txt'
        with open(concat_file, 'w') as f:
            for input_file in input_files:
                f.write(f"file '{os.path.abspath(input_file)}'\n")
        
        cmd = [
            self.ffmpeg_path,
            '-f', 'concat',
            '-safe', '0',
            '-i', concat_file
        ]
        
        if lossless:
            cmd.extend(['-c', 'copy'])
        
        cmd.extend(['-y', output_file])
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            os.remove(concat_file)
            return True
        except subprocess.CalledProcessError:
            if os.path.exists(concat_file):
                os.remove(concat_file)
            return False
    
    def cut_video(
        self,
        input_file: str,
        output_file: str,
        start_time: str,
        end_time: Optional[str] = None,
        duration: Optional[str] = None,
        lossless: bool = True
    ) -> bool:
        """
        Cut/trim video
        
        Args:
            input_file: Input file path
            output_file: Output file path
            start_time: Start time (format: HH:MM:SS or seconds)
            end_time: End time (format: HH:MM:SS or seconds)
            duration: Duration instead of end time
            lossless: If True, use stream copy
        
        Returns:
            True if cut successful, False otherwise
        """
        cmd = [self.ffmpeg_path, '-i', input_file, '-ss', start_time]
        
        if end_time:
            cmd.extend(['-to', end_time])
        elif duration:
            cmd.extend(['-t', duration])
        
        if lossless:
            cmd.extend(['-c', 'copy'])
        
        cmd.extend(['-y', output_file])
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
    
    def rotate_video(
        self,
        input_file: str,
        output_file: str,
        rotation: int
    ) -> bool:
        """
        Rotate video
        
        Args:
            input_file: Input file path
            output_file: Output file path
            rotation: Rotation angle (90, 180, 270)
        
        Returns:
            True if rotation successful, False otherwise
        """
        transpose_map = {
            90: '1',
            180: '2,transpose=2',
            270: '2'
        }
        
        transpose = transpose_map.get(rotation)
        if not transpose:
            return False
        
        cmd = [
            self.ffmpeg_path,
            '-i', input_file,
            '-vf', f'transpose={transpose}',
            '-c:a', 'copy',
            '-y', output_file
        ]
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
    
    def crop_video(
        self,
        input_file: str,
        output_file: str,
        width: int,
        height: int,
        x: int = 0,
        y: int = 0
    ) -> bool:
        """
        Crop video
        
        Args:
            input_file: Input file path
            output_file: Output file path
            width: Crop width
            height: Crop height
            x: X offset
            y: Y offset
        
        Returns:
            True if crop successful, False otherwise
        """
        cmd = [
            self.ffmpeg_path,
            '-i', input_file,
            '-vf', f'crop={width}:{height}:{x}:{y}',
            '-c:a', 'copy',
            '-y', output_file
        ]
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
    
    def apply_filters(
        self,
        input_file: str,
        output_file: str,
        filters: List[str]
    ) -> bool:
        """
        Apply video filters
        
        Args:
            input_file: Input file path
            output_file: Output file path
            filters: List of filter strings
        
        Returns:
            True if filters applied successfully, False otherwise
        """
        filter_chain = ','.join(filters)
        
        cmd = [
            self.ffmpeg_path,
            '-i', input_file,
            '-vf', filter_chain,
            '-y', output_file
        ]
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
