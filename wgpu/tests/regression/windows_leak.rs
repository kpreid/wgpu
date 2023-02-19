use crate::common::{initialize_adapter, initialize_device};

use wasm_bindgen_test::wasm_bindgen_test;
use wgpu::*;

#[wasm_bindgen_test]
#[test]
fn allocate_large_3d_textures_in_devices() {
    let (adapter, _surface_guard) = initialize_adapter();

    for i in 0..100 {
        let (device, _queue) = pollster::block_on(initialize_device(
            &adapter,
            Features::empty(),
            Limits::default(),
        ));

        println!("{i}");
        // Each one of these textures occupies at least 256*256*256*4 ≈ 67MB.
        let _texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 256,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            view_formats: &[],
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });
    }
}
