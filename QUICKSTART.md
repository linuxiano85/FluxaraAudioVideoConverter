# Quick Start Guide - NovaAudioVideoConverter

## Installazione Rapida (Ubuntu/Debian)

```bash
# 1. Clona il repository
git clone https://github.com/linuxiano85/NovaAudioVideoConverter.git
cd NovaAudioVideoConverter

# 2. Esegui lo script di installazione
chmod +x install.sh
./install.sh

# 3. Avvia l'applicazione
python3 main.py
```

## Primo Utilizzo

### 1. Conversione Base

1. Apri l'applicazione
2. Vai alla scheda "Convert"
3. Clicca "Browse..." per selezionare il file di input
4. Clicca "Browse..." per selezionare dove salvare l'output
5. Scegli un profilo dispositivo o personalizza le impostazioni
6. Clicca "Start Conversion"

### 2. Ripping DVD

1. Vai alla scheda "DVD Ripper"
2. Inserisci il percorso del DVD (es. /dev/dvd) o file ISO
3. Seleziona il titolo da estrarre
4. Assicurati che "Remove Copy Protection" sia selezionato
5. Clicca "Rip DVD"

### 3. Editing Video

1. Vai alla scheda "Video Editor"
2. Seleziona l'operazione desiderata:
   - **Cut/Trim**: Taglia parti del video
   - **Rotate**: Ruota il video
   - **Crop**: Ritaglia il video
   - **Merge**: Unisci più video

### 4. Miglioramento AI

1. Vai alla scheda "AI Enhancement"
2. Per l'audio:
   - "Remove Audio Noise": Rimuove il rumore di fondo
   - "Enhance Voice": Migliora la voce
   - "Repair Damaged Audio": Ripara audio danneggiato
   - "Normalize Audio Levels": Normalizza il volume

3. Per il video:
   - "Remove Video Noise": Rimuove il rumore video
   - "Sharpen Video": Aumenta la nitidezza
   - "Stabilize Video": Stabilizza video mossi
   - "Upscale Video": Aumenta la risoluzione con AI
   - "Repair Damaged Video": Ripara video danneggiato

### 5. Sottotitoli

1. Vai alla scheda "Subtitles"
2. Inserisci il nome del film nella casella di ricerca
3. Seleziona la lingua
4. Clicca "Search" per cercare sottotitoli online
5. Seleziona un risultato e scaricalo
6. Aggiungi i sottotitoli al video:
   - "Burn Subtitle": Brucia i sottotitoli nel video (permanente)
   - "Add Subtitle Stream": Aggiunge come traccia separata (opzionale)

## Esempi da Riga di Comando

Vedi il file `examples.py` per esempi di codice Python:

```bash
python3 examples.py
```

## Profili Dispositivo più Usati

- **iPhone 15 Pro**: Per iPhone recenti
- **Samsung Galaxy S24 Ultra**: Per Samsung Galaxy recenti
- **YouTube 1080p**: Per upload su YouTube in Full HD
- **4K Ultra HD**: Per TV 4K
- **Full HD 1080p**: Standard Full HD universale

## Formati più Comuni

### Video
- MP4 (universale)
- MKV (alta qualità)
- AVI (compatibilità)
- MOV (Apple)
- WebM (web)

### Audio
- MP3 (universale)
- AAC (alta qualità)
- FLAC (lossless)
- OGG (open source)
- WAV (non compresso)

## Risoluzione Problemi

### FFmpeg non trovato
```bash
# Ubuntu/Debian
sudo apt install ffmpeg

# Fedora
sudo dnf install ffmpeg

# Arch Linux
sudo pacman -S ffmpeg
```

### HandBrake non trovato
```bash
# Ubuntu/Debian
sudo apt install handbrake-cli

# Fedora
sudo dnf install handbrake-cli

# Arch Linux
sudo pacman -S handbrake-cli
```

### Errore con DVD protetti
```bash
# Ubuntu/Debian
sudo apt install libdvdcss2

# Per altre distribuzioni, cerca libdvdcss nel gestore pacchetti
```

### PyQt5 non trovato
```bash
pip3 install --user PyQt5
```

## Tips & Tricks

### Accelerazione Hardware
- Abilita "Use Hardware Acceleration" nella scheda Convert
- Richiede GPU compatibile (NVIDIA/AMD/Intel)
- Velocizza la conversione fino a 10x

### Preset di Qualità
- **ultrafast**: Veloce ma qualità ridotta
- **fast**: Buon compromesso velocità/qualità
- **medium**: Bilanciato (consigliato)
- **slow**: Alta qualità ma lento
- **veryslow**: Massima qualità, molto lento

### Conversione Batch
Per convertire più file:
1. Usa lo script Python con un loop
2. O esegui conversioni multiple in sequenza dalla GUI

### Salvataggio Spazio Disco
- Usa codec HEVC (H.265) invece di H.264 per video 4K
- Riduci il bitrate per file più piccoli
- Usa profili ottimizzati per il dispositivo target

## Supporto

- GitHub Issues: https://github.com/linuxiano85/NovaAudioVideoConverter/issues
- Documentazione completa: README.md
- Esempi di codice: examples.py

## Prossimi Passi

1. Esplora tutte le 5 schede dell'applicazione
2. Prova diversi profili dispositivo
3. Sperimenta con le funzioni AI
4. Leggi la documentazione completa in README.md
