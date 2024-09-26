use std::sync::Arc;

use wgpu::RequestAdapterOptions;
use wgpu_dummy::*;

fn main() {
    println!("Hello, world!");
    let instance = wgpu::Instance::from_any(Arc::new(DummyInstance()));
    let _ = instance.request_adapter(&RequestAdapterOptions::default());
}
