use clap::{App, Arg};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::{stdin, BufReader};

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
        .pausable(false)
        .repeat_infinite();

    sink.append(source);
    sink.play();
    println!("Type 'help' to get the list of commands. Type 'exit' to stop the player.");

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "help" => {
                println!("List of commands:");
                println!("- help: Display this help message.");
                // TODO: println!("- status: Display configuration");
                // TODO: println!("- repeate: Toggle between repeate/no repeate");
                println!("- mute: Toggle between mute/unmute");
                // TODO: println!("- next: Play the next audio in the queue");
                // TODO: println!("- prev: Play the previous audio in the queue");
                println!("- play: Play/Unpause the audio");
                println!("- pause: Pause the audio");
                // TODO: println!("- fw: Go forward with 5s");
                // TODO: println!("- bw (time): Go backward with 5s");
                println!("- raise: Raise the volume");
                println!("- lower: Lower the volume");
                println!("- exit: Stop the audio player.");
            }
            "repeate" => {}
            "mute" => {
                if sink.volume() == 0.0 {
                    sink.set_volume(1.0)
                } else {
                    sink.set_volume(0.0)
                }
            }
            "play" => {
                if sink.is_paused() {
                    sink.play()
                } else {
                    sink.pause()
                }
            }
            "raise" => {
                if sink.volume() >= 0.8 {
                    sink.set_volume(1.0)
                } else {
                    sink.set_volume(sink.volume() + 0.2)
                }

                println!("volume: {}%", (sink.volume() * 100.0).round());
            }
            "lower" => {
                if sink.volume() <= 0.2 {
                    sink.set_volume(0.0)
                } else {
                    sink.set_volume(sink.volume() - 0.2)
                }
                println!("volume: {}%", (sink.volume() * 100.0).round());
            }
            "pause" => sink.pause(),
            "exit" => return Ok(()),
            _ => println!("Command not recognized. Try 'help'."),
        }

        input.clear();
    }
}
