use clap::Parser;
use opencv::{
    highgui,
    prelude::*,
    videoio,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    cam: i32
}


fn main() -> opencv::Result<()>{
    let args = Args::parse();

    let window = "virtual camera capture";
    highgui::named_window(window, highgui::WINDOW_FULLSCREEN)?;

    let mut cam = videoio::VideoCapture::new(args.cam, videoio::CAP_ANY)?;
    cam.set(videoio::CAP_PROP_FRAME_WIDTH, 1920f64)?;
    cam.set(videoio::CAP_PROP_FRAME_HEIGHT, 1080f64)?;

    let mut frame = Mat::default();

    if !cam.is_opened()? {
        panic!("Unable to capture any video from camera!")
    }

    println!("Capturing video now...");

    while cam.is_opened()? {
        cam.read(&mut frame)?;
        highgui::imshow(window, &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 113 { // quit with q
            break;
        }
    }
    
    Ok(())
}
