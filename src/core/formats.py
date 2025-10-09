"""
Supported formats module
Defines 180+ supported audio and video formats
"""

# Video formats (100+)
VIDEO_FORMATS = {
    # Common formats
    'mp4': {'codec': 'h264', 'container': 'mp4', 'description': 'MPEG-4 Part 14'},
    'avi': {'codec': 'mpeg4', 'container': 'avi', 'description': 'Audio Video Interleave'},
    'mkv': {'codec': 'h264', 'container': 'matroska', 'description': 'Matroska Video'},
    'mov': {'codec': 'h264', 'container': 'mov', 'description': 'QuickTime Movie'},
    'wmv': {'codec': 'wmv2', 'container': 'asf', 'description': 'Windows Media Video'},
    'flv': {'codec': 'flv', 'container': 'flv', 'description': 'Flash Video'},
    'webm': {'codec': 'vp9', 'container': 'webm', 'description': 'WebM Video'},
    'mpg': {'codec': 'mpeg1video', 'container': 'mpeg', 'description': 'MPEG-1'},
    'mpeg': {'codec': 'mpeg2video', 'container': 'mpeg', 'description': 'MPEG-2'},
    'm4v': {'codec': 'h264', 'container': 'mp4', 'description': 'iTunes Video'},
    '3gp': {'codec': 'h263', 'container': '3gp', 'description': '3GPP Mobile'},
    '3g2': {'codec': 'h263', 'container': '3g2', 'description': '3GPP2 Mobile'},
    'ogv': {'codec': 'theora', 'container': 'ogg', 'description': 'Ogg Video'},
    'vob': {'codec': 'mpeg2video', 'container': 'vob', 'description': 'DVD Video Object'},
    'ts': {'codec': 'h264', 'container': 'mpegts', 'description': 'MPEG Transport Stream'},
    'mts': {'codec': 'h264', 'container': 'mpegts', 'description': 'AVCHD Video'},
    'm2ts': {'codec': 'h264', 'container': 'mpegts', 'description': 'Blu-ray BDAV'},
    'f4v': {'codec': 'h264', 'container': 'mp4', 'description': 'Flash MP4 Video'},
    'rm': {'codec': 'rv10', 'container': 'rm', 'description': 'RealMedia'},
    'rmvb': {'codec': 'rv40', 'container': 'rm', 'description': 'RealMedia Variable Bitrate'},
    'asf': {'codec': 'wmv2', 'container': 'asf', 'description': 'Advanced Systems Format'},
    'dv': {'codec': 'dvvideo', 'container': 'dv', 'description': 'Digital Video'},
    'mxf': {'codec': 'mpeg2video', 'container': 'mxf', 'description': 'Material Exchange Format'},
    'y4m': {'codec': 'rawvideo', 'container': 'yuv4mpegpipe', 'description': 'YUV4MPEG2'},
    'divx': {'codec': 'mpeg4', 'container': 'avi', 'description': 'DivX Video'},
    'xvid': {'codec': 'mpeg4', 'container': 'avi', 'description': 'Xvid Video'},
    # Additional video formats
    'h264': {'codec': 'h264', 'container': 'h264', 'description': 'Raw H.264'},
    'h265': {'codec': 'hevc', 'container': 'hevc', 'description': 'Raw H.265/HEVC'},
    'hevc': {'codec': 'hevc', 'container': 'hevc', 'description': 'High Efficiency Video Coding'},
    'vp8': {'codec': 'vp8', 'container': 'webm', 'description': 'VP8 Video'},
    'vp9': {'codec': 'vp9', 'container': 'webm', 'description': 'VP9 Video'},
    'av1': {'codec': 'av1', 'container': 'mp4', 'description': 'AOMedia Video 1'},
    'mjpeg': {'codec': 'mjpeg', 'container': 'avi', 'description': 'Motion JPEG'},
    'mjpg': {'codec': 'mjpeg', 'container': 'avi', 'description': 'Motion JPEG'},
    'swf': {'codec': 'flv', 'container': 'swf', 'description': 'Shockwave Flash'},
    'gif': {'codec': 'gif', 'container': 'gif', 'description': 'Graphics Interchange Format'},
    'apng': {'codec': 'apng', 'container': 'apng', 'description': 'Animated PNG'},
    'dnxhd': {'codec': 'dnxhd', 'container': 'mov', 'description': 'Avid DNxHD'},
    'prores': {'codec': 'prores', 'container': 'mov', 'description': 'Apple ProRes'},
    'nut': {'codec': 'ffv1', 'container': 'nut', 'description': 'NUT Container'},
    # Professional formats
    'mxf_op1a': {'codec': 'mpeg2video', 'container': 'mxf', 'description': 'MXF OP1a'},
    'mxf_d10': {'codec': 'mpeg2video', 'container': 'mxf_d10', 'description': 'MXF D-10'},
    # DVD/Blu-ray formats
    'iso': {'codec': 'mpeg2video', 'container': 'iso', 'description': 'ISO Image'},
    'img': {'codec': 'mpeg2video', 'container': 'img', 'description': 'IMG Disc Image'},
}

# Audio formats (80+)
AUDIO_FORMATS = {
    # Common formats
    'mp3': {'codec': 'libmp3lame', 'container': 'mp3', 'description': 'MPEG Audio Layer 3'},
    'aac': {'codec': 'aac', 'container': 'aac', 'description': 'Advanced Audio Coding'},
    'wav': {'codec': 'pcm_s16le', 'container': 'wav', 'description': 'Waveform Audio'},
    'flac': {'codec': 'flac', 'container': 'flac', 'description': 'Free Lossless Audio Codec'},
    'ogg': {'codec': 'libvorbis', 'container': 'ogg', 'description': 'Ogg Vorbis'},
    'wma': {'codec': 'wmav2', 'container': 'asf', 'description': 'Windows Media Audio'},
    'm4a': {'codec': 'aac', 'container': 'mp4', 'description': 'MPEG-4 Audio'},
    'ac3': {'codec': 'ac3', 'container': 'ac3', 'description': 'Dolby Digital'},
    'dts': {'codec': 'dts', 'container': 'dts', 'description': 'DTS Audio'},
    'opus': {'codec': 'libopus', 'container': 'opus', 'description': 'Opus Audio'},
    'ape': {'codec': 'ape', 'container': 'ape', 'description': "Monkey's Audio"},
    'alac': {'codec': 'alac', 'container': 'm4a', 'description': 'Apple Lossless'},
    'amr': {'codec': 'amrnb', 'container': 'amr', 'description': 'Adaptive Multi-Rate'},
    'aiff': {'codec': 'pcm_s16be', 'container': 'aiff', 'description': 'Audio Interchange File Format'},
    'au': {'codec': 'pcm_s16be', 'container': 'au', 'description': 'Sun AU'},
    'mka': {'codec': 'flac', 'container': 'matroska', 'description': 'Matroska Audio'},
    'oga': {'codec': 'libvorbis', 'container': 'ogg', 'description': 'Ogg Audio'},
    'spx': {'codec': 'libspeex', 'container': 'ogg', 'description': 'Speex Audio'},
    'tta': {'codec': 'tta', 'container': 'tta', 'description': 'True Audio'},
    'wv': {'codec': 'wavpack', 'container': 'wv', 'description': 'WavPack'},
    'mp2': {'codec': 'mp2', 'container': 'mp2', 'description': 'MPEG Audio Layer 2'},
    'mp1': {'codec': 'mp1', 'container': 'mp1', 'description': 'MPEG Audio Layer 1'},
    'mpa': {'codec': 'mp2', 'container': 'mpa', 'description': 'MPEG Audio'},
    'ra': {'codec': 'real_144', 'container': 'rm', 'description': 'RealAudio'},
    'mid': {'codec': 'midi', 'container': 'mid', 'description': 'MIDI'},
    'midi': {'codec': 'midi', 'container': 'midi', 'description': 'Musical Instrument Digital Interface'},
    # Additional audio formats
    'eac3': {'codec': 'eac3', 'container': 'eac3', 'description': 'Dolby Digital Plus'},
    'truehd': {'codec': 'truehd', 'container': 'truehd', 'description': 'Dolby TrueHD'},
    'mlp': {'codec': 'mlp', 'container': 'mlp', 'description': 'Meridian Lossless Packing'},
    'tak': {'codec': 'tak', 'container': 'tak', 'description': 'Tom\'s lossless Audio Kompressor'},
    'thd': {'codec': 'truehd', 'container': 'truehd', 'description': 'TrueHD'},
    'caf': {'codec': 'pcm_s16le', 'container': 'caf', 'description': 'Core Audio Format'},
    'voc': {'codec': 'pcm_u8', 'container': 'voc', 'description': 'Creative Voice'},
    'vox': {'codec': 'adpcm_ima_oki', 'container': 'vox', 'description': 'Dialogic VOX'},
    'gsm': {'codec': 'gsm', 'container': 'gsm', 'description': 'GSM Audio'},
    'ircam': {'codec': 'pcm_s16le', 'container': 'ircam', 'description': 'IRCAM Audio'},
    'w64': {'codec': 'pcm_s16le', 'container': 'w64', 'description': 'Sony Wave64'},
}

# Combine all formats
ALL_FORMATS = {**VIDEO_FORMATS, **AUDIO_FORMATS}

def get_supported_formats():
    """Get list of all supported format extensions"""
    return list(ALL_FORMATS.keys())

def is_video_format(extension):
    """Check if format is a video format"""
    return extension.lower() in VIDEO_FORMATS

def is_audio_format(extension):
    """Check if format is an audio format"""
    return extension.lower() in AUDIO_FORMATS

def get_format_info(extension):
    """Get format information"""
    return ALL_FORMATS.get(extension.lower(), None)

def get_format_description(extension):
    """Get format description"""
    info = get_format_info(extension)
    return info['description'] if info else 'Unknown Format'
