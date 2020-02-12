use cpython::{PyDict, PyResult, Python};

pub fn find_beats(filename: &str) -> PyResult<Vec<f32>> {
    let gil = Python::acquire_gil();
    let music = load_music(gil.python(), filename)?;
    let py = gil.python();
    let locals = PyDict::new(py);
    locals.set_item(py, "madmom", py.import("madmom")?)?;
    locals.set_item(py, "np", py.import("numpy")?)?;
    locals.set_item(py, "music", &music)?;
    locals.set_item(py, "fps", 100)?;
    let proc = py.eval(
        "madmom.features.beats.DBNBeatTrackingProcessor(fps=fps)",
        None,
        Some(&locals),
    )?;
    locals.set_item(py, "proc", &proc)?;
    let act = py.eval(
        "madmom.features.beats.RNNBeatProcessor()(np.array(music))",
        None,
        Some(&locals),
    )?;
    locals.set_item(py, "act", &act)?;
    py.eval("proc(act)", None, Some(&locals))?
        .extract::<Vec<f32>>(py)
}

fn load_music(py: Python, filename: &str) -> PyResult<Vec<f32>> {
    let locals = PyDict::new(py);
    locals.set_item(py, "filename", filename)?;
    locals.set_item(py, "librosa", py.import("librosa")?)?;
    let (music, _) = py
        .eval("librosa.load(filename)", None, Some(&locals))?
        .extract::<(Vec<f32>, usize)>(py)?;
    Ok(music)
}

pub fn beats_to_intervals(beats: Vec<f32>, scale: f32) -> Vec<u64> {
    println!("{:?}", beats);
    let mut intervals: Vec<u64> = vec![];
    for i in 0..(beats.len() - 1) {
        let interval = beats[i] - beats[i + 1];
        intervals.push((interval * scale) as u64);
    }
    intervals
}
