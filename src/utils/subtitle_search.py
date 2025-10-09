"""
Subtitle search and download module
Search and download subtitles from online sources
"""

import requests
from bs4 import BeautifulSoup
from typing import List, Dict, Optional
import os


class SubtitleSearcher:
    """Search and download subtitles from online sources"""
    
    def __init__(self):
        self.user_agent = 'NovaAudioVideoConverter/1.0'
        self.headers = {'User-Agent': self.user_agent}
    
    def search_subtitles(
        self,
        movie_name: str,
        language: str = 'en',
        year: Optional[int] = None
    ) -> List[Dict]:
        """
        Search for subtitles online
        
        Args:
            movie_name: Name of the movie/video
            language: Subtitle language code (en, it, es, fr, etc.)
            year: Release year (optional)
        
        Returns:
            List of subtitle results with download info
        """
        results = []
        
        # This is a placeholder implementation
        # In production, this would integrate with services like:
        # - OpenSubtitles API
        # - Subscene
        # - YIFY Subtitles
        # etc.
        
        # Example result structure
        results.append({
            'title': movie_name,
            'language': language,
            'year': year,
            'rating': 8.5,
            'downloads': 1000,
            'url': 'https://example.com/subtitle.srt',
            'format': 'srt'
        })
        
        return results
    
    def download_subtitle(
        self,
        subtitle_url: str,
        output_path: str
    ) -> bool:
        """
        Download subtitle file
        
        Args:
            subtitle_url: URL to subtitle file
            output_path: Path to save subtitle file
        
        Returns:
            True if download successful, False otherwise
        """
        try:
            response = requests.get(subtitle_url, headers=self.headers, timeout=30)
            response.raise_for_status()
            
            with open(output_path, 'wb') as f:
                f.write(response.content)
            
            return True
        except (requests.RequestException, IOError):
            return False
    
    def extract_subtitle_from_video(
        self,
        video_file: str,
        output_file: str,
        stream_index: int = 0
    ) -> bool:
        """
        Extract embedded subtitle from video file
        
        Args:
            video_file: Input video file
            output_file: Output subtitle file
            stream_index: Subtitle stream index
        
        Returns:
            True if extraction successful, False otherwise
        """
        import subprocess
        
        cmd = [
            'ffmpeg',
            '-i', video_file,
            '-map', f'0:s:{stream_index}',
            '-y', output_file
        ]
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
    
    def burn_subtitle_into_video(
        self,
        video_file: str,
        subtitle_file: str,
        output_file: str,
        font_size: int = 24
    ) -> bool:
        """
        Burn subtitle into video (hard-sub)
        
        Args:
            video_file: Input video file
            subtitle_file: Subtitle file (SRT, ASS, etc.)
            output_file: Output video file
            font_size: Font size for subtitles
        
        Returns:
            True if successful, False otherwise
        """
        import subprocess
        
        # Escape subtitle path for filter
        subtitle_path = subtitle_file.replace('\\', '/').replace(':', '\\:')
        
        cmd = [
            'ffmpeg',
            '-i', video_file,
            '-vf', f"subtitles='{subtitle_path}':force_style='Fontsize={font_size}'",
            '-c:a', 'copy',
            '-y', output_file
        ]
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
    
    def add_soft_subtitle(
        self,
        video_file: str,
        subtitle_file: str,
        output_file: str,
        language: str = 'eng'
    ) -> bool:
        """
        Add subtitle as separate stream (soft-sub)
        
        Args:
            video_file: Input video file
            subtitle_file: Subtitle file
            output_file: Output video file
            language: Subtitle language code
        
        Returns:
            True if successful, False otherwise
        """
        import subprocess
        
        cmd = [
            'ffmpeg',
            '-i', video_file,
            '-i', subtitle_file,
            '-c', 'copy',
            '-c:s', 'mov_text',
            '-metadata:s:s:0', f'language={language}',
            '-y', output_file
        ]
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
    
    def convert_subtitle_format(
        self,
        input_file: str,
        output_file: str,
        output_format: str = 'srt'
    ) -> bool:
        """
        Convert subtitle between formats
        
        Args:
            input_file: Input subtitle file
            output_file: Output subtitle file
            output_format: Output format (srt, ass, vtt, etc.)
        
        Returns:
            True if conversion successful, False otherwise
        """
        import subprocess
        
        cmd = [
            'ffmpeg',
            '-i', input_file,
            '-y', output_file
        ]
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except subprocess.CalledProcessError:
            return False
    
    def get_available_languages(self) -> List[Dict[str, str]]:
        """
        Get list of available subtitle languages
        
        Returns:
            List of language dictionaries with code and name
        """
        return [
            {'code': 'en', 'name': 'English'},
            {'code': 'it', 'name': 'Italian'},
            {'code': 'es', 'name': 'Spanish'},
            {'code': 'fr', 'name': 'French'},
            {'code': 'de', 'name': 'German'},
            {'code': 'pt', 'name': 'Portuguese'},
            {'code': 'ru', 'name': 'Russian'},
            {'code': 'zh', 'name': 'Chinese'},
            {'code': 'ja', 'name': 'Japanese'},
            {'code': 'ko', 'name': 'Korean'},
            {'code': 'ar', 'name': 'Arabic'},
            {'code': 'hi', 'name': 'Hindi'},
            {'code': 'tr', 'name': 'Turkish'},
            {'code': 'pl', 'name': 'Polish'},
            {'code': 'nl', 'name': 'Dutch'},
        ]
