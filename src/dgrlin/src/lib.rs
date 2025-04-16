use pyo3::prelude::*;

pub mod opcode;
pub mod compiler;
pub mod decompiler;

#[pyfunction]
fn compile(filename: String) -> eyre::Result<()> {
    compiler::text_to_byte(filename)?;
    Ok(())
}

#[pyfunction]
fn decompile(filename: String) -> eyre::Result<()> {
    decompiler::byte_to_text(filename)?;
    Ok(())
}


#[pymodule]
fn dgrlin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(compile, m)?)?;
    m.add_function(wrap_pyfunction!(decompile, m)?)?;
    Ok(())
}
