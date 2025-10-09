"""
DVD ripper module
Handles DVD ripping and protection removal
"""

import subprocess
import os
from typing import Optional, List, Dict, Callable


class DVDRipper:
    """DVD ripping functionality"""
    
    def __init__(self):
        self.lsdvd_path = 'lsdvd'
        self.handbrake_path = 'HandBrakeCLI'
        self.dvdbackup_path = 'dvdbackup'
        
    def check_tools_installed(self) -> Dict[str, bool]:
        """Check if DVD tools are installed"""
        tools = {}
        for tool_name, tool_path in [
            ('lsdvd', self.lsdvd_path),
            ('handbrake', self.handbrake_path),
            ('dvdbackup', self.dvdbackup_path)
        ]:
            try:
                subprocess.run(
                    [tool_path, '--version'],
                    capture_output=True,
                    timeout=5
                )
                tools[tool_name] = True
            except (subprocess.CalledProcessError, FileNotFoundError, subprocess.TimeoutExpired):
                tools[tool_name] = False
        return tools
    
    def get_dvd_info(self, dvd_path: str) -> Optional[Dict]:
        """
        Get DVD information
        
        Args:
            dvd_path: Path to DVD device or ISO file
        
        Returns:
            Dictionary with DVD information or None
        """
        try:
            cmd = [self.lsdvd_path, '-Ox', dvd_path]
            result = subprocess.run(cmd, capture_output=True, text=True, check=True)
            # Parse lsdvd XML output
            # For simplicity, returning basic info
            return {'raw_output': result.stdout}
        except (subprocess.CalledProcessError, FileNotFoundError):
            return None
    
    def list_titles(self, dvd_path: str) -> List[Dict]:
        """
        List all titles on DVD
        
        Args:
            dvd_path: Path to DVD device or ISO file
        
        Returns:
            List of title information dictionaries
        """
        try:
            cmd = [self.handbrake_path, '--scan', '-i', dvd_path]
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=30
            )
            # Parse HandBrake scan output
            # Simplified version - returns empty list for now
            titles = []
            return titles
        except (subprocess.CalledProcessError, FileNotFoundError, subprocess.TimeoutExpired):
            return []
    
    def rip_dvd(
        self,
        dvd_path: str,
        output_file: str,
        title: int = 1,
        preset: str = 'Fast 1080p30',
        remove_protection: bool = True,
        progress_callback: Optional[Callable] = None
    ) -> bool:
        """
        Rip DVD to file
        
        Args:
            dvd_path: Path to DVD device or ISO file
            output_file: Output file path
            title: Title number to rip
            preset: HandBrake preset
            remove_protection: Attempt to remove copy protection
            progress_callback: Progress callback function
        
        Returns:
            True if rip successful, False otherwise
        """
        cmd = [
            self.handbrake_path,
            '-i', dvd_path,
            '-o', output_file,
            '-t', str(title),
            '--preset', preset
        ]
        
        try:
            process = subprocess.Popen(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                universal_newlines=True
            )
            
            # Monitor progress
            if progress_callback:
                for line in process.stderr:
                    if 'Encoding:' in line and '%' in line:
                        try:
                            # Parse progress from HandBrake output
                            percent_str = line.split('%')[0].split()[-1]
                            percent = float(percent_str)
                            progress_callback(percent)
                        except (ValueError, IndexError):
                            pass
            
            process.wait()
            return process.returncode == 0
            
        except (subprocess.CalledProcessError, FileNotFoundError):
            return False
    
    def backup_dvd(
        self,
        dvd_path: str,
        output_dir: str,
        title: Optional[int] = None
    ) -> bool:
        """
        Backup entire DVD structure
        
        Args:
            dvd_path: Path to DVD device
            output_dir: Output directory
            title: Specific title to backup (None for all)
        
        Returns:
            True if backup successful, False otherwise
        """
        cmd = [
            self.dvdbackup_path,
            '-i', dvd_path,
            '-o', output_dir
        ]
        
        if title:
            cmd.extend(['-t', str(title)])
        else:
            cmd.append('-M')  # Mirror entire DVD
        
        try:
            subprocess.run(cmd, check=True, capture_output=True)
            return True
        except (subprocess.CalledProcessError, FileNotFoundError):
            return False
    
    def convert_iso_to_video(
        self,
        iso_path: str,
        output_file: str,
        title: int = 1,
        preset: str = 'Fast 1080p30'
    ) -> bool:
        """
        Convert ISO file to video
        
        Args:
            iso_path: Path to ISO file
            output_file: Output video file
            title: Title number
            preset: Encoding preset
        
        Returns:
            True if conversion successful, False otherwise
        """
        return self.rip_dvd(iso_path, output_file, title, preset)
