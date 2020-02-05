use cpython::{PyDict, PyResult, Python, ToPyObject};

fn main() -> PyResult<()> {
    let gil = Python::acquire_gil();
    let filename = "data/Pop-Rock-Loop1.wav";
    let beats = find_beats(gil.python(), filename)?;
    println!("{:?}", beats);
    Ok(())
}

fn find_beats(py: Python, filename: &str) -> PyResult<Vec<f32>> {
    let locals = PyDict::new(py);
    locals.set_item(py, "filename", filename);
    locals.set_item(py, "librosa", py.import("librosa")?)?;
    locals.set_item(py, "madmom", py.import("madmom")?)?;
    locals.set_item(py, "np", py.import("numpy")?)?;
    let (music, sr) = py
        .eval("librosa.load(filename)", None, Some(&locals))?
        .extract::<(Vec<f32>, usize)>(py)?;
    locals.set_item(py, "music", &music);
    locals.set_item(py, "sr", &sr);

    let proc = py.eval(
        "madmom.features.beats.DBNBeatTrackingProcessor(fps=100)",
        None,
        Some(&locals),
    )?;
    locals.set_item(py, "proc", &proc);
    let act = py.eval(
        "madmom.features.beats.RNNBeatProcessor()(np.array(music))",
        None,
        Some(&locals),
    )?;
    locals.set_item(py, "act", &act);
    py.eval("proc(act)", None, Some(&locals))?
        .extract::<Vec<f32>>(py)
}
