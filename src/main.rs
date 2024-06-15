use clap::{App, Arg};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Taup")
        .version("0.1")
        .author("ykk2")
        .about("Terminal-based audio player")
        .arg(
            Arg::with_name("FILE")
                .help("Path to the audio file")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file_path = matches.value_of("FILE").unwrap();

    play_audio(file_path).await?;
    Ok(())
}

async fn play_audio(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let source = Decoder::new(reader)
        .map_err(|e| {
            eprintln!("Error decoding audio file: {}", e);
            e
        })
        .unwrap()
        .pausable(false);

    println!("Playing {:?}s", source.total_duration().unwrap().as_secs());

    sink.append(source);
    sink.play();
    sink.sleep_until_end();

    Ok(())
}
