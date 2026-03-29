// Этап 11 — ML Inference: MLEngine, VisionPerceptor, AudioPerceptor, Guardian ML filters
use axiom_agent::ml::engine::{MLEngine, MLDetection, MLError};
use axiom_agent::channels::vision::{
    VisionPerceptor, detection_to_command, parse_detections, pixels_to_tensor,
    VISION_DEFAULT_DOMAIN,
};
use axiom_agent::channels::audio::{
    AudioPerceptor, frame_rms, analyze_frames, vad_frame_to_command, VAD_FRAME_SIZE,
};
use axiom_runtime::{Perceptor, Guardian};
use axiom_ucl::OpCode;

// ─── MLEngine mock ────────────────────────────────────────────────────────────

#[test]
fn test_mlengine_mock_infer() {
    let engine = MLEngine::mock(vec![4], vec![0.9, 0.05, 0.03, 0.02]);
    let result = engine.infer(&[0.1, 0.2, 0.3, 0.4]).unwrap();
    assert_eq!(result, vec![0.9, 0.05, 0.03, 0.02]);
}

#[test]
fn test_mlengine_mock_shape_mismatch() {
    let engine = MLEngine::mock(vec![4], vec![0.5, 0.5]);
    let err = engine.infer(&[0.1, 0.2]).unwrap_err();
    // shape mismatch: expected 4, got 2
    assert!(matches!(err, MLError::ShapeMismatch { expected: 4, got: 2 }));
}

#[test]
fn test_mlengine_mock_zero_input_size_accepts_any() {
    // input_shape = [0] → произведение = 0 → size check skipped
    let engine = MLEngine::mock(vec![0], vec![1.0]);
    let result = engine.infer(&[0.1; 100]).unwrap();
    assert_eq!(result, vec![1.0]);
}

#[test]
fn test_mlengine_input_size() {
    let engine = MLEngine::mock(vec![3, 224, 224], vec![]);
    assert_eq!(engine.input_size(), 3 * 224 * 224);
}

#[test]
fn test_mlengine_output_size() {
    let engine = MLEngine::mock(vec![1], vec![0.1, 0.2, 0.3]);
    assert_eq!(engine.output_size(), 3);
}

#[test]
fn test_mlengine_input_shape() {
    let engine = MLEngine::mock(vec![3, 224, 224], vec![]);
    assert_eq!(engine.input_shape(), &[3, 224, 224]);
}

#[test]
fn test_mlengine_load_missing_file() {
    let result = MLEngine::load(std::path::Path::new("/nonexistent.onnx"));
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(matches!(err, MLError::ModelNotFound(_) | MLError::NotEnabled));
}

// ─── parse_detections ─────────────────────────────────────────────────────────

#[test]
fn test_parse_detections_above_threshold() {
    // 2 детекции: [class, confidence, x, y, w, h]
    let output = vec![
        0.0, 0.9, 10.0, 20.0, 50.0, 60.0, // class 0, conf 0.9 ✓
        1.0, 0.3, 5.0, 5.0, 10.0, 10.0,   // class 1, conf 0.3 ✗ (< 0.5)
        2.0, 0.7, 100.0, 100.0, 30.0, 30.0, // class 2, conf 0.7 ✓
    ];
    let dets = parse_detections(&output, 0.5);
    assert_eq!(dets.len(), 2);
    assert_eq!(dets[0].class_id, 0);
    assert!((dets[0].confidence - 0.9).abs() < 1e-6);
    assert_eq!(dets[1].class_id, 2);
}

#[test]
fn test_parse_detections_all_below_threshold() {
    let output = vec![0.0, 0.1, 0.0, 0.0, 1.0, 1.0];
    let dets = parse_detections(&output, 0.5);
    assert!(dets.is_empty());
}

#[test]
fn test_parse_detections_incomplete_chunk_ignored() {
    let output = vec![0.0, 0.9, 10.0, 20.0]; // только 4 float → не кратно 6
    let dets = parse_detections(&output, 0.5);
    assert!(dets.is_empty());
}

// ─── detection_to_command ─────────────────────────────────────────────────────

#[test]
fn test_detection_to_command_inject_token() {
    let det = MLDetection { class_id: 0, confidence: 0.8, bbox: [0.0; 4] };
    let cmd = detection_to_command(&det, VISION_DEFAULT_DOMAIN);
    assert_eq!(cmd.opcode, OpCode::InjectToken as u16);
    assert_eq!(cmd.target_id, VISION_DEFAULT_DOMAIN);
    assert_eq!(cmd.priority, (0.8 * 255.0) as u8);
}

#[test]
fn test_detection_confidence_clamp() {
    let det = MLDetection { class_id: 0, confidence: 2.0, bbox: [0.0; 4] }; // > 1.0
    let cmd = detection_to_command(&det, 106);
    assert_eq!(cmd.priority, 255); // clamped
}

// ─── pixels_to_tensor ────────────────────────────────────────────────────────

#[test]
fn test_pixels_to_tensor_normalization() {
    let pixels = vec![0u8, 128, 255];
    let tensor = pixels_to_tensor(&pixels);
    assert!((tensor[0] - 0.0).abs() < 1e-6);
    assert!((tensor[1] - 128.0 / 255.0).abs() < 1e-3);
    assert!((tensor[2] - 1.0).abs() < 1e-6);
}

// ─── VisionPerceptor ──────────────────────────────────────────────────────────

#[test]
fn test_vision_perceptor_feed_detections() {
    let engine = MLEngine::mock(vec![0], vec![]);
    let mut p = VisionPerceptor::new(engine);
    p.feed_detections(vec![
        MLDetection { class_id: 0, confidence: 0.9, bbox: [0.0; 4] },
        MLDetection { class_id: 1, confidence: 0.7, bbox: [0.0; 4] },
    ]);
    assert_eq!(p.pending_count(), 2);
    assert_eq!(p.receive().unwrap().opcode, OpCode::InjectToken as u16);
    assert_eq!(p.receive().unwrap().opcode, OpCode::InjectToken as u16);
    assert!(p.receive().is_none());
}

#[test]
fn test_vision_perceptor_domain() {
    let engine = MLEngine::mock(vec![0], vec![]);
    let mut p = VisionPerceptor::new(engine).with_domain(104); // MAP
    p.feed_detections(vec![
        MLDetection { class_id: 0, confidence: 0.9, bbox: [0.0; 4] },
    ]);
    let cmd = p.receive().unwrap();
    assert_eq!(cmd.target_id, 104);
}

#[test]
fn test_vision_perceptor_name() {
    let engine = MLEngine::mock(vec![0], vec![]);
    let p = VisionPerceptor::new(engine);
    assert_eq!(p.name(), "vision");
}

#[test]
fn test_vision_process_image_with_mock_engine() {
    // Создаём тестовое изображение в памяти
    let img = image::RgbaImage::from_pixel(4, 4, image::Rgba([128, 64, 32, 255]));
    let tmp = std::env::temp_dir().join("axiom_test_img.png");
    img.save(&tmp).unwrap();

    // MLEngine.mock с 6 float выходом = 1 детекция
    let engine = MLEngine::mock(vec![0], vec![0.0, 0.8, 10.0, 10.0, 5.0, 5.0]);
    let mut p = VisionPerceptor::new(engine).with_threshold(0.5);
    let count = p.process_image(&tmp).unwrap();
    assert_eq!(count, 1);
    assert_eq!(p.pending_count(), 1);

    std::fs::remove_file(tmp).ok();
}

// ─── Audio VAD ───────────────────────────────────────────────────────────────

#[test]
fn test_frame_rms_silence() {
    let silence = vec![0i16; VAD_FRAME_SIZE];
    assert!((frame_rms(&silence) - 0.0).abs() < 1e-6);
}

#[test]
fn test_frame_rms_max_signal() {
    let loud = vec![i16::MAX; VAD_FRAME_SIZE];
    let rms = frame_rms(&loud);
    assert!((rms - 1.0).abs() < 0.001);
}

#[test]
fn test_analyze_frames_detects_speech() {
    // Первый фрейм тишина, второй — громкий
    let mut samples = vec![0i16; VAD_FRAME_SIZE];
    samples.extend(vec![i16::MAX / 2; VAD_FRAME_SIZE]);

    let frames = analyze_frames(&samples, 0.1);
    assert_eq!(frames.len(), 2);
    assert!(!frames[0].is_speech); // тишина
    assert!(frames[1].is_speech);  // речь
}

#[test]
fn test_analyze_frames_empty() {
    let frames = analyze_frames(&[], 0.1);
    assert!(frames.is_empty());
}

#[test]
fn test_vad_frame_to_command() {
    let frame = axiom_agent::channels::audio::VadFrame {
        frame_idx: 0,
        energy: 0.5,
        is_speech: true,
    };
    let cmd = vad_frame_to_command(&frame, 100);
    assert_eq!(cmd.opcode, OpCode::InjectToken as u16);
    assert_eq!(cmd.target_id, 100);
    assert_eq!(cmd.priority, (0.5 * 255.0) as u8);
}

// ─── AudioPerceptor ───────────────────────────────────────────────────────────

#[test]
fn test_audio_perceptor_feed_silence() {
    let mut p = AudioPerceptor::new().with_threshold(0.01);
    let silence = vec![0i16; VAD_FRAME_SIZE * 3];
    let count = p.feed_samples(&silence);
    assert_eq!(count, 0);
    assert_eq!(p.pending_count(), 0);
}

#[test]
fn test_audio_perceptor_feed_speech() {
    let mut p = AudioPerceptor::new().with_threshold(0.01);
    let speech: Vec<i16> = (0..VAD_FRAME_SIZE).map(|i| ((i % 100) as i16) * 200).collect();
    let count = p.feed_samples(&speech);
    assert!(count > 0);
    assert!(p.pending_count() > 0);
}

#[test]
fn test_audio_perceptor_receive() {
    let mut p = AudioPerceptor::new().with_threshold(0.01);
    let speech = vec![i16::MAX / 4; VAD_FRAME_SIZE];
    p.feed_samples(&speech);
    let cmd = p.receive().unwrap();
    assert_eq!(cmd.opcode, OpCode::InjectToken as u16);
}

#[test]
fn test_audio_perceptor_name() {
    assert_eq!(AudioPerceptor::new().name(), "audio");
}

#[test]
fn test_audio_process_wav() {
    // Создаём минимальный WAV файл (тишина)
    let tmp = std::env::temp_dir().join("axiom_test.wav");
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&tmp, spec).unwrap();
    for _ in 0..(VAD_FRAME_SIZE * 3) {
        writer.write_sample(0i16).unwrap();
    }
    writer.finalize().unwrap();

    let mut p = AudioPerceptor::new();
    let count = p.process_wav(&tmp).unwrap();
    assert_eq!(count, 0); // тишина
    std::fs::remove_file(tmp).ok();
}

// ─── Guardian ML filters ──────────────────────────────────────────────────────

#[test]
fn test_guardian_validate_confidence_ok() {
    assert!(Guardian::validate_ml_confidence(0.9, 0.5));
    assert!(Guardian::validate_ml_confidence(0.5, 0.5)); // граница
    assert!(Guardian::validate_ml_confidence(0.99, 0.5)); // верхняя граница
}

#[test]
fn test_guardian_validate_confidence_below_threshold() {
    assert!(!Guardian::validate_ml_confidence(0.3, 0.5));
    assert!(!Guardian::validate_ml_confidence(0.0, 0.5));
}

#[test]
fn test_guardian_validate_confidence_adversarial() {
    // > 0.99 — adversarial defense
    assert!(!Guardian::validate_ml_confidence(0.999, 0.5));
    assert!(!Guardian::validate_ml_confidence(1.0, 0.5));
}

#[test]
fn test_guardian_validate_ml_output_all_valid() {
    let output = vec![0.9, 0.8, 0.7];
    assert!(Guardian::validate_ml_output(&output, 0.5));
}

#[test]
fn test_guardian_validate_ml_output_one_invalid() {
    let output = vec![0.9, 0.3, 0.8]; // 0.3 < 0.5
    assert!(!Guardian::validate_ml_output(&output, 0.5));
}

#[test]
fn test_guardian_validate_ml_output_empty() {
    assert!(!Guardian::validate_ml_output(&[], 0.5));
}

#[test]
fn test_guardian_validate_ml_output_adversarial() {
    let output = vec![0.9, 0.999]; // 0.999 > 0.99
    assert!(!Guardian::validate_ml_output(&output, 0.5));
}
