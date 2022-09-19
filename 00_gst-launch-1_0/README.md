# Gstreamer Basics

This page is basic sample of gstreamer pipeline.

[More detail](https://qiita.com/alivelime/items/50d796c09baabb765625)

# install on Mac.

```shell
$ brew install gstreamer \
  gst-libav \
  gst-plugins-bad \
  gst-plugins-base \
  gst-plugins-good \
  gst-plugins-ugly

# download sample movie.
$ wget https://mirror.clarkson.edu/blender/demo/movies/BBB/bbb_sunflower_1080p_30fps_normal.mp4
```

# simple pipeline. Src and Sink

## do nothin.

```shell
$ gst-launch-1.0 fakesrc ! fakesink
```

## video test src

```shell
$ gst-launch-1.0 videotestsrc ! autovideosink
```

## show video camera

<img width=320 src="https://camo.qiitausercontent.com/00b7678312e44d323e9173b43380bd508335cde5/68747470733a2f2f71696974612d696d6167652d73746f72652e73332e61702d6e6f727468656173742d312e616d617a6f6e6177732e636f6d2f302f39313731302f63363634363966322d316665362d343237382d653262632d6261386662363635393738312e706e67" />

```shell
$ gst-launch-1.0 avfvideosrc ! autovideosin

# play mic sound?  not work!
$ gst-launch-1.0 audiotestsrc ! autoaudiosink
```

# Src, filter and Sinkd

## print datetime in movie

```shell
$ gst-launch-1.0 videotestsrc ! clockoverlay auto-resize=false time-format="%Y-%m-%d %H:%M:%S" ! autovideosink
```

## resize 1280 x 720

```shell
$ gst-launch-1.0 avfvideosrc ! video/x-raw,width=1280,height=720 ! autovideosink
```

## play test audio

```shell
$ gst-launch-1.0 audiotestsrc ! audioconvert ! autoaudiosink
```

## play mic sound. it works

```shell
$ gst-launch-1.0 autoaudiosrc ! audioconvert ! audioresample ! autoaudiosink
```

# filter (mux, demux, tee, queue, capsfilter)

## demux split video and audio

### play Big Buck Bummy

<img width=320 src="https://camo.qiitausercontent.com/140c68f8bdb95080fe453ccb27f563c6a1c01526/68747470733a2f2f71696974612d696d6167652d73746f72652e73332e61702d6e6f727468656173742d312e616d617a6f6e6177732e636f6d2f302f39313731302f62383931363562652d303136652d376563342d346630332d3930313135383834383130362e706e67" />

```shell
$  gst-launch-1.0 filesrc location="./bbb_sunflower_1080p_30fps_normal.mp4" ! qtdemux name=demux \
    demux. ! queue ! avdec_h264 ! autovideosink \
    demux. ! queue ! mpegaudioparse ! avdec_mp3  ! audioconvert ! audioresample ! autoaudiosink
```

### receive SRT stream and write out DASH

```shell
$ gst-launch-1.0 -v srtserversrc uri="srt://:4201?mode=listener" ! tsdemux name=demux \
    demux. ! queue ! h264parse ! dash.video_0 \
    demux. ! queue ! audio/mpeg ! aacparse ! dash.audio_0 \
    dashsink name=dash mpd-root-path=/media muxer=ts dynamic=true target-duration=1
```

## demux combine video and audio

### camera and mic to MP4 file

```shell
$ gst-launch-1.0 -e autovideosrc ! videoconvert ! x264enc tune=zerolatency speed-preset=ultrafast bitrate=500 ! h264parse ! queue ! mux. \
    autoaudiosrc ! audioconvert ! audioresample ! faac bitrate=192000 ! queue ! mux. \
    qtmux name=mux ! filesink location="./cam_mic.mp4"
```

POINT : add `-e` option, or get broken file

```shell
-e, --eos-on-shutdown             Force EOS on sources before shutting the pipeline down
```

