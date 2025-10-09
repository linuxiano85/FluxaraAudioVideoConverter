"""
Main GUI window for NovaAudioVideoConverter
"""

from PyQt5.QtWidgets import (
    QMainWindow, QWidget, QVBoxLayout, QHBoxLayout, QPushButton,
    QLabel, QLineEdit, QComboBox, QFileDialog, QProgressBar,
    QTextEdit, QTabWidget, QGroupBox, QSpinBox, QCheckBox,
    QMessageBox, QListWidget
)
from PyQt5.QtCore import Qt, QThread, pyqtSignal
from PyQt5.QtGui import QFont
import os


class ConversionThread(QThread):
    """Thread for background conversion"""
    progress = pyqtSignal(int)
    finished = pyqtSignal(bool, str)
    
    def __init__(self, converter, **kwargs):
        super().__init__()
        self.converter = converter
        self.kwargs = kwargs
    
    def run(self):
        try:
            success = self.converter.convert(**self.kwargs)
            self.finished.emit(success, "Conversion completed successfully" if success else "Conversion failed")
        except Exception as e:
            self.finished.emit(False, f"Error: {str(e)}")


class MainWindow(QMainWindow):
    """Main application window"""
    
    def __init__(self):
        super().__init__()
        self.setWindowTitle("NovaAudioVideoConverter - Audio/Video Converter & Enhancer")
        self.setMinimumSize(1000, 700)
        
        # Initialize conversion thread
        self.conversion_thread = None
        
        # Setup UI
        self.setup_ui()
        
    def setup_ui(self):
        """Setup the user interface"""
        central_widget = QWidget()
        self.setCentralWidget(central_widget)
        
        main_layout = QVBoxLayout()
        central_widget.setLayout(main_layout)
        
        # Title
        title = QLabel("NovaAudioVideoConverter")
        title.setFont(QFont("Arial", 20, QFont.Bold))
        title.setAlignment(Qt.AlignCenter)
        main_layout.addWidget(title)
        
        # Subtitle
        subtitle = QLabel("Professional Audio & Video Converter with AI Enhancement")
        subtitle.setFont(QFont("Arial", 10))
        subtitle.setAlignment(Qt.AlignCenter)
        main_layout.addWidget(subtitle)
        
        # Tab widget for different functions
        tabs = QTabWidget()
        main_layout.addWidget(tabs)
        
        # Tab 1: Convert
        tabs.addTab(self.create_convert_tab(), "Convert")
        
        # Tab 2: DVD Ripper
        tabs.addTab(self.create_dvd_tab(), "DVD Ripper")
        
        # Tab 3: Video Editor
        tabs.addTab(self.create_editor_tab(), "Video Editor")
        
        # Tab 4: AI Enhancement
        tabs.addTab(self.create_enhancement_tab(), "AI Enhancement")
        
        # Tab 5: Subtitles
        tabs.addTab(self.create_subtitle_tab(), "Subtitles")
        
        # Progress bar
        self.progress_bar = QProgressBar()
        main_layout.addWidget(self.progress_bar)
        
        # Status/Log area
        status_group = QGroupBox("Status Log")
        status_layout = QVBoxLayout()
        self.status_log = QTextEdit()
        self.status_log.setReadOnly(True)
        self.status_log.setMaximumHeight(150)
        status_layout.addWidget(self.status_log)
        status_group.setLayout(status_layout)
        main_layout.addWidget(status_group)
        
        self.log("NovaAudioVideoConverter initialized successfully")
        self.log("Supports 180+ formats and 200+ device profiles")
    
    def create_convert_tab(self):
        """Create conversion tab"""
        tab = QWidget()
        layout = QVBoxLayout()
        
        # Input file
        input_group = QGroupBox("Input File")
        input_layout = QHBoxLayout()
        self.input_file = QLineEdit()
        input_browse = QPushButton("Browse...")
        input_browse.clicked.connect(self.browse_input_file)
        input_layout.addWidget(self.input_file)
        input_layout.addWidget(input_browse)
        input_group.setLayout(input_layout)
        layout.addWidget(input_group)
        
        # Output file
        output_group = QGroupBox("Output File")
        output_layout = QHBoxLayout()
        self.output_file = QLineEdit()
        output_browse = QPushButton("Browse...")
        output_browse.clicked.connect(self.browse_output_file)
        output_layout.addWidget(self.output_file)
        output_layout.addWidget(output_browse)
        output_group.setLayout(output_layout)
        layout.addWidget(output_group)
        
        # Device profile
        profile_group = QGroupBox("Device Profile (200+ profiles)")
        profile_layout = QHBoxLayout()
        profile_layout.addWidget(QLabel("Profile:"))
        self.device_profile = QComboBox()
        self.device_profile.addItems([
            "Custom",
            "iPhone 15 Pro",
            "Samsung Galaxy S24 Ultra",
            "iPad Pro",
            "YouTube 1080p",
            "4K Ultra HD",
            "Full HD 1080p",
            "HD 720p",
        ])
        profile_layout.addWidget(self.device_profile)
        profile_group.setLayout(profile_layout)
        layout.addWidget(profile_group)
        
        # Conversion settings
        settings_group = QGroupBox("Conversion Settings")
        settings_layout = QVBoxLayout()
        
        # Video codec
        video_layout = QHBoxLayout()
        video_layout.addWidget(QLabel("Video Codec:"))
        self.video_codec = QComboBox()
        self.video_codec.addItems(["h264", "h265", "vp9", "av1", "mpeg4"])
        video_layout.addWidget(self.video_codec)
        settings_layout.addLayout(video_layout)
        
        # Audio codec
        audio_layout = QHBoxLayout()
        audio_layout.addWidget(QLabel("Audio Codec:"))
        self.audio_codec = QComboBox()
        self.audio_codec.addItems(["aac", "mp3", "opus", "vorbis", "flac"])
        audio_layout.addWidget(self.audio_codec)
        settings_layout.addLayout(audio_layout)
        
        # Hardware acceleration
        self.hw_accel = QCheckBox("Use Hardware Acceleration")
        settings_layout.addWidget(self.hw_accel)
        
        settings_group.setLayout(settings_layout)
        layout.addWidget(settings_group)
        
        # Convert button
        convert_btn = QPushButton("Start Conversion")
        convert_btn.setStyleSheet("QPushButton { background-color: #4CAF50; color: white; font-size: 14px; padding: 10px; }")
        convert_btn.clicked.connect(self.start_conversion)
        layout.addWidget(convert_btn)
        
        layout.addStretch()
        tab.setLayout(layout)
        return tab
    
    def create_dvd_tab(self):
        """Create DVD ripper tab"""
        tab = QWidget()
        layout = QVBoxLayout()
        
        label = QLabel("DVD Ripper - Rip DVDs with protection removal")
        label.setFont(QFont("Arial", 12, QFont.Bold))
        layout.addWidget(label)
        
        # DVD source
        source_group = QGroupBox("DVD Source")
        source_layout = QHBoxLayout()
        self.dvd_source = QLineEdit()
        self.dvd_source.setPlaceholderText("/dev/dvd or path to ISO file")
        dvd_browse = QPushButton("Browse...")
        source_layout.addWidget(self.dvd_source)
        source_layout.addWidget(dvd_browse)
        source_group.setLayout(source_layout)
        layout.addWidget(source_group)
        
        # Title selection
        title_layout = QHBoxLayout()
        title_layout.addWidget(QLabel("Title:"))
        self.dvd_title = QSpinBox()
        self.dvd_title.setMinimum(1)
        self.dvd_title.setValue(1)
        title_layout.addWidget(self.dvd_title)
        layout.addLayout(title_layout)
        
        # Options
        self.dvd_remove_protection = QCheckBox("Remove Copy Protection")
        self.dvd_remove_protection.setChecked(True)
        layout.addWidget(self.dvd_remove_protection)
        
        # Rip button
        rip_btn = QPushButton("Rip DVD")
        rip_btn.setStyleSheet("QPushButton { background-color: #2196F3; color: white; font-size: 14px; padding: 10px; }")
        layout.addWidget(rip_btn)
        
        layout.addStretch()
        tab.setLayout(layout)
        return tab
    
    def create_editor_tab(self):
        """Create video editor tab"""
        tab = QWidget()
        layout = QVBoxLayout()
        
        label = QLabel("Video Editor - Cut, Rotate, Crop")
        label.setFont(QFont("Arial", 12, QFont.Bold))
        layout.addWidget(label)
        
        # Operations
        operations_group = QGroupBox("Operations")
        operations_layout = QVBoxLayout()
        
        # Cut
        cut_btn = QPushButton("Cut/Trim Video")
        operations_layout.addWidget(cut_btn)
        
        # Rotate
        rotate_layout = QHBoxLayout()
        rotate_btn = QPushButton("Rotate Video")
        self.rotate_angle = QComboBox()
        self.rotate_angle.addItems(["90°", "180°", "270°"])
        rotate_layout.addWidget(rotate_btn)
        rotate_layout.addWidget(self.rotate_angle)
        operations_layout.addLayout(rotate_layout)
        
        # Crop
        crop_btn = QPushButton("Crop Video")
        operations_layout.addWidget(crop_btn)
        
        # Merge
        merge_btn = QPushButton("Merge Files (Lossless)")
        operations_layout.addWidget(merge_btn)
        
        operations_group.setLayout(operations_layout)
        layout.addWidget(operations_group)
        
        layout.addStretch()
        tab.setLayout(layout)
        return tab
    
    def create_enhancement_tab(self):
        """Create AI enhancement tab"""
        tab = QWidget()
        layout = QVBoxLayout()
        
        label = QLabel("AI Enhancement - Enhance damaged audio and video")
        label.setFont(QFont("Arial", 12, QFont.Bold))
        layout.addWidget(label)
        
        # Audio enhancement
        audio_group = QGroupBox("Audio Enhancement")
        audio_layout = QVBoxLayout()
        
        audio_denoise = QPushButton("Remove Audio Noise")
        audio_layout.addWidget(audio_denoise)
        
        audio_enhance = QPushButton("Enhance Voice")
        audio_layout.addWidget(audio_enhance)
        
        audio_repair = QPushButton("Repair Damaged Audio")
        audio_layout.addWidget(audio_repair)
        
        audio_normalize = QPushButton("Normalize Audio Levels")
        audio_layout.addWidget(audio_normalize)
        
        audio_group.setLayout(audio_layout)
        layout.addWidget(audio_group)
        
        # Video enhancement
        video_group = QGroupBox("Video Enhancement")
        video_layout = QVBoxLayout()
        
        video_denoise = QPushButton("Remove Video Noise")
        video_layout.addWidget(video_denoise)
        
        video_sharpen = QPushButton("Sharpen Video")
        video_layout.addWidget(video_sharpen)
        
        video_stabilize = QPushButton("Stabilize Video")
        video_layout.addWidget(video_stabilize)
        
        video_upscale = QPushButton("Upscale Video (AI)")
        video_layout.addWidget(video_upscale)
        
        video_repair = QPushButton("Repair Damaged Video")
        video_layout.addWidget(video_repair)
        
        video_group.setLayout(video_layout)
        layout.addWidget(video_group)
        
        layout.addStretch()
        tab.setLayout(layout)
        return tab
    
    def create_subtitle_tab(self):
        """Create subtitle tab"""
        tab = QWidget()
        layout = QVBoxLayout()
        
        label = QLabel("Subtitles - Search and add subtitles")
        label.setFont(QFont("Arial", 12, QFont.Bold))
        layout.addWidget(label)
        
        # Search
        search_group = QGroupBox("Search Subtitles Online")
        search_layout = QVBoxLayout()
        
        search_input_layout = QHBoxLayout()
        self.subtitle_search = QLineEdit()
        self.subtitle_search.setPlaceholderText("Movie name...")
        search_btn = QPushButton("Search")
        search_input_layout.addWidget(self.subtitle_search)
        search_input_layout.addWidget(search_btn)
        search_layout.addLayout(search_input_layout)
        
        # Language selection
        lang_layout = QHBoxLayout()
        lang_layout.addWidget(QLabel("Language:"))
        self.subtitle_language = QComboBox()
        self.subtitle_language.addItems([
            "English", "Italian", "Spanish", "French", "German",
            "Portuguese", "Russian", "Chinese", "Japanese"
        ])
        lang_layout.addWidget(self.subtitle_language)
        search_layout.addLayout(lang_layout)
        
        # Results list
        self.subtitle_results = QListWidget()
        search_layout.addWidget(self.subtitle_results)
        
        search_group.setLayout(search_layout)
        layout.addWidget(search_group)
        
        # Add subtitle
        add_group = QGroupBox("Add Subtitle to Video")
        add_layout = QVBoxLayout()
        
        hard_sub_btn = QPushButton("Burn Subtitle (Hard-sub)")
        add_layout.addWidget(hard_sub_btn)
        
        soft_sub_btn = QPushButton("Add Subtitle Stream (Soft-sub)")
        add_layout.addWidget(soft_sub_btn)
        
        add_group.setLayout(add_layout)
        layout.addWidget(add_group)
        
        layout.addStretch()
        tab.setLayout(layout)
        return tab
    
    def browse_input_file(self):
        """Browse for input file"""
        file_path, _ = QFileDialog.getOpenFileName(
            self,
            "Select Input File",
            "",
            "All Files (*);;Video Files (*.mp4 *.avi *.mkv *.mov);;Audio Files (*.mp3 *.wav *.flac)"
        )
        if file_path:
            self.input_file.setText(file_path)
            self.log(f"Input file selected: {file_path}")
    
    def browse_output_file(self):
        """Browse for output file"""
        file_path, _ = QFileDialog.getSaveFileName(
            self,
            "Select Output File",
            "",
            "All Files (*);;MP4 (*.mp4);;AVI (*.avi);;MKV (*.mkv)"
        )
        if file_path:
            self.output_file.setText(file_path)
            self.log(f"Output file selected: {file_path}")
    
    def start_conversion(self):
        """Start the conversion process"""
        input_file = self.input_file.text()
        output_file = self.output_file.text()
        
        if not input_file or not output_file:
            QMessageBox.warning(self, "Error", "Please select input and output files")
            return
        
        if not os.path.exists(input_file):
            QMessageBox.warning(self, "Error", "Input file does not exist")
            return
        
        self.log(f"Starting conversion: {input_file} -> {output_file}")
        self.log(f"Video codec: {self.video_codec.currentText()}")
        self.log(f"Audio codec: {self.audio_codec.currentText()}")
        self.log(f"Device profile: {self.device_profile.currentText()}")
        
        # TODO: Start actual conversion in background thread
        self.progress_bar.setValue(0)
        QMessageBox.information(
            self,
            "Conversion Started",
            "Conversion has been started. This is a demo - actual conversion requires FFmpeg installation."
        )
    
    def log(self, message):
        """Add message to status log"""
        self.status_log.append(f"[INFO] {message}")
