use gst::prelude::*;

#[path = "../tutorial-commons.rs"]
mod tutorials_common;

fn tutorial_main() {
    gst::init().unwrap();

    let uri = "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";
    let pipeline = gst::parse_launch(&format!("playbin uri={}", uri)).unwrap();
    pipeline.set_state(gst::State::Playing).expect("unalble to set the pipeline to the playing state.");

    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) =>break,
            MessageView::Error(err) => {
                println!("Error from {:?} {} {:?}", err.src().map(|s| s.path_string()), err.error(), err.debug());
                break;
            },
            other => {
                println!("{:?}", other);
            },
        };
    };

    pipeline.set_state(gst::State::Null).expect("unable to set pipeline state to the null.");
}

fn main() {
    tutorials_common::run(tutorial_main);
}
