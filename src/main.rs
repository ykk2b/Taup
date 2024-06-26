use clap::{App, Arg};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::{self, File};
use std::io::{stdin, BufReader};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Taup")
        .version("0.1.1")
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
    let mut files = Vec::new();

    let currently_playing = Arc::new(Mutex::new(String::new()));

    if is_dir {
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
        println!("{:?}", sink.len());
        if let Some(file_name) = files
            .get(0)
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
        {
            let mut current = currently_playing.lock().unwrap();
            *current = file_name.to_string();
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
            let mut current = currently_playing.lock().unwrap();
            *current = file_name_str.to_string();
            println!("Playing: {}", file_name_str);
        }
    }

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let args: Vec<&str> = input.trim_end().split(" ").collect();
        match args[0] {
            "help" => {
                if args.len() > 1 {
                    match args[1] {
                        "play" | "unpause" | "p" => {
                            println!("\n Play");
                            println!("- aliases: play, unpause, p");
                            println!("- description: Pause the audio");
                        }
                        "mute" | "silence" | "t" => {
                            println!("\n Mute");
                            println!("- aliases: mute, silence, t");
                            println!("- description: Toggle between mute/unmute");
                        }
                        "next" | "skip" | "n" => {
                            println!("\n Next");
                            println!("- aliases: next, skip, n");
                            println!("- description: Play the next audio in the queue");
                        }
                        "volume" | "vol" | "v" => {
                            println!("\n Volume");
                            println!("- aliases: volume, vol, v");
                            println!("- description: Get the audio volume");
                        }
                        "raise" | "incr" | "r" => {
                            println!("\n Raise");
                            println!("- aliases: raise, incr, r");
                            println!("- description: Raise the volume by (volume) or 20%");
                        }
                        "lower" | "decr" | "l" => {
                            println!("\n Lower");
                            println!("- aliases: lower, decr, l");
                            println!("- description: Lower the volume by (volume) or 20%");
                        }
                        "shuffle" | "random" | "h" => {
                            println!("\n Shuffle");
                            println!("- aliases: shuffle, random, h");
                            println!("- description: Toggle between shuffle/no shuffle");
                        }
                        "exit" | "end" | "e" => {
                            println!("\n Exit");
                            println!("- aliases: exit, end, e");
                            println!("- description: Stop the audio player");
                        }
                        _ => {}
                    }
                } else {
                    println!("List of commands:");
                    println!(
                        "- help (command): Display this help message, or more info about the command"
                    );
                    println!("- play: Play/Unpause the audio");
                    println!("- pause: Pause the audio");
                    println!("- mute: Toggle between mute/unmute");
                    println!("- next: Play the next audio in the queue");
                    println!("- volume: Get the audio volume");
                    println!("- raise (volume): Raise the volume by (volume) or 20%");
                    println!("- lower (volume): Lower the volume by (volume) or 20%");
                    println!("- shuffle: Toggle between shuffle/no shuffle");
                    println!("- exit: Stop the audio player");
                }
            }
            "play" | "unpause" | "p" => {
                if sink.is_paused() {
                    sink.play()
                } else {
                    sink.pause()
                }
            }
            "mute" | "silence" | "t" => {
                if sink.volume() == 0.0 {
                    sink.set_volume(1.0)
                } else {
                    sink.set_volume(0.0)
                }
            }
            "next" | "skip" | "n" => {
                if is_dir {
                    sink.skip_one();
                    if let Some(file_name) = path.file_name() {
                        let file_name_str = file_name.to_str().unwrap_or("Unknown");
                        let mut current = currently_playing.lock().unwrap();
                        if *current != file_name_str {
                            println!("Playing: {}", file_name_str);
                            *current = file_name_str.to_string();
                        }
                    }
                } else {
                    println!("You can't skip");
                }
            }
            "volume" | "vol" | "v" => {
                println!("volume: {}%", (sink.volume() * 100.0).round());
            }
            "raise" | "incr" | "r" => {
                if args.len() > 1 {
                    let volume_change: f32 =
                        args[1].parse().expect("Volume change must be a number");

                    if sink.volume() >= 1.0 - volume_change.round() / 100.0 {
                        sink.set_volume(1.0)
                    } else {
                        sink.set_volume(sink.volume() + volume_change.round() / 100.0)
                    }
                } else {
                    if sink.volume() >= 0.8 {
                        sink.set_volume(1.0)
                    } else {
                        sink.set_volume(sink.volume() + 0.2)
                    }
                }

                println!("volume: {}%", (sink.volume() * 100.0).round());
            }
            "lower" | "decr" | "l" => {
                if args.len() > 1 {
                    let volume_change: f32 =
                        args[1].parse().expect("Volume change must be a number");

                    if sink.volume() >= volume_change.round() / 100.0 {
                        sink.set_volume(0.0)
                    } else {
                        sink.set_volume(sink.volume() - volume_change.round() / 100.0)
                    }
                } else {
                    if sink.volume() >= 0.8 {
                        sink.set_volume(0.0)
                    } else {
                        sink.set_volume(sink.volume() - 0.2)
                    }
                }

                println!("volume: {}%", (sink.volume() * 100.0).round());
            }
            "shuffle" | "random" | "h" => {
                shuffle = !shuffle;
                if shuffle {
                    println!("shuffle enabled")
                } else {
                    println!("shuffle disabled")
                }
            }
            "exit" => return Ok(()),
            _ => println!("Command not recognized. Try 'help'."),
        }

        input.clear();
    }
}
