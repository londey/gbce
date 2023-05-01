#![allow(dead_code)]

mod machine;

// use std::env;
// use std::fs;

// use machine::Cart;
// use machine::MachineState;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let rom_path = &args[1];
//     let rom_contents = fs::read(rom_path).expect("Something went wrong reading ROM file");

//     let machine = MachineState::new(Cart::new(&rom_contents[..]));

//     machine.step_forwards();

//     println!("Hello, world!");
// }

// Add the following dependencies to your Cargo.toml file
// sdl2 = "0.34.5"
// serde = "1.0"
// serde_json = "1.0"

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};
use std::time::Duration;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const FPS: u64 = 60;
const DIAGNOSTICS_WINDOW_WIDTH: u32 = 300;
const DIAGNOSTICS_WINDOW_HEIGHT: u32 = 200;

struct MonoSound {
    samples: Vec<u8>,
}

impl AudioCallback for MonoSound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for (src, dest) in self.samples.iter().zip(out.iter_mut()) {
            *dest = *src;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init().expect("Failed to initialize SDL");
    let video_subsystem = sdl_context.video().expect("Failed to get video subsystem");
    let audio_subsystem = sdl_context.audio().expect("Failed to get audio subsystem");
    let controller_subsystem = sdl_context.game_controller().expect("Failed to get controller subsystem");

    let window = video_subsystem
        .window("Game Emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("Failed to create game window");

    let diagnostics_window = video_subsystem
        .window(
            "Diagnostics",
            DIAGNOSTICS_WINDOW_WIDTH,
            DIAGNOSTICS_WINDOW_HEIGHT,
        )
        .position_centered()
        .build()
        .expect("Failed to create diagnostics window");

    let mut canvas = window.into_canvas().build().unwrap();
    let _diagnostics_canvas = diagnostics_window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_WIDTH, SCREEN_HEIGHT)
        .expect("Failed to create texture for game window");

    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1),
        samples: None,
    };

    let device: AudioDevice<MonoSound> = audio_subsystem
        .open_playback(None, &desired_spec, |spec| MonoSound {
            samples: vec![0; spec.samples as usize],
        })
        .expect("Failed to open audio device");

    device.resume();

    // Process input from the Xbox controller and keyboard
    let mut controller : Option<sdl2::controller::GameController>   = None;

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::ControllerDeviceAdded { which, .. } => {
                    if controller.is_none() {
                        controller = controller_subsystem.open(which).ok();
                    }
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    if let Some(ref c) = controller {
                        if c.instance_id() == which {
                            controller = None;
                        }
                    }
                }


                _ => {}
            }
        }

        if let Some(ref controller) = controller {
            // Read input from the Xbox controller here
            // Example: press the 'A' button to quit the emulator
            if controller.button(sdl2::controller::Button::Back) {
                break 'running;
            }
        }

        // Your emulation and rendering logic here

        // Example: Fill the texture with a red color
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..SCREEN_HEIGHT as usize {
                    for x in 0..SCREEN_WIDTH as usize {
                        let offset = y * pitch + x * 3;
                        buffer[offset] = 255;
                        buffer[offset + 1] = 0;
                        buffer[offset + 2] = 0;
                    }
                }
            })
            .unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        // Sleep to maintain the desired frame rate
        let frame_duration = Duration::from_millis(1000 / FPS);
        std::thread::sleep(frame_duration);



        // for event in event_pump.poll_iter() {
        //     match event {
        //         Event::KeyDown {
        //             keycode: Some(Keycode::Escape),
        //             ..
        //         } => {
        //             break 'running;
        //         }

        //         // Event::ControllerDeviceAdded { which, .. } => {
        //         //     if controller.is_none() {
        //         //         controller = controller_subsystem.open(which).ok();
        //         //     }
        //         // }
        //         // Event::ControllerDeviceRemoved { which, .. } => {
        //         //     if let Some(ref c) = controller {
        //         //         if c.instance_id() == which {
        //         //             controller = None;
        //         //         }
        //         //     }
        //         // }
        //         Event::KeyDown {
        //             keycode: Some(key), ..
        //         } => {
        //             // Handle keyboard input here
        //             // Example: press 'Q' to quit the emulator
        //             if key == Keycode::Q {
        //                 break 'running;
        //             }
        //         }
        //         _ => {}
        //     }
        // }

        // Update emulated RAM and registers
        // TODO: Implement your own emulation logic here

        // Show diagnostic information in the second window
        // Serialize the diagnostic information as a JSON string
        // TODO: Replace `your_emulator` with your own emulator instance
        // let diagnostics_data = serde_json::to_string(&your_emulator).unwrap();

        // Render the diagnostic information in the diagnostics window
        // TODO: Implement your own rendering logic here
    }

    Ok(())
}
