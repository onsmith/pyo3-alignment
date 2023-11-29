use pyo3::types::PyModule;
use pyo3::{pyclass, pymethods, pymodule, PyResult, Python};

#[derive(Clone)]
#[repr(align(64))]
struct Aligned64<T>(T);

#[pyclass]
pub(crate) struct StructWithAlignedFields {
    aligned_field: Aligned64<u64>,
}

#[pymethods]
impl StructWithAlignedFields {
    #[new]
    pub(crate) fn new() -> PyResult<Self> {
        Ok(StructWithAlignedFields {
            aligned_field: Aligned64(0),
        })
    }
}

#[pymodule]
fn foopy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<StructWithAlignedFields>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Aligned64, StructWithAlignedFields};

    #[test]
    fn can_create_struct_from_rust() {
        StructWithAlignedFields {
            aligned_field: Aligned64(0),
        };
    }
}
