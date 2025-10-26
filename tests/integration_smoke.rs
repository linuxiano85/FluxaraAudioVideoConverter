use std::process::Command;

/// Test that FFmpeg is available
#[test]
fn test_ffmpeg_available() {
    let output = Command::new("ffmpeg").arg("-version").output();

    assert!(output.is_ok(), "FFmpeg should be installed");
    if let Ok(output) = output {
        assert!(output.status.success(), "FFmpeg should run successfully");
    }
}

/// Test that FFprobe is available
#[test]
fn test_ffprobe_available() {
    let output = Command::new("ffprobe").arg("-version").output();

    assert!(output.is_ok(), "FFprobe should be installed");
    if let Ok(output) = output {
        assert!(output.status.success(), "FFprobe should run successfully");
    }
}

/// Test generating a synthetic audio file using lavfi
#[test]
fn test_generate_synthetic_audio() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_audio.wav");

    // Generate 1 second of 440Hz sine wave
    let output = Command::new("ffmpeg")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("sine=frequency=440:duration=1")
        .arg("-y")
        .arg(&test_file)
        .output();

    assert!(output.is_ok(), "Should generate synthetic audio");
    if let Ok(output) = output {
        assert!(
            output.status.success(),
            "FFmpeg should complete successfully"
        );
        assert!(test_file.exists(), "Output file should exist");

        // Clean up
        let _ = std::fs::remove_file(&test_file);
    }
}

/// Test generating a synthetic video file using lavfi
#[test]
fn test_generate_synthetic_video() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_video.mp4");

    // Generate 1 second of test pattern
    let output = Command::new("ffmpeg")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("testsrc=duration=1:size=320x240:rate=30")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("sine=frequency=1000:duration=1")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-y")
        .arg(&test_file)
        .output();

    assert!(output.is_ok(), "Should generate synthetic video");
    if let Ok(output) = output {
        assert!(
            output.status.success(),
            "FFmpeg should complete successfully"
        );
        assert!(test_file.exists(), "Output file should exist");

        // Clean up
        let _ = std::fs::remove_file(&test_file);
    }
}

/// Test basic audio conversion
#[test]
fn test_audio_conversion() {
    let temp_dir = std::env::temp_dir();
    let input_file = temp_dir.join("test_input.wav");
    let output_file = temp_dir.join("test_output.mp3");

    // First generate a test file
    let gen = Command::new("ffmpeg")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("sine=frequency=440:duration=1")
        .arg("-y")
        .arg(&input_file)
        .output();

    if gen.is_ok() && input_file.exists() {
        // Try to convert it
        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(&input_file)
            .arg("-c:a")
            .arg("libmp3lame")
            .arg("-b:a")
            .arg("128k")
            .arg("-y")
            .arg(&output_file)
            .output();

        assert!(output.is_ok(), "Should convert audio");
        if let Ok(output) = output {
            assert!(output.status.success(), "Conversion should succeed");
            assert!(output_file.exists(), "Output file should exist");
        }

        // Clean up
        let _ = std::fs::remove_file(&input_file);
        let _ = std::fs::remove_file(&output_file);
    }
}

/// Test video deinterlacing filter
#[test]
fn test_video_deinterlace_filter() {
    let temp_dir = std::env::temp_dir();
    let input_file = temp_dir.join("test_interlaced.mp4");
    let output_file = temp_dir.join("test_deinterlaced.mp4");

    // Generate interlaced test video
    let gen = Command::new("ffmpeg")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("testsrc=duration=1:size=320x240:rate=30")
        .arg("-vf")
        .arg("tinterlace=mode=interlacex2")
        .arg("-y")
        .arg(&input_file)
        .output();

    if gen.is_ok() && input_file.exists() {
        // Apply bwdif filter
        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(&input_file)
            .arg("-vf")
            .arg("bwdif")
            .arg("-c:v")
            .arg("libx264")
            .arg("-preset")
            .arg("ultrafast")
            .arg("-y")
            .arg(&output_file)
            .output();

        assert!(output.is_ok(), "Should deinterlace video");
        if let Ok(output) = output {
            assert!(output.status.success(), "Deinterlacing should succeed");
            assert!(output_file.exists(), "Output file should exist");
        }

        // Clean up
        let _ = std::fs::remove_file(&input_file);
        let _ = std::fs::remove_file(&output_file);
    }
}

/// Test audio denoise filter
#[test]
fn test_audio_denoise_filter() {
    let temp_dir = std::env::temp_dir();
    let input_file = temp_dir.join("test_noisy_audio.wav");
    let output_file = temp_dir.join("test_clean_audio.wav");

    // Generate test audio
    let gen = Command::new("ffmpeg")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("sine=frequency=440:duration=1")
        .arg("-y")
        .arg(&input_file)
        .output();

    if gen.is_ok() && input_file.exists() {
        // Apply afftdn filter
        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(&input_file)
            .arg("-af")
            .arg("afftdn")
            .arg("-y")
            .arg(&output_file)
            .output();

        assert!(output.is_ok(), "Should denoise audio");
        if let Ok(output) = output {
            assert!(output.status.success(), "Denoising should succeed");
            assert!(output_file.exists(), "Output file should exist");
        }

        // Clean up
        let _ = std::fs::remove_file(&input_file);
        let _ = std::fs::remove_file(&output_file);
    }
}

/// Test loudness normalization
#[test]
fn test_loudness_normalization() {
    let temp_dir = std::env::temp_dir();
    let input_file = temp_dir.join("test_loud_audio.wav");
    let output_file = temp_dir.join("test_normalized_audio.wav");

    // Generate test audio
    let gen = Command::new("ffmpeg")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("sine=frequency=440:duration=1")
        .arg("-y")
        .arg(&input_file)
        .output();

    if gen.is_ok() && input_file.exists() {
        // Apply loudnorm filter
        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(&input_file)
            .arg("-af")
            .arg("loudnorm")
            .arg("-y")
            .arg(&output_file)
            .output();

        assert!(output.is_ok(), "Should normalize audio");
        if let Ok(output) = output {
            assert!(output.status.success(), "Normalization should succeed");
            assert!(output_file.exists(), "Output file should exist");
        }

        // Clean up
        let _ = std::fs::remove_file(&input_file);
        let _ = std::fs::remove_file(&output_file);
    }
}
