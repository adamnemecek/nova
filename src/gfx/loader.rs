// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

const STAGING_BUFFER_LEN: u64 = 64 * 1024 * 1024;

/// Cloneable handle to a background thread for loading the data for large
/// buffers and images onto the graphics device.
#[derive(Clone)]
pub struct Loader {
  messages: channel::Sender<Message>,
}

struct LoaderState {
  cmd_pool: cmd::Pool,
  queue_id: cmd::QueueId,
  fence: cmd::Fence,
  staging_buffer: Buffer,
}

impl LoaderState {
  fn new(context: &Arc<Context>) -> Result<Self, LoaderCreationError> {
    let queue_id = context.queues().find_transfer_queue();
    let cmd_pool = cmd::Pool::new(&context, queue_id)?;
    let fence = cmd::Fence::new(&context, false)?;

    let mut staging_buffer = Buffer::new(&context, BufferKind::Staging, STAGING_BUFFER_LEN)
      .map_err(LoaderCreationError::StagingBufferCreationFailed)?;

    Ok(Self { cmd_pool, queue_id, fence, staging_buffer })
  }
}

impl Loader {
  /// Starts a background thread to load the data for large buffers and images
  /// onto the device, returning a cloneable control handle.
  pub fn new(
    thread_scope: &thread::Scope,
    context: &Arc<Context>,
  ) -> Result<Loader, LoaderCreationError> {
    let context = context.clone();
    let (send_result, recv_result) = channel::bounded(0);

    thread_scope.spawn(move |thread_scope| {
      // Initialize the loader state.
      let mut state = match LoaderState::new(&context) {
        Ok(state) => state,
        Err(err) => {
          let _ = send_result.send(Err(err));

          return;
        }
      };

      // Create a channel for control messages and send it back to the main
      // thread wrapped in `Loader`.
      let (send_messages, recv_messages) = channel::unbounded();

      let loader = Loader { messages: send_messages };

      if send_result.send(Ok(loader)).is_err() {
        return;
      }

      // Process incoming messages.
      while let Ok(message) = recv_messages.recv() {
        match message {
          Message::LoadBuffer { src, kind, result } => {
            let src = (*src).as_ref();

            // Create the buffer to load data into.
            let dest = match Buffer::new(&context, kind, src.len() as u64) {
              Ok(b) => b,

              Err(err) => {
                let _ = result.send(Err(err.into()));

                continue;
              }
            };

            // Copy the src data into the staging buffer.
            unsafe {
              std::ptr::copy_nonoverlapping(
                &src[0] as *const u8,
                &mut state.staging_buffer.as_mut()[0] as *mut u8,
                src.len(),
              );
            }

            // Record a command list to copy data from the staging buffer into
            // the destination buffer.
            let mut cmd_list = cmd::List::new(&state.cmd_pool);
            let mut cmd = cmd_list.begin();

            cmd.copy_buffer(&state.staging_buffer, 0..dest.len(), &dest, 0);
            cmd.end();

            // Submit the transfer commands and wait for them to complete.
            state.submission.command_buffers.push(cmd_list);

            context.queues().submit(&state.submission, &state.fence);
            state.fence.wait_and_reset();

            state.submission.clear();

            // Send the filled buffer as a result.
            let _ = result.send(Ok(dest));
          }

          Message::LoadImage { src, size, result } => {
            let src = (*src).as_ref();

            // Create the image to load data into.
            let dest = match Image::new(&context, size) {
              Ok(image) => image,

              Err(err) => {
                let _ = result.send(Err(err.into()));

                continue;
              }
            };

            // Copy the src data into the staging buffer.
            unsafe {
              std::ptr::copy_nonoverlapping(
                &src[0] as *const u8,
                &mut state.staging_buffer.as_mut()[0] as *mut u8,
                src.len(),
              );
            }

            // Record a command list to copy data from the staging buffer into
            // the image.
            let mut cmd_list = cmd::List::new(&state.cmd_pool);
            let mut cmd = cmd_list.begin();

            // Record a command to change the layout of the image for optimal
            // transfer.
            cmd.pipeline_barrier(
              pipeline::Stage::BOTTOM_OF_PIPE..pipeline::Stage::TRANSFER,
              &[cmd::image_barrier(
                &dest,
                cmd::ImageAccess::empty()..cmd::ImageAccess::TRANSFER_WRITE,
                cmd::ImageLayout::Undefined..cmd::ImageLayout::TransferDstOptimal,
              )],
            );

            // Record the copy command.
            cmd.copy_buffer_to_image(
              &state.staging_buffer,
              0,
              &dest,
              cmd::ImageLayout::TransferDstOptimal,
              Rect { start: Point2::origin(), end: Point2::new(size.width, size.height) },
            );

            // Record a command to change the layout of the image for optimal
            // shader reads.
            cmd.pipeline_barrier(
              pipeline::Stage::TRANSFER..pipeline::Stage::TOP_OF_PIPE,
              &[cmd::image_barrier(
                &dest,
                cmd::ImageAccess::TRANSFER_WRITE..cmd::ImageAccess::empty(),
                cmd::ImageLayout::TransferDstOptimal..cmd::ImageLayout::ShaderReadOnlyOptimal,
              )],
            );

            cmd.end();

            // Submit the transfer commands and wait for them to complete.
            state.submission.command_buffers.push(cmd_list);

            context.queues().submit(&state.submission, &state.fence);
            state.fence.wait_and_reset();

            state.submission.clear();

            // Send the filled buffer as a result.
            let _ = result.send(Ok(dest));
          }
        }
      }
    });

    recv_result.recv()?
  }

  /// Asynchronously loads a buffer from the given source data.
  pub fn load_buffer<T: Copy + Send + 'static>(
    &self,
    kind: BufferKind,
    src: impl Into<Vec<T>>,
  ) -> LoaderResult<Buffer, LoadBufferError> {
    let (result, result_recv) = channel::bounded(0);
    let src = DynamicSrc(src.into());

    debug_assert!((src.as_ref().len() as u64) < STAGING_BUFFER_LEN);

    let _ = self.messages.send(Message::LoadBuffer { src: Box::new(src), kind, result });

    LoaderResult { receiver: result_recv }
  }

  /// Asynchronously loads an image from the given source data.
  pub fn load_image(
    &self,
    size: Size<u32>,
    src: impl Into<Vec<u8>>,
  ) -> LoaderResult<Image, LoadImageError> {
    let (result, result_recv) = channel::bounded(0);
    let src = src.into();

    debug_assert!((src.len() as u64) < STAGING_BUFFER_LEN);

    let _ = self.messages.send(Message::LoadImage { src: Box::new(src), size, result });

    LoaderResult { receiver: result_recv }
  }
}

/// Receiver for the result of an asynchrounous load by a `Loader` background
/// thread.
pub struct LoaderResult<T, E> {
  receiver: channel::Receiver<Result<T, E>>,
}

impl<T, E: From<channel::RecvError>> LoaderResult<T, E> {
  /// Returns the result, blocking until it is available.
  pub fn recv(&self) -> Result<T, E> {
    self.receiver.recv().map_err(E::from).and_then(|res| res)
  }
}

impl<T, E: From<channel::TryRecvError>> LoaderResult<T, E> {
  /// Attempts to receive the result without blocking.
  pub fn try_recv(&self) -> Result<T, E> {
    self.receiver.try_recv().map_err(E::from).and_then(|res| res)
  }
}

/// Control message sent to a `Loader` background thread.
enum Message {
  LoadBuffer {
    src: Box<AsRef<[u8]> + Send>,
    kind: BufferKind,
    result: channel::Sender<Result<Buffer, LoadBufferError>>,
  },
  LoadImage {
    src: Box<AsRef<[u8]> + Send>,
    size: Size<u32>,
    result: channel::Sender<Result<Image, LoadImageError>>,
  },
}

/// Wrapper around a generic `Vec<T>` for implementing `AsBytes`.
struct DynamicSrc<T>(Vec<T>);

impl<T: Copy + Send> AsRef<[u8]> for DynamicSrc<T> {
  fn as_ref(&self) -> &[u8] {
    unsafe {
      slice::from_raw_parts(&self.0[0] as *const T as *const u8, self.0.len() * mem::size_of::<T>())
    }
  }
}

/// An error that occurred during the creation of a `Loader`.
#[derive(Debug)]
pub enum LoaderCreationError {
  /// Out of either host or device memory.
  OutOfMemory,
  /// An error occurred during the creation of a staging buffer.
  StagingBufferCreationFailed(BufferCreationError),
  /// An unknown error occurred.
  Unknown,
}

impl std::error::Error for LoaderCreationError {}

impl fmt::Display for LoaderCreationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      LoaderCreationError::OutOfMemory => write!(f, "out of memory"),
      LoaderCreationError::StagingBufferCreationFailed(err) => {
        write!(f, "failed to create staging buffer: {}", err)
      }
      LoaderCreationError::Unknown => write!(f, "unknown error"),
    }
  }
}

impl From<channel::RecvError> for LoaderCreationError {
  fn from(_: channel::RecvError) -> Self {
    LoaderCreationError::Unknown
  }
}

impl From<OutOfMemoryError> for LoaderCreationError {
  fn from(_: OutOfMemoryError) -> Self {
    LoaderCreationError::OutOfMemory
  }
}

/// An error that occurred while loading a buffer on a `Loader` background
/// thread.
#[derive(Debug)]
pub enum LoadBufferError {
  /// The `Loader` background thread has been shut down.
  LoaderShutDown,
  /// An error occurred during the creation of the buffer.
  CreationFailed(BufferCreationError),
}

impl std::error::Error for LoadBufferError {}

impl fmt::Display for LoadBufferError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      LoadBufferError::LoaderShutDown => write!(f, "background loader has shut down"),
      LoadBufferError::CreationFailed(err) => write!(f, "failed to create buffer: {}", err),
    }
  }
}

impl From<BufferCreationError> for LoadBufferError {
  fn from(err: BufferCreationError) -> Self {
    LoadBufferError::CreationFailed(err)
  }
}

impl From<channel::RecvError> for LoadBufferError {
  fn from(_: channel::RecvError) -> Self {
    LoadBufferError::LoaderShutDown
  }
}

/// An error that occurred while loading an [`Image`] on a [`Loader`] background
/// thread.
#[derive(Debug)]
pub enum LoadImageError {
  /// The `Loader` has been shut down.
  LoaderShutDown,
  /// An error occurred while creating the [`Image`].
  CreationFailed(ImageCreationError),
}

impl std::error::Error for LoadImageError {}

impl fmt::Display for LoadImageError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      LoadImageError::LoaderShutDown => write!(f, "background loader has shut down"),
      LoadImageError::CreationFailed(err) => write!(f, "failed to create image: {}", err),
    }
  }
}

impl From<channel::RecvError> for LoadImageError {
  fn from(_: channel::RecvError) -> Self {
    LoadImageError::LoaderShutDown
  }
}

impl From<ImageCreationError> for LoadImageError {
  fn from(err: ImageCreationError) -> Self {
    LoadImageError::CreationFailed(err)
  }
}
