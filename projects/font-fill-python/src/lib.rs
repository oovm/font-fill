use font_fill::{FontFillResult, FontFillVideo, Rgba};

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn string_sum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

fn main() -> FontFillResult<()> {
    let font = r"F:\project-a\client\ProjectA\Assets\Art\Fonts\SourceHanSansSC-Regular.otf";
    let mut file = FontFillVideo::create("../../target/out.av1", font, 200).unwrap();
    for c in include_str!("../../font-fill/tests/长恨歌.txt").chars() {
        if !c.is_alphabetic() {
            continue;
        }
        println!("encode: {}", c);
        file.encode_frame(c, Rgba([1.0, 1.0, 1.0, 1.0]))?;
    }
    file.report_json()
}
