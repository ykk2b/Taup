use clap::{App, Arg};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::{self, File};
use std::io::{stdin, BufReader};
use std::path::{Path, PathBuf};

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

fn shuffle_files(files: &mut [PathBuf]) {
    let n = files.len();
    for i in (0..n).rev() {
        let j = rand::random::<usize>() % (i + 1);
        files.swap(i, j);
    }
}
async fn play_audio(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let is_dir = Path::new(file_path).is_dir();
    let path = Path::new(file_path);
    let mut shuffle = true;
    if is_dir {
        let mut files = Vec::new();
        for entry in fs::read_dir(file_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("mp3") {
                files.push(path);
            }
        }
        if shuffle {
            shuffle_files(&mut files);
        }

        for file in &files {
            let file = File::open(file)?;
            let reader = BufReader::new(file);

            let source = Decoder::new(reader)
                .map_err(|e| {
                    eprintln!("Error decoding audio file: {}", e);
                    e
                })?
                .pausable(false);

            sink.append(source);
        }

        sink.play();
        if let Some(file_name) = files
            .get(0)
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
        {
            println!("Playing: {}", file_name);
        }
    } else {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let source = Decoder::new(reader)
            .map_err(|e| {
                eprintln!("Error decoding audio file: {}", e);
                e
            })?
            .pausable(false)
            .repeat_infinite();

        sink.append(source);
        sink.play();
        if let Some(file_name) = path.file_name() {
            let file_name_str = file_name.to_str().unwrap_or("Unknown");
            println!("Playing: {}", file_name_str);
        }
    }

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "help" => {
                println!("List of commands:");
                println!("- help: Display this help message.");
                println!("- shuffle: Toggle between shuffle/no shuffe");
                println!("- mute: Toggle between mute/unmute");
                println!("- skip: Skip to the next song");
                println!("- next: Play the next audio in the queue");
                println!("- play: Play/Unpause the audio");
                println!("- pause: Pause the audio");
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
            "next" => {
                if is_dir {
                    sink.skip_one();
                    if let Some(file_name) = path.file_name() {
                        let file_name_str = file_name.to_str().unwrap_or("Unknown");
                        println!("Playing: {}", file_name_str);
                    }
                } else {
                    println!("You can't skip");
                }
            }
            "shuffle" => {
                shuffle = !shuffle;
                if shuffle {
                    println!("shuffle enabled")
                } else {
                    println!("shuffle disabled")
                }
            }
            "pause" => sink.pause(),
            "exit" => return Ok(()),
            _ => println!("Command not recognized. Try 'help'."),
        }

        input.clear();
    }
}
