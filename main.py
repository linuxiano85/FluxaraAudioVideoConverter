#!/usr/bin/env python3
"""
NovaAudioVideoConverter - Main Application Entry Point
A comprehensive audio and video converter for Linux with AI-powered enhancements
"""

import sys
from PyQt5.QtWidgets import QApplication
from src.gui.main_window import MainWindow

def main():
    """Main application entry point"""
    app = QApplication(sys.argv)
    app.setApplicationName("NovaAudioVideoConverter")
    app.setOrganizationName("NovaAudioVideoConverter")
    
    window = MainWindow()
    window.show()
    
    sys.exit(app.exec_())

if __name__ == "__main__":
    main()
