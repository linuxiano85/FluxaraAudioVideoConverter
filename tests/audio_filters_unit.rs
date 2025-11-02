use std::path::PathBuf;

use fluxara_avc::audio::{
    build_audio_filters, enhance_audio, enhance_audio_only, AudioEnhanceOptions,
};

fn temp_file(name: &str) -> PathBuf {
    std::env::temp_dir().join(name)
}

#[test]
fn should_build_filters_with_defaults() {
    let opts = AudioEnhanceOptions::default();
    let filters = build_audio_filters(&opts);

    // Defaults include: highpass (80), afftdn, agate with -50dB, acompressor, loudnorm
    assert!(filters.iter().any(|f| f.starts_with("highpass=f=80")));
    assert!(filters.iter().any(|f| f.starts_with("afftdn")));
    assert!(filters.iter().any(|f| f.contains("agate=threshold=-50")));
    assert!(filters.iter().any(|f| f.starts_with("acompressor")));
    assert!(filters.iter().any(|f| f.starts_with("loudnorm")));
}

#[test]
fn should_build_filters_respecting_all_options() {
    let opts = AudioEnhanceOptions {
        denoise: false,
        normalize: false,
        highpass_freq: Some(60),
        lowpass_freq: Some(12000),
        notch_freq: Some(50),
        compressor: false,
        gate: true,
        gate_threshold: -42.5,
    };

    let filters = build_audio_filters(&opts);

    // Included
    assert!(filters.iter().any(|f| f == "highpass=f=60"));
    assert!(filters.iter().any(|f| f == "lowpass=f=12000"));
    assert!(filters
        .iter()
        .any(|f| f == "equalizer=f=50:width_type=h:width=10:g=-20"));
    assert!(filters
        .iter()
        .any(|f| f == "agate=threshold=-42.5dB:ratio=3:attack=20:release=250"));

    // Excluded
    assert!(!filters.iter().any(|f| f.starts_with("afftdn")));
    assert!(!filters.iter().any(|f| f.starts_with("acompressor")));
    assert!(!filters.iter().any(|f| f.starts_with("loudnorm")));
}

#[test]
fn should_build_filters_without_high_or_lowpass_when_none() {
    let opts = AudioEnhanceOptions {
        denoise: true,
        normalize: true,
        highpass_freq: None,
        lowpass_freq: None,
        notch_freq: None,
        compressor: true,
        gate: false,
        gate_threshold: -50.0,
    };

    let filters = build_audio_filters(&opts);
    assert!(!filters.iter().any(|f| f.starts_with("highpass")));
    assert!(!filters.iter().any(|f| f.starts_with("lowpass")));
    assert!(!filters.iter().any(|f| f.starts_with("equalizer")));
    assert!(filters.iter().any(|f| f.starts_with("afftdn")));
    assert!(filters.iter().any(|f| f.starts_with("acompressor")));
    assert!(filters.iter().any(|f| f.starts_with("loudnorm")));
}

#[test]
fn should_enhance_audio_video_copy_stream() {
    // Integration-like test that the command executes with video stream copied
    // Generate short synthetic video with audio
    let input = temp_file("avc_test_input_av.mp4");
    let output = temp_file("avc_test_output_av.mp4");

    // create input via ffmpeg lavfi
    let gen = std::process::Command::new("ffmpeg")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("testsrc=duration=1:size=160x120:rate=15")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("sine=frequency=440:duration=1")
        .arg("-shortest")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-y")
        .arg(&input)
        .output();

    if gen.is_err() {
        panic!("ffmpeg not found or failed to start: {:?}", gen.err());
    }
    let gen = gen.unwrap();
    if !gen.status.success() {
        eprintln!("ffmpeg stdout: {}", String::from_utf8_lossy(&gen.stdout));
        eprintln!("ffmpeg stderr: {}", String::from_utf8_lossy(&gen.stderr));
        panic!("ffmpeg failed to generate input video: exit {}", gen.status);
    }

    let opts = AudioEnhanceOptions::default();
    let res = enhance_audio(&input, &output, &opts);
    assert!(res.is_ok(), "enhance_audio should succeed");
    assert!(output.exists(), "output file should exist");

    let _ = std::fs::remove_file(input);
    let _ = std::fs::remove_file(output);
}

#[test]
fn should_enhance_audio_only_to_lossless_flac() {
    let input = temp_file("avc_test_input_audio.wav");
    let output = temp_file("avc_test_output_audio.flac");

    // create input via ffmpeg lavfi
    let gen = std::process::Command::new("ffmpeg")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("sine=frequency=440:duration=1")
        .arg("-y")
        .arg(&input)
        .output();

    if gen.is_err() {
        panic!("ffmpeg not found or failed to start: {:?}", gen.err());
    }
    let gen = gen.unwrap();
    if !gen.status.success() {
        eprintln!("ffmpeg stdout: {}", String::from_utf8_lossy(&gen.stdout));
        eprintln!("ffmpeg stderr: {}", String::from_utf8_lossy(&gen.stderr));
        panic!("ffmpeg failed to generate input audio: exit {}", gen.status);
    }

    let opts = AudioEnhanceOptions {
        denoise: true,
        normalize: true,
        highpass_freq: Some(100),
        lowpass_freq: Some(10000),
        notch_freq: None,
        compressor: true,
        gate: false,
        gate_threshold: -40.0,
    };

    let res = enhance_audio_only(&input, &output, &opts);
    assert!(res.is_ok(), "enhance_audio_only should succeed");
    assert!(output.exists(), "output file should exist");

    let _ = std::fs::remove_file(input);
    let _ = std::fs::remove_file(output);
}
