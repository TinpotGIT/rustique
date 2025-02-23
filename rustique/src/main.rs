use std::borrow::Cow;
use wgpu::util::DeviceExt;
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::EventLoop,
    window::Window,
    dpi::PhysicalPosition,
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

    let instance = wgpu::Instance::default();
    let surface = instance.create_surface(&window).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let swapchain_format = surface.get_capabilities(&adapter).formats[0];

    // Création du buffer de pixels
    let buffer_size = (size.width * size.height * 4) as usize;
    let mut state = State {
        mouse_pos: PhysicalPosition::new(0.0, 0.0),
        pixels: vec![255; buffer_size], // Initialisé en blanc
        buffer_dimensions: (size.width, size.height),
    };

    // Création de la texture
    let texture_size = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Pixel Buffer"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Texture Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Texture Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
    });

    event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    state.mouse_pos = position;
                }
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                } => {
                    let x = state.mouse_pos.x as u32;
                    let y = state.mouse_pos.y as u32;
                    if x < state.buffer_dimensions.0 && y < state.buffer_dimensions.1 {
                        let idx = ((y * state.buffer_dimensions.0 + x) * 4) as usize;
                        state.pixels[idx..idx + 4].copy_from_slice(&[0, 0, 0, 255]);

                        queue.write_texture(
                            wgpu::ImageCopyTexture {
                                texture: &texture,
                                mip_level: 0,
                                origin: wgpu::Origin3d { x, y, z: 0 },
                                aspect: wgpu::TextureAspect::All,
                            },
                            &[0, 0, 0, 255], // Données RGBA pour un pixel noir
                            wgpu::ImageDataLayout {
                                offset: 0,
                                bytes_per_row: Some(state.buffer_dimensions.0 * 4), // ✅ Largeur * 4 octets (RGBA)
                                rows_per_image: Some(state.buffer_dimensions.1),
                            },
                            wgpu::Extent3d {
                                width: 1,
                                height: 1,
                                depth_or_array_layers: 1,
                            },
                        );
                        
                    }
                }
                WindowEvent::CloseRequested => target.exit(),
                _ => {}
            },
            _ => {}
        }
    }).unwrap();
}
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