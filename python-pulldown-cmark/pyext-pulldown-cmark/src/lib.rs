use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use pyo3::{wrap_pyfunction, wrap_pymodule};
use pulldown_cmark::{Parser, Options, Event, html};

#[pyclass]
struct OptionFlags {
    options: Options,
}

#[pymethods]
impl OptionFlags {
    #[new]
    fn new() -> Self {
        let options = Options::all();
        OptionFlags { options }
    }
}

#[pymodule]
fn options(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<OptionFlags>()?;
    Ok(())
}

#[pyfunction]
fn to_html(_py: Python, buffer: String) -> PyResult<String> {
    let opts = OptionFlags::new();
    let parser = Parser::new_ext(buffer.as_str(), opts.options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(html_output)
}

#[pyfunction]
fn get_offsets(_py: Python, buffer: String) -> PyResult<&PyDict> {
    let opts = OptionFlags::new();
    let parser = Parser::new_ext(buffer.as_str(), opts.options);
    let starts = PyDict::new(_py);
    let mut i: u32 = 1;
    for (event, range) in parser.into_offset_iter() {
        if let Event::Start(tag) = event {
            let tagstr = format!("{:#?}", tag);
            let rangestr = format!("{:?}", range);
            let data = PyTuple::new(_py, vec![rangestr, tagstr]);
            starts.set_item(i, data)?;
            i += 1;
        }
    }
    Ok(starts)
}

#[pymodule]
fn libpulldowncmark(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(options))?;

    m.add_wrapped(wrap_pyfunction!(to_html))?;
    m.add_wrapped(wrap_pyfunction!(get_offsets))?;

    Ok(())
}
