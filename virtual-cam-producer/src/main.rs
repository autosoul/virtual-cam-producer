use std::time::Duration;
use clap::Parser;
use opencv::{
    core,
    highgui,
    prelude::*,
    videoio,
    imgcodecs
};
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    cam: i32,
    #[arg(short, long)]
    bootstrap_servers: String,
    #[arg(short, long)]
    topic: String
}

fn encode(mat: &impl core::ToInputArray) -> opencv::Result<core::Vector<u8>> {
    let mut encoded_frame = core::Vector::new();
    let params = core::Vector::new();

    imgcodecs::imencode(".png", mat, &mut encoded_frame, &params)?;

    Ok(encoded_frame)
}

fn decode(buf: &impl core::ToInputArray) -> opencv::Result<Mat> {
    imgcodecs::imdecode(buf, imgcodecs::IMREAD_COLOR)
}

#[tokio::main]
async fn main() -> opencv::Result<()>{
    let args = Args::parse();

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", &args.bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

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

        // Encode and send to Kafka
        let encoded_frame = encode(&frame)?;
        let delivery_result = producer.send(
            FutureRecord::to(&args.topic)
                .key(&())
                .payload(encoded_frame.as_slice()),
            Duration::from_secs(0)
        )
            .await
            .expect("Failed to send frame to broker");

        let key = highgui::wait_key(1)?;
        if key == 113 { // quit with q
            break;
        }
    }
    
    Ok(())
}
