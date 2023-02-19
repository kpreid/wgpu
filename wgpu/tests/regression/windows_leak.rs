use crate::common::{initialize_test, TestParameters};

use wasm_bindgen_test::wasm_bindgen_test;
use wgpu::*;

#[wasm_bindgen_test]
#[test]
fn allocate_large_3d_textures_in_devices() {
    initialize_test(TestParameters::default(), |ctx| {
        for i in 0..100 {
            println!("{i}");
            // Each one of these textures occupies at least 256*256*256*4 ≈ 67MB.
            let _texture = ctx.device.create_texture(&TextureDescriptor {
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
    })
}
