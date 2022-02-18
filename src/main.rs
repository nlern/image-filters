fn main() -> anyhow::Result<()> {
    use pollster::FutureExt;

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

    
}
