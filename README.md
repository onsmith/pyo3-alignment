# PyO3 doesn't respect struct memory alignment requirements

Minimal example to demonstrate PyO3 memory alignment issue.

Run `test.bash` to reproduce (assumes `python3` and `cargo` are on the `$PATH`).

## PyO3/Rust code

Define a Python module called `foopy` containing a Python class named `StructWithAlignedFields`:

```rust
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
```

## Python code

Import the compiled `foopy` module and create a `StructWithAlignedFields` instance.

```python
import foopy as foo

foo.StructWithAlignedFields()
```

## Testing bash code

Compile `foopy`, copy the compiled `.so` out of the `target` dir, and run `script.py`.

```bash
#!/bin/bash

# build
cargo build --manifest-path foopy/Cargo.toml
cp ./foopy/target/debug/libfoopy.so ./foopy.so

# run
python3 script.py
```

## Results

When executing `test.bash` on my x86_64 machine, I get the following panic:

```
misaligned pointer dereference: address must be a multiple of 0x40 but is 0x7f8a02ef9f30
```
