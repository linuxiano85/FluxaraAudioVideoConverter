# NovaAudioVideoConverter

**Professional Audio and Video Converter for Linux with AI Enhancement**

NovaAudioVideoConverter è un'applicazione completa per Linux che permette di convertire DVD in file digitali, pulire audio danneggiato, migliorare video rovinati, rimuovere protezioni da DVD e molto altro, il tutto assistito da tecnologie AI.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Python](https://img.shields.io/badge/python-3.7+-blue.svg)
![Platform](https://img.shields.io/badge/platform-Linux-lightgrey.svg)

## 🌟 Caratteristiche Principali

### 🎬 Conversione Universale
- **Supporto per oltre 180 formati** audio e video
- **Profili ottimizzati per oltre 200 dispositivi** (iPhone, Samsung, iPad, PlayStation, Xbox, Smart TV, ecc.)
- **Conversione ultra-rapida** con supporto per accelerazione hardware
- **Unione istantanea di file** senza perdita di qualità (lossless)
- Conversione batch per processare più file contemporaneamente

### 📀 DVD Ripper
- Estrazione da DVD a file digitali
- **Rimozione protezioni da DVD**
- Supporto per file ISO
- Selezione titoli e tracce audio/sottotitoli
- Backup completo della struttura DVD

### ✂️ Video Editor
- **Taglio e trimming** di video preciso
- **Rotazione** video (90°, 180°, 270°)
- **Ritaglio (crop)** video personalizzato
- Unione di più video in un unico file
- Applicazione di effetti e filtri avanzati

### 🤖 AI Enhancement (Miglioramento con IA)

#### Audio Enhancement
- **Pulizia audio rovinato** con rimozione intelligente del rumore
- Rimozione di click e pop
- Normalizzazione automatica del volume
- Miglioramento della voce
- Compressione dinamica
- Equalizzazione con preset ottimizzati

#### Video Enhancement
- **Miglioramento video rovinati** con denoise avanzato
- Sharpening (nitidezza) intelligente
- **Stabilizzazione video** per riprese mosse
- **Upscaling AI** per aumentare la risoluzione
- Deinterlacciamento
- Miglioramento colori (contrasto, luminosità, saturazione)
- Rimozione watermark

### 📝 Sottotitoli
- **Ricerca sottotitoli online** da database internazionali
- Download automatico dei sottotitoli
- Aggiunta sottotitoli hard-sub (bruciati nel video)
- Aggiunta sottotitoli soft-sub (come traccia separata)
- Conversione tra formati sottotitoli (SRT, ASS, VTT, ecc.)
- Estrazione sottotitoli da file video
- Supporto per oltre 15 lingue

### 🎯 Profili Dispositivo
Oltre 200 profili pre-configurati per:
- **Smartphone**: iPhone (6-15), Samsung Galaxy (S10-S24), Google Pixel, Xiaomi, Oppo, Vivo, OnePlus, Huawei, Realme, Motorola, Nokia
- **Tablet**: iPad Pro/Air, Samsung Galaxy Tab, Lenovo, Amazon Fire, Huawei
- **Console**: PlayStation 4/5, Xbox One/Series X
- **Smart TV**: LG, Samsung, Sony, Panasonic (HD, Full HD, 4K)
- **Standard**: 4K Ultra HD, Full HD 1080p, HD 720p, SD 480p
- **Web/Streaming**: YouTube (4K/1080p/720p), Facebook, Instagram, TikTok

## 📋 Requisiti di Sistema

### Sistema Operativo
- Linux (Ubuntu, Debian, Fedora, Arch Linux, ecc.)

### Dipendenze
- Python 3.7 o superiore
- FFmpeg (con supporto per tutti i codec)
- HandBrake CLI (per ripping DVD)
- libdvdread e libdvdcss (per leggere DVD protetti)

### Python Packages
```
PyQt5>=5.15.0
ffmpeg-python>=0.2.0
opencv-python>=4.5.0
numpy>=1.21.0
Pillow>=8.3.0
pydub>=0.25.0
beautifulsoup4>=4.9.3
requests>=2.26.0
lxml>=4.6.3
```

## 🚀 Installazione

### 1. Installare le dipendenze di sistema

#### Ubuntu/Debian:
```bash
sudo apt update
sudo apt install -y python3 python3-pip ffmpeg handbrake-cli libdvdread-dev libdvdcss2
```

#### Fedora:
```bash
sudo dnf install python3 python3-pip ffmpeg handbrake-cli libdvdread-devel libdvdcss
```

#### Arch Linux:
```bash
sudo pacman -S python python-pip ffmpeg handbrake-cli libdvdread libdvdcss
```

### 2. Clonare il repository
```bash
git clone https://github.com/linuxiano85/NovaAudioVideoConverter.git
cd NovaAudioVideoConverter
```

### 3. Installare le dipendenze Python
```bash
pip3 install -r requirements.txt
```

### 4. Eseguire l'applicazione
```bash
python3 main.py
```

## 💻 Utilizzo

### Interfaccia Grafica (GUI)
Avviare l'applicazione con:
```bash
python3 main.py
```

L'interfaccia presenta 5 schede principali:
1. **Convert**: Conversione base con selezione formato e profilo dispositivo
2. **DVD Ripper**: Estrazione e conversione da DVD
3. **Video Editor**: Operazioni di editing (taglio, rotazione, crop)
4. **AI Enhancement**: Miglioramento audio e video con IA
5. **Subtitles**: Gestione sottotitoli

### Esempi di Utilizzo

#### Conversione Base
```python
from src.core.converter import Converter

converter = Converter()
converter.convert(
    input_file='input.avi',
    output_file='output.mp4',
    video_codec='h264',
    audio_codec='aac'
)
```

#### Ripping DVD
```python
from src.core.dvd_ripper import DVDRipper

ripper = DVDRipper()
ripper.rip_dvd(
    dvd_path='/dev/dvd',
    output_file='movie.mp4',
    title=1,
    remove_protection=True
)
```

#### Miglioramento Audio con IA
```python
from src.ai.audio_enhancer import AudioEnhancer

enhancer = AudioEnhancer()
enhancer.repair_damaged_audio('damaged_audio.mp3', 'repaired_audio.mp3')
```

#### Miglioramento Video con IA
```python
from src.ai.video_enhancer import VideoEnhancer

enhancer = VideoEnhancer()
enhancer.repair_damaged_video('damaged_video.mp4', 'repaired_video.mp4')
```

#### Ricerca Sottotitoli
```python
from src.utils.subtitle_search import SubtitleSearcher

searcher = SubtitleSearcher()
results = searcher.search_subtitles('Movie Name', language='it')
```

## 🎨 Formati Supportati

### Video (100+)
MP4, AVI, MKV, MOV, WMV, FLV, WebM, MPG, MPEG, M4V, 3GP, 3G2, OGV, VOB, TS, MTS, M2TS, F4V, RM, RMVB, ASF, DV, MXF, H264, H265, HEVC, VP8, VP9, AV1, ProRes, DNxHD, e molti altri...

### Audio (80+)
MP3, AAC, WAV, FLAC, OGG, WMA, M4A, AC3, DTS, Opus, APE, ALAC, AMR, AIFF, AU, MKA, OGA, WV, MP2, RA, MIDI, EAC3, TrueHD, e molti altri...

## 🔧 Configurazione

### Hardware Acceleration
Per abilitare l'accelerazione hardware (NVIDIA/AMD/Intel):
```python
converter.convert(
    input_file='input.mp4',
    output_file='output.mp4',
    hwaccel=True
)
```

### Preset di Velocità
Bilanciamento tra velocità e qualità:
- `ultrafast`: Massima velocità, qualità minore
- `fast`: Veloce con buona qualità
- `medium`: Bilanciato (default)
- `slow`: Lento ma alta qualità
- `veryslow`: Massima qualità, molto lento

## 🤝 Contribuire

I contributi sono benvenuti! Per contribuire:
1. Fork del repository
2. Crea un branch per la tua feature (`git checkout -b feature/AmazingFeature`)
3. Commit delle modifiche (`git commit -m 'Add some AmazingFeature'`)
4. Push al branch (`git push origin feature/AmazingFeature`)
5. Apri una Pull Request

## 📝 Licenza

Questo progetto è rilasciato sotto licenza MIT. Vedi il file `LICENSE` per maggiori dettagli.

## 🐛 Segnalazione Bug

Per segnalare bug o richiedere nuove funzionalità, aprire una issue su GitHub:
https://github.com/linuxiano85/NovaAudioVideoConverter/issues

## 📧 Contatti

Per domande o supporto, contattare:
- GitHub Issues: https://github.com/linuxiano85/NovaAudioVideoConverter/issues

## 🙏 Ringraziamenti

NovaAudioVideoConverter utilizza le seguenti librerie open source:
- FFmpeg - Framework multimedia completo
- HandBrake - Convertitore video open source
- PyQt5 - Framework GUI per Python
- OpenCV - Computer vision e image processing
- E molte altre...

## 🎯 Roadmap

### Versione 1.1
- [ ] Integrazione con servizi di sottotitoli online (OpenSubtitles API)
- [ ] Supporto per profili personalizzati
- [ ] Miglioramento dell'interfaccia grafica
- [ ] Supporto per conversione Blu-ray

### Versione 1.2
- [ ] Integrazione modelli AI più avanzati (super-resolution)
- [ ] Editing audio avanzato
- [ ] Timeline per editing video
- [ ] Supporto per streaming online

### Versione 2.0
- [ ] Web interface
- [ ] Server mode per conversioni remote
- [ ] API REST per integrazione con altri software
- [ ] Plugin system per estensioni

---

**Sviluppato con ❤️ per la comunità Linux**
