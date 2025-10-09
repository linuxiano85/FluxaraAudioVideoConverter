# NovaAudioVideoConverter - Architettura e Struttura

## 📁 Struttura del Progetto

```
NovaAudioVideoConverter/
├── main.py                    # Entry point dell'applicazione GUI
├── examples.py                # Script di esempio e demo
├── install.sh                 # Script di installazione automatica
├── requirements.txt           # Dipendenze Python
├── README.md                  # Documentazione completa
├── QUICKSTART.md              # Guida rapida
├── LICENSE                    # Licenza MIT
├── .gitignore                 # File da ignorare in git
│
└── src/                       # Codice sorgente
    ├── __init__.py
    │
    ├── core/                  # Moduli di conversione core
    │   ├── __init__.py
    │   ├── converter.py       # Converter principale (FFmpeg)
    │   ├── dvd_ripper.py      # Ripper DVD (HandBrake)
    │   └── formats.py         # 180+ formati supportati
    │
    ├── ai/                    # Moduli di miglioramento AI
    │   ├── __init__.py
    │   ├── audio_enhancer.py  # Miglioramento audio AI
    │   └── video_enhancer.py  # Miglioramento video AI
    │
    ├── gui/                   # Interfaccia grafica
    │   ├── __init__.py
    │   └── main_window.py     # Finestra principale PyQt5
    │
    ├── profiles/              # Profili dispositivo
    │   ├── __init__.py
    │   └── devices.py         # 200+ profili dispositivo
    │
    └── utils/                 # Utilità
        ├── __init__.py
        ├── config.py          # Configurazione app
        ├── helpers.py         # Funzioni di utilità
        └── subtitle_search.py # Ricerca sottotitoli
```

## 🏗️ Architettura del Sistema

```
┌─────────────────────────────────────────────────────────────┐
│                    GUI Layer (PyQt5)                         │
│  ┌──────────┬──────────┬──────────┬──────────┬──────────┐   │
│  │ Convert  │   DVD    │  Video   │   AI     │Subtitles │   │
│  │   Tab    │ Ripper   │  Editor  │Enhancement│   Tab    │   │
│  └──────────┴──────────┴──────────┴──────────┴──────────┘   │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────┴───────────────────────────────────┐
│                    Core Processing Layer                     │
│  ┌──────────┬──────────┬──────────┬──────────┬──────────┐   │
│  │Converter │   DVD    │  Video   │   Audio  │ Subtitle │   │
│  │  Engine  │  Ripper  │ Enhancer │ Enhancer │ Manager  │   │
│  └──────────┴──────────┴──────────┴──────────┴──────────┘   │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────┴───────────────────────────────────┐
│                   External Tools Layer                       │
│  ┌──────────┬──────────┬──────────┬──────────┬──────────┐   │
│  │  FFmpeg  │ HandBrake│  OpenCV  │libdvdcss │   Web    │   │
│  │          │    CLI   │          │          │  APIs    │   │
│  └──────────┴──────────┴──────────┴──────────┴──────────┘   │
└───────────────────────────────────────────────────────────────┘
```

## 🔄 Flusso di Conversione

```
┌──────────┐      ┌──────────┐      ┌──────────┐      ┌──────────┐
│  Input   │─────▶│  Format  │─────▶│ Process  │─────▶│  Output  │
│   File   │      │Detection │      │  Video   │      │   File   │
└──────────┘      └──────────┘      └──────────┘      └──────────┘
                        │                  │
                        ▼                  ▼
                  ┌──────────┐      ┌──────────┐
                  │  Device  │      │    AI    │
                  │ Profile  │      │Enhancement│
                  └──────────┘      └──────────┘
```

## 🎯 Moduli Principali

### 1. Core Converter (`src/core/converter.py`)
**Funzionalità:**
- Conversione formato con FFmpeg
- Supporto hardware acceleration
- Merge file lossless
- Video editing (cut, rotate, crop)
- Applicazione filtri

**Metodi principali:**
```python
convert()        # Conversione completa
merge_files()    # Unione file
cut_video()      # Taglio video
rotate_video()   # Rotazione video
crop_video()     # Ritaglio video
```

### 2. DVD Ripper (`src/core/dvd_ripper.py`)
**Funzionalità:**
- Ripping DVD con HandBrake
- Rimozione protezioni CSS
- Supporto ISO
- Selezione titoli
- Backup struttura DVD

**Metodi principali:**
```python
rip_dvd()           # Rip DVD completo
backup_dvd()        # Backup DVD
convert_iso_to_video() # Converti ISO
```

### 3. AI Audio Enhancer (`src/ai/audio_enhancer.py`)
**Funzionalità:**
- Denoise audio intelligente
- Normalizzazione volume
- Miglioramento voce
- Rimozione click/pop
- Compressione dinamica
- Equalizzazione

**Metodi principali:**
```python
denoise_audio()         # Rimuovi rumore
normalize_audio()       # Normalizza volume
enhance_voice()         # Migliora voce
repair_damaged_audio()  # Ripara audio
```

### 4. AI Video Enhancer (`src/ai/video_enhancer.py`)
**Funzionalità:**
- Denoise video
- Sharpening
- Stabilizzazione
- Upscaling AI
- Deinterlacciamento
- Miglioramento colori

**Metodi principali:**
```python
denoise_video()         # Rimuovi rumore
sharpen_video()         # Aumenta nitidezza
stabilize_video()       # Stabilizza video
upscale_video()         # Upscale AI
repair_damaged_video()  # Ripara video
```

### 5. Device Profiles (`src/profiles/devices.py`)
**200+ Profili per:**
- Smartphone (iPhone, Samsung, Google, Xiaomi, ecc.)
- Tablet (iPad, Samsung Tab, ecc.)
- Console (PlayStation, Xbox)
- Smart TV (LG, Samsung, Sony, ecc.)
- Standard (4K, 1080p, 720p, ecc.)
- Web/Streaming (YouTube, Facebook, Instagram, TikTok)

### 6. Subtitle Search (`src/utils/subtitle_search.py`)
**Funzionalità:**
- Ricerca sottotitoli online
- Download sottotitoli
- Estrazione da video
- Hard-sub (bruciati)
- Soft-sub (traccia separata)
- Conversione formati

## 📊 Formati Supportati

### Video (100+)
- Container: MP4, MKV, AVI, MOV, WebM, FLV, VOB, TS, MXF, ecc.
- Codec: H.264, H.265/HEVC, VP8, VP9, AV1, MPEG-2, MPEG-4, ecc.
- Risoluzione: Da SD a 8K
- Frame rate: Qualsiasi

### Audio (80+)
- Formati: MP3, AAC, FLAC, WAV, OGG, WMA, M4A, ALAC, ecc.
- Codec: MP3, AAC, Vorbis, Opus, FLAC, PCM, ecc.
- Bitrate: Da 64 kbps a 320 kbps e oltre
- Canali: Mono, Stereo, 5.1, 7.1, ecc.

## 🤖 Tecnologie AI Utilizzate

### Audio AI
1. **Adaptive Noise Reduction**: Rimozione rumore adattiva
2. **Loudness Normalization**: LUFS-based normalization
3. **Voice Enhancement**: Filtri ottimizzati per voce
4. **Click/Pop Removal**: Rimozione imperfezioni digitali
5. **Dynamic Range Compression**: Bilanciamento volume

### Video AI
1. **HQ Denoising (HQDN3D)**: Denoise di alta qualità
2. **Unsharp Masking**: Sharpening intelligente
3. **Video Stabilization**: Stabilizzazione a 2 passaggi
4. **Lanczos Upscaling**: Upscaling di alta qualità
5. **Deinterlacing (YADIF)**: Deinterlacciamento adattivo
6. **Color Enhancement**: Miglioramento automatico colori

## 🔧 Configurazione e Personalizzazione

### File di Configurazione
Posizione: `~/.novaaudiovideoconverter/config.json`

**Impostazioni disponibili:**
- Directory output di default
- Accelerazione hardware
- Preset di velocità
- Codec di default
- Bitrate di default
- Lingua sottotitoli
- Tema interfaccia

### Preset di Qualità
- **Low**: 1M video, 96k audio - File piccoli
- **Medium**: 3M video, 128k audio - Bilanciato
- **High**: 5M video, 192k audio - Alta qualità
- **Ultra**: 10M video, 256k audio - Massima qualità

### Preset di Velocità (FFmpeg)
- **ultrafast**: ~10x velocità reale
- **fast**: ~4x velocità reale
- **medium**: ~2x velocità reale (default)
- **slow**: ~1x velocità reale
- **veryslow**: ~0.5x velocità reale

## 📈 Performance

### Benchmark Tipici (Full HD 1080p)
- **Conversione H.264**: ~2x velocità reale (senza HW accel)
- **Conversione H.264**: ~10x velocità reale (con HW accel)
- **Conversione H.265**: ~1x velocità reale
- **Denoise AI**: ~0.8x velocità reale
- **Upscaling 2x**: ~0.5x velocità reale
- **Stabilizzazione**: ~0.6x velocità reale

### Requisiti Sistema Consigliati
- **CPU**: Intel i5 / AMD Ryzen 5 o superiore
- **RAM**: 8 GB minimo, 16 GB consigliato
- **GPU**: NVIDIA/AMD per HW acceleration (opzionale)
- **Spazio disco**: 10 GB liberi (più spazio per file temporanei)

## 🚀 Estensibilità

### Aggiungere Nuovi Formati
Modificare `src/core/formats.py`:
```python
VIDEO_FORMATS['nuovo_formato'] = {
    'codec': 'codec_name',
    'container': 'container_name',
    'description': 'Descrizione'
}
```

### Aggiungere Nuovi Profili Dispositivo
Modificare `src/profiles/devices.py`:
```python
DEVICE_PROFILES['nuovo_dispositivo'] = {
    'name': 'Nome Dispositivo',
    'resolution': '1920x1080',
    'video_codec': 'h264',
    'audio_codec': 'aac',
    'video_bitrate': '5M',
    'audio_bitrate': '192k',
    'container': 'mp4'
}
```

### Aggiungere Nuovi Filtri AI
Implementare in `src/ai/audio_enhancer.py` o `src/ai/video_enhancer.py`

## 📚 Dipendenze

### Dipendenze di Sistema
- **FFmpeg**: Core conversion engine
- **HandBrake CLI**: DVD ripping
- **libdvdread**: Lettura DVD
- **libdvdcss**: Rimozione protezioni DVD

### Dipendenze Python
- **PyQt5**: GUI framework
- **opencv-python**: Computer vision
- **numpy**: Calcoli numerici
- **Pillow**: Image processing
- **requests**: HTTP requests
- **beautifulsoup4**: HTML parsing

## 🔒 Sicurezza e Privacy

- Nessun dato inviato online (tranne ricerca sottotitoli)
- Elaborazione completamente locale
- File temporanei automaticamente eliminati
- Supporto per file protetti da password (ove applicabile)

## 🌍 Internazionalizzazione

### Lingue Supportate per Sottotitoli
Inglese, Italiano, Spagnolo, Francese, Tedesco, Portoghese, Russo, Cinese, Giapponese, Coreano, Arabo, Hindi, Turco, Polacco, Olandese

### Interfaccia
Attualmente in Inglese/Italiano
(Estensibile per altre lingue)

---

**Versione**: 1.0.0  
**Data**: 2024  
**Licenza**: MIT
