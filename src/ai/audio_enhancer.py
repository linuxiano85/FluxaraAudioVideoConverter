"""
AI enhancement module for audio
Uses AI techniques to clean and enhance damaged audio
"""

import subprocess
from typing import Optional


class AudioEnhancer:
    """AI-powered audio enhancement"""
    
    def __init__(self):
        self.ffmpeg_path = 'ffmpeg'
    
    def denoise_audio(
        self,
        input_file: str,
        output_file: str,
        noise_reduction: float = 0.21
    ) -> bool:
        """
        Remove noise from audio using AI-based filtering
        
        Args:
            input_file: Input audio file
            output_file: Output audio file
            noise_reduction: Noise reduction amount (0.0-1.0)
        
        Returns:
            True if successful, False otherwise
        """
        cmd = [
            self.ffmpeg_path,
            '-i', input_file,
            '-af', f'anlmdn=s={noise_reduction}',
            '-y', output_file
        ]
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
    
    def normalize_audio(
        self,
        input_file: str,
        output_file: str,
        target_level: float = -23.0
    ) -> bool:
        """
        Normalize audio levels
        
        Args:
            input_file: Input audio file
            output_file: Output audio file
            target_level: Target loudness in LUFS
        
        Returns:
            True if successful, False otherwise
        """
        cmd = [
            self.ffmpeg_path,
            '-i', input_file,
            '-af', f'loudnorm=I={target_level}:TP=-1.5:LRA=11',
            '-y', output_file
        ]
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
    
    def enhance_voice(
        self,
        input_file: str,
        output_file: str
    ) -> bool:
        """
        Enhance voice in audio (reduce non-voice frequencies)
        
        Args:
            input_file: Input audio file
            output_file: Output audio file
        
        Returns:
            True if successful, False otherwise
        """
        # Apply highpass and lowpass filters optimized for voice
        cmd = [
            self.ffmpeg_path,
            '-i', input_file,
            '-af', 'highpass=f=200,lowpass=f=3000,afftdn=nf=-25',
            '-y', output_file
        ]
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
    
    def remove_clicks_pops(
        self,
        input_file: str,
        output_file: str
    ) -> bool:
        """
        Remove clicks and pops from audio
        
        Args:
            input_file: Input audio file
            output_file: Output audio file
        
        Returns:
            True if successful, False otherwise
        """
        cmd = [
            self.ffmpeg_path,
            '-i', input_file,
            '-af', 'adeclick=t=2:w=10',
            '-y', output_file
        ]
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
    
    def compress_dynamics(
        self,
        input_file: str,
        output_file: str,
        threshold: float = -20.0,
        ratio: float = 4.0
    ) -> bool:
        """
        Apply dynamic range compression
        
        Args:
            input_file: Input audio file
            output_file: Output audio file
            threshold: Compression threshold in dB
            ratio: Compression ratio
        
        Returns:
            True if successful, False otherwise
        """
        cmd = [
            self.ffmpeg_path,
            '-i', input_file,
            '-af', f'acompressor=threshold={threshold}dB:ratio={ratio}:attack=5:release=50',
            '-y', output_file
        ]
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
    
    def equalize_audio(
        self,
        input_file: str,
        output_file: str,
        preset: str = 'balanced'
    ) -> bool:
        """
        Apply equalization to audio
        
        Args:
            input_file: Input audio file
            output_file: Output audio file
            preset: EQ preset (balanced, bass_boost, treble_boost, voice)
        
        Returns:
            True if successful, False otherwise
        """
        presets = {
            'balanced': 'equalizer=f=100:t=q:w=1:g=2,equalizer=f=1000:t=q:w=1:g=1',
            'bass_boost': 'equalizer=f=60:t=q:w=1:g=6,equalizer=f=100:t=q:w=1:g=4',
            'treble_boost': 'equalizer=f=5000:t=q:w=1:g=4,equalizer=f=10000:t=q:w=1:g=6',
            'voice': 'equalizer=f=300:t=q:w=1:g=3,equalizer=f=1000:t=q:w=1:g=2'
        }
        
        eq_filter = presets.get(preset, presets['balanced'])
        
        cmd = [
            self.ffmpeg_path,
            '-i', input_file,
            '-af', eq_filter,
            '-y', output_file
        ]
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
    
    def repair_damaged_audio(
        self,
        input_file: str,
        output_file: str
    ) -> bool:
        """
        Comprehensive audio repair for damaged files
        Applies multiple filters to clean up damaged audio
        
        Args:
            input_file: Input audio file
            output_file: Output audio file
        
        Returns:
            True if successful, False otherwise
        """
        # Chain multiple filters for comprehensive repair
        cmd = [
            self.ffmpeg_path,
            '-i', input_file,
            '-af', 'adeclick=t=2:w=10,afftdn=nf=-25,anlmdn=s=0.15,loudnorm=I=-16:TP=-1.5:LRA=11',
            '-y', output_file
        ]
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
