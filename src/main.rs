use cpython::{PyDict, PyResult, Python};
use rodio::Sink;
use rodio::Source;
use std::io::BufReader;
use std::time::Duration;

fn main() -> PyResult<()> {
    let gil = Python::acquire_gil();
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    let filename = "data/Pop-Rock-Loop1.wav";
    let music = load_music(gil.python(), filename)?;
    let beats: Vec<f32> = find_beats(gil.python(), music)?;
    let intervals: Vec<u64> = beats_to_intervals(beats);
    play_beats(&sink, intervals);
    sink.sleep_until_end();
    Ok(())
}

fn beats_to_intervals(beats: Vec<f32>) -> Vec<u64> {
    let mut intervals: Vec<u64> = vec![];
    for i in 0..(beats.len() - 1) {
        let interval = beats[i + 1] - beats[i];
        intervals.push((interval * 1000.0) as u64);
    }
    intervals
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
fn find_beats(py: Python, music: Vec<f32>) -> PyResult<Vec<f32>> {
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

fn play_beats(sink: &Sink, intervals: Vec<u64>) {
    let file = std::fs::File::open("data/beat.wav").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file))
        .unwrap()
        .buffered();
    intervals.iter().for_each(|interval| {
        let s = source.clone().delay(Duration::from_millis(*interval));
        sink.append(s);
    });
}
