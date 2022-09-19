use gst::prelude::*;

#[path = "./tutorials_common.rs"]
mod tutorials_common;

fn tutorial_main() {
    gst::init().unwrap();

    let pipeline = gst::Pipeline::new(Some("test-pipeline"));
    let v_source = gst::ElementFactory::make("videotestsrc", Some("v_source")).expect("error videotestsrc");
    let v_sink = gst::ElementFactory::make("autovideosink", Some("v_sink")).expect("error autovideosink");
    let a_source = gst::ElementFactory::make("audiotestsrc", Some("a_source")).expect("error audiotestsrc");
    let a_convert = gst::ElementFactory::make("audioconvert", Some("a_convert")).expect("error audioconvert");
    let a_sink = gst::ElementFactory::make("autoaudiosink", Some("a_sink")).expect("error autoaudiosink");
    let capsfilter = gst::ElementFactory::make("capsfilter", None).expect("error capsfilter");

    pipeline.add_many(&[&v_source, &capsfilter,  &v_sink]);
    gst::Element::link_many(&[&v_source, &capsfilter,  &v_sink]);
    pipeline.add_many(&[&a_source, &a_convert,  &a_sink]);
    gst::Element::link_many(&[&a_source, &a_convert, &a_sink]);

    v_source.set_property_from_str("pattern", "ball");
    capsfilter.set_property(
        "caps",
        gst::Caps::builder("video/x-raw")
        .field("format", "I420")
        .field("width", 720)
        .field("height", 480)
        .field("framerate", gst::Fraction::new(30, 1))
        .build(),
        );


    pipeline.set_state(gst::State::Playing).expect("set state");

    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Error(err) => {
                println!("{:?} {} {:?}", err.src().map(|s| s.path_string()), err.error(), err.debug());
                break;
            },
            MessageView::Eos(..) => break,
            _ => {},
        };
    }

    pipeline.set_state(gst::State::Null).expect("set to null");
}

fn main() {
    tutorials_common::run(tutorial_main);
}
