use std::borrow::Cow;
use winit::{
    event::{Event, WindowEvent, MouseButton,},
    event_loop::EventLoop,
    window::Window,
    dpi::PhysicalPosition, // Add this import
    
};

struct State {
    mouse_pos: PhysicalPosition<f64>,
    pixels: Vec<u8>,
    buffer_dimensions: (u32, u32),
}

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());

    let surface = instance.create_surface(&window).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // Load the shaders from disk
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(swapchain_format.into())],
        }),

        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    let mut config = surface
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();
    surface.configure(&device, &config);

    let window = &window;
    let mut state = State {
        mouse_pos: PhysicalPosition::new(0.0, 0.0),
        pixels: vec![255; (size.width * size.height * 4) as usize],
    buffer_dimensions: (size.width, size.height),
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Pixel Buffer"),
        size: wgpu::Extent3d {
            width: size.width,
            height: size.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    event_loop
    .run(move |event, target| {
        let _ = (&instance, &adapter, &shader, &pipeline_layout);

        match event {
            Event::WindowEvent { 
                window_id,
                event,
            } => {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        state.mouse_pos = position;
                        window.set_title(&format!(
                            "Mouse Position: ({:.1}, {:.1})",
                            position.x,
                            position.y
                        ));
                        window.request_redraw();
                    }
                    WindowEvent::Resized(new_size) => {
                        size = new_size;
                        config.width = new_size.width.max(1);
                        config.height = new_size.height.max(1);
                        surface.configure(&device, &config);
                        window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        let frame = surface
                            .get_current_texture()
                            .expect("Failed to acquire next swap chain texture");
                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());
                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });
                        {
                            let mut rpass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: None,
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: &view,
                                        resolve_target: None,
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });

                            // Draw a small point at the mouse position
                            rpass.set_pipeline(&render_pipeline);
                            rpass.draw(0..0, 0..0);
                        }

                        queue.submit(Some(encoder.finish()));
                        frame.present();
                    }
                    WindowEvent::MouseInput {
                        state: winit::event::ElementState::Pressed,
                        button: MouseButton::Left,
                        ..
                    } => {
                        let x = state.mouse_pos.x as u32;
                        let y = state.mouse_pos.y as u32;
                        if x < state.buffer_dimensions.0 && y < state.buffer_dimensions.1 {
                            let idx = ((y * state.buffer_dimensions.0 + x) * 4) as usize;
                            state.pixels[idx..idx + 4].copy_from_slice(&[0, 0, 0, 255]); // Black pixel
                            
                            // Update texture with new pixel data
                            queue.write_texture(
                                wgpu::ImageCopyTexture {
                                    texture: &texture,
                                    mip_level: 0,
                                    origin: wgpu::Origin3d { x, y, z: 0 },
                                    aspect: wgpu::TextureAspect::All,
                                },
                                &[0, 0, 0, 255],
                                wgpu::ImageDataLayout {
                                    offset: 0,
                                    bytes_per_row: Some(4),
                                    rows_per_image: Some(1),
                                },
                                wgpu::Extent3d {
                                    width: 1,
                                    height: 1,
                                    depth_or_array_layers: 1,
                                },
                            );
                            window.request_redraw();
                        }
                    },
                    WindowEvent::CloseRequested => target.exit(),
                    _ => {}
                }
            }
            // Handle other event types if needed
            _ => {}
        }
    })
    .unwrap();} 

pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    #[cfg_attr(
        not(target_arch = "wasm32"),
        expect(unused_mut, reason = "`wasm32` re-assigns to specify canvas")
    )]
    let mut builder = winit::window::WindowBuilder::new();
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        builder = builder.with_canvas(Some(canvas));
    }
    let window = builder.build(&event_loop).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run(event_loop, window));
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }
}