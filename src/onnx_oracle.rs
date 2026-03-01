use ort::{
    session::Session,
    value::Value,
};
use std::path::Path;
use anyhow::{Result, Context};
use std::sync::Once;

static INIT_ORT: Once = Once::new();

/// Pastikan ORT sudah diinisialisasi
fn ensure_ort_initialized() {
    INIT_ORT.call_once(|| {
        let _ = ort::init()
            .with_name("omnilang_oracle")
            .commit();
    });
}

/// Jalankan inferensi pada model ONNX dengan input tensor. 
/// Mengembalikan output tensor sebagai vektor flat.
pub fn run_inference(
    model_path: &Path,
    inputs_data: Vec<(Vec<usize>, Vec<f32>)>,
) -> Result<Vec<Vec<f32>>> {
    ensure_ort_initialized();
    
    // Konfigurasi session (Execution provider bisa disesuaikan, misal CUDA jika ada)
    let mut session = Session::builder()?
        .commit_from_file(model_path)
        .context(format!("Failed to load ONNX model from: {:?}", model_path))?;

    let expected_inputs = session.inputs();
    if inputs_data.len() != expected_inputs.len() {
        anyhow::bail!("ONNX model expects {} inputs, but got {}", expected_inputs.len(), inputs_data.len());
    }

    let mut input_values = Vec::new();
    for (i, (expected, (in_shape, in_data))) in expected_inputs.iter().zip(inputs_data.into_iter()).enumerate() {
        // Build tensor
        let shape_i64: Vec<i64> = in_shape.iter().map(|&x| x as i64).collect();
        let tensor = Value::from_array((shape_i64, in_data))?;
        input_values.push((expected.name().to_string(), tensor));
    }

    // Mengambil run context
    let outputs = session.run(input_values)?;
    if outputs.len() == 0 {
        return Err(anyhow::anyhow!("ONNX inference returned empty outputs."));
    }

    let mut output_vectors = Vec::new();
    for i in 0..outputs.len() {
        let output = outputs[i].try_extract_tensor::<f32>()?;
        output_vectors.push(output.1.to_vec());
    }
    
    Ok(output_vectors)
}
