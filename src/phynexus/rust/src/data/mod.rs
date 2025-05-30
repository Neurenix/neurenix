
pub mod dataset_hub;
pub mod arrow;
pub mod parquet;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::error::PhynexusError;

pub fn register_data(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let data = PyModule::new(py, "data")?;
    
    dataset_hub::register_dataset_hub(py, data)?;
    arrow::register_arrow(py, data)?;
    parquet::register_parquet(py, data)?;
    
    m.add_submodule(&data)?;
    
    Ok(())
}
