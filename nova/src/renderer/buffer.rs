use super::device::{Device, DeviceExt, Gpu, PhysicalDeviceExt};
use super::Backend;
use std::mem;

pub use gfx_hal::buffer::Usage as BufferUsage;

type RawBuffer = <Backend as gfx_hal::Backend>::Buffer;
type RawMemory = <Backend as gfx_hal::Backend>::Memory;

pub struct Buffer<T> {
  pub(crate) raw: RawBuffer,
  pub(crate) raw_memory: RawMemory,
  size: u64,
  _marker: std::marker::PhantomData<T>,
}

impl<T: Copy> Buffer<T> {
  pub fn new(gpu: &Gpu, size: usize, usage: BufferUsage) -> Self {
    let size = mem::size_of::<T>() as u64 * size as u64;

    let mut raw = unsafe {
      gpu
        .device()
        .create_buffer(size, usage)
        .expect("could not create buffer")
    };

    let requirements = unsafe { gpu.device().get_buffer_requirements(&raw) };

    let upload_type = gpu
      .physical_device()
      .memory_properties()
      .memory_types
      .iter()
      .enumerate()
      .find(|(id, ty)| {
        let supported = requirements.type_mask & (1_u64 << id) != 0;

        supported
          && ty
            .properties
            .contains(gfx_hal::memory::Properties::CPU_VISIBLE)
      })
      .map(|(id, _ty)| gfx_hal::MemoryTypeId(id))
      .expect("could not find approprate buffer memory type");

    let raw_memory = unsafe {
      gpu
        .device()
        .allocate_memory(upload_type, requirements.size)
        .expect("could not allocate memory")
    };

    unsafe {
      gpu
        .device()
        .bind_buffer_memory(&raw_memory, 0, &mut raw)
        .expect("could not bind buffer memory");
    }

    Buffer {
      raw: raw.into(),
      raw_memory: raw_memory.into(),
      size,
      _marker: std::marker::PhantomData,
    }
  }

  pub fn write(&mut self, device: &Device, values: &[T]) {
    let mut dest = unsafe {
      device
        .acquire_mapping_writer::<T>(&self.raw_memory, 0..self.size)
        .expect("Could not acquire mapping writer")
    };

    dest.copy_from_slice(values);

    unsafe {
      device
        .release_mapping_writer(dest)
        .expect("Could not release mapping writer");
    }
  }

  fn destroy(self, device: &Device) {
    unsafe {
      device.free_memory(self.raw_memory);
      device.destroy_buffer(self.raw);
    }
  }
}
