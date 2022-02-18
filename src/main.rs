fn main() -> anyhow::Result<()> {
    use pollster::FutureExt;

    // create device, queue
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .block_on()
        .ok_or(anyhow::anyhow!("Couldn't create the adapter"))?;
    let (device, queue) = adapter
        .request_device(&Default::default(), None)
        .block_on()?;

    // load image
    let input_image = image::load_from_memory(include_bytes!("sushi.png"))?.to_rgba8();
    let (width, height) = input_image.dimensions();

    let texture_size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let input_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("input texture"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    });

    queue.write_texture(
        input_texture.as_image_copy(),
        bytemuck::cast_slice(input_image.as_raw()),
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: std::num::NonZeroU32::new(4 * width),
            rows_per_image: None, // doesn't need to be specified as we're writing a single image
        },
        texture_size,
    );

    let output_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("output texture"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::STORAGE_BINDING,
    });

    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Grayscale shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shaders/grayscale.wgsl").into()),
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Grayscale pipeline"),
        layout: None,
        module: &shader,
        entry_point: "grayscale_main",
    });
    
}
