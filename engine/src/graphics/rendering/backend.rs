//! The `backend` module exposes the contents of the most appropriate gfx-hal
//! backend.
//!
//! Additionally, it exposes type aliases for the common types found in the
//! backend like `Surface`.

// Use DirectX 12 on Windows, Metal on MacOS, and Vulkan on Linux.
#[cfg(windows)]
pub use gfx_backend_dx12::*;
#[cfg(target_os = "macos")]
pub use gfx_backend_metal::*;
#[cfg(all(unix, not(target_os = "macos")))]
pub use gfx_backend_vulkan::*;

#[cfg(windows)]
pub const NAME: &str = "DirectX 12";
#[cfg(target_os = "macos")]
pub const NAME: &str = "Metal";
#[cfg(all(unix, not(target_os = "macos")))]
pub const NAME: &str = "Vulkan";

pub type Adapter = gfx_hal::Adapter<Backend>;
pub type ShaderEntryPoint<'a> = gfx_hal::pso::EntryPoint<'a, Backend>;
pub type Queues = gfx_hal::queue::Queues<Backend>;

pub type Surface = <Backend as gfx_hal::Backend>::Surface;
pub type Device = <Backend as gfx_hal::Backend>::Device;
pub type Swapchain = <Backend as gfx_hal::Backend>::Swapchain;
pub type Image = <Backend as gfx_hal::Backend>::Image;
pub type ImageView = <Backend as gfx_hal::Backend>::ImageView;
pub type Framebuffer = <Backend as gfx_hal::Backend>::Framebuffer;
pub type RenderPass = <Backend as gfx_hal::Backend>::RenderPass;
pub type ShaderModule = <Backend as gfx_hal::Backend>::ShaderModule;
pub type PipelineLayout = <Backend as gfx_hal::Backend>::PipelineLayout;
pub type GraphicsPipeline = <Backend as gfx_hal::Backend>::GraphicsPipeline;
pub type CommandPool = <Backend as gfx_hal::Backend>::CommandPool;
pub type CommandBuffer = <Backend as gfx_hal::Backend>::CommandBuffer;
pub type CommandQueue = <Backend as gfx_hal::Backend>::CommandQueue;
pub type Fence = <Backend as gfx_hal::Backend>::Fence;
pub type Semaphore = <Backend as gfx_hal::Backend>::Semaphore;
pub type Buffer = <Backend as gfx_hal::Backend>::Buffer;
pub type Memory = <Backend as gfx_hal::Backend>::Memory;
