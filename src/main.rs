use egui::Checkbox;
use egui_backend::sdl2::video::GLProfile;
use egui_backend::{egui, gl, sdl2};
use egui_backend::{sdl2::event::Event, DpiScaling, ShaderVersion};
use std::time::Instant;
// Alias the backend to something less mouthful
use egui_sdl2_gl as egui_backend;
use sdl2::video::SwapInterval;

use rust_boy::cartridge::Cartridge;
use rust_boy::cpu::Cpu;
// use rust_boy::header::Header;
use rust_boy::memorymap::MemoryMap;

fn main() {
    // LOAD CARTRIDGE
    let rom_path = "roms/cpu_instrs.gb";
    let cartridge = Cartridge::load(rom_path);
    let mut memmap = MemoryMap::default();
    memmap.load_cartridge(&cartridge);
    let mut cpu = Cpu::load(&mut memmap);

    // SETUP SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    // For OpenGL ES Mesa driver 22.0.0+
    // gl_attr.set_context_profile(GLProfile::GLES);

    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);

    let window = video_subsystem
        .window("RustBoy", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _ctx = window.gl_create_context().unwrap();

    // let shader_ver = ShaderVersion::Default;
    // On linux use GLES SL 100+, like so:
    let shader_ver = ShaderVersion::Adaptive;
    let (mut painter, mut egui_state) =
        egui_backend::with_sdl2(&window, shader_ver, DpiScaling::Custom(2.0));
    let mut egui_ctx = egui::CtxRef::default();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut enable_vsync = false;
    let mut quit = false;
    let mut slider = 0.0;

    let _iters = 10000;
    let mut i = 100;

    let start_time = Instant::now();
    // GAME LOOP
    'running: loop {
        let mut cpu_data = cpu.get_cpu_data();

        if enable_vsync {
            window
                .subsystem()
                .gl_set_swap_interval(SwapInterval::VSync)
                .unwrap()
        } else {
            window
                .subsystem()
                .gl_set_swap_interval(SwapInterval::Immediate)
                .unwrap()
        }

        egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_state.input.take());

        egui::CentralPanel::default().show(&egui_ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("AF: ");
                ui.label(format!("{:#6X}", cpu_data.af));
            });
            ui.horizontal(|ui| {
                ui.label("BC: ");
                ui.label(format!("{:#6X}", cpu_data.bc));
            });
            ui.horizontal(|ui| {
                ui.label("DE: ");
                ui.label(format!("{:#6X}", cpu_data.de));
            });
            ui.horizontal(|ui| {
                ui.label("HL: ");
                ui.label(format!("{:#6X}", cpu_data.hl));
            });
            ui.horizontal(|ui| {
                ui.label("SP: ");
                ui.label(format!("{:#6X}", cpu_data.sp));
            });
            ui.horizontal(|ui| {
                ui.label("PC: ");
                ui.label(format!("{:#6X}", cpu_data.pc));
            });
            ui.horizontal(|ui| {
                ui.label("Z: ");
                ui.add(Checkbox::new(&mut cpu_data.z, ""));
                ui.label("N: ");
                ui.add(Checkbox::new(&mut cpu_data.n, ""));
                ui.label("H: ");
                ui.add(Checkbox::new(&mut cpu_data.h, ""));
                ui.label("C: ");
                ui.add(Checkbox::new(&mut cpu_data.c, ""));
            });

            if ui.button("STEP").clicked() {
                // cpu.step();
                i = 0;
            }
            ui.separator();
            ui.add(egui::Slider::new(&mut slider, 0.0..=50.0).text("Slider"));
            ui.add(Checkbox::new(&mut enable_vsync, "Enable vsync?"));
            ui.separator();
            if ui.button("Quit?").clicked() {
                quit = true;
            }
        });

        cpu.step();

        let (egui_output, paint_cmds) = egui_ctx.end_frame();
        // Process ouput
        egui_state.process_output(&window, &egui_output);
        let paint_jobs = egui_ctx.tessellate(paint_cmds);

        unsafe {
            // Clear the screen to green
            gl::ClearColor(0.3, 0.6, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        painter.paint_jobs(None, paint_jobs, &egui_ctx.font_image());
        window.gl_swap_window();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {
                    // Process input event
                    egui_state.process_input(&window, event, &mut painter);
                }
            }
        }

        if quit {
            break;
        }
    }

    /* let header = Header::new(&cartridge.data);
    if !header.is_compatible() {
        println!(
            "{:?} is not supported by rust_boy yet.",
            header.cartridge_type
        );
        std::process::exit(0);
    } */

    // println!("{:?}", memmap.memory)

    // let tile_test = Tile::new(header.logo[0..16].to_vec());
    // let tile_map_1 = cartridge.data[0x8000..0x87FF + 1].to_vec();
    // let tile_sz = 16;
    // let tile_id_offset = 128;

    /* (0..40).for_each(|sprite| {
        // The OAM (Object Attribute Map) is empty until runtime
        let y_pos = cartridge.data[0xFE00 + (sprite * 4) + 0] as usize;
        let x_pos = cartridge.data[0xFE00 + (sprite * 4) + 1] as usize;
        let tile_index = cartridge.data[0xFE00 + (sprite * 4) + 2] as usize;
        let attrs = cartridge.data[0xFE00 + (sprite * 4) + 3] as usize;
        println!("y_pos: {}", y_pos);
        println!("x_pos: {}", x_pos);
        println!("tile_index: {:?}", tile_index);
        println!("attrs: {}", attrs);

        let tile_addr = mem_region_1 + (tile_index * tile_sz);
        println!("{:X?}", tile_addr);
        println!("{:X?}", tile_addr + 15);
        println!(
            "{}",
            Tile::new(cartridge.data[tile_addr..tile_addr + 16].to_vec())
        );
    }); */
}
