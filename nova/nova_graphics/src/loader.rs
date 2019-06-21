// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

#[derive(Clone)]
pub struct Loader {
  messages: channel::Sender<Message>,
}

impl Loader {
  const STAGING_BUFFER_LEN: usize = 64 * 1024 * 1024;

  pub fn new(context: &Context) -> Loader {
    let context = context.clone();
    let (messages_send, messages) = channel::unbounded();

    let queue_id = context.queues().find_transfer_queue();
    let command_pool = cmd::Pool::new(&context, queue_id).expect("failed to create command pool");
    let mut submission = cmd::Submission::new(queue_id);
    let fence = Fence::new(&context, false).expect("failed to create fence");

    let mut staging_buffer = Buffer::new(
      &context,
      BufferKind::Staging,
      Self::STAGING_BUFFER_LEN as u64,
    )
    .expect("failed to create staging buffer");

    thread::spawn(move || {
      while let Ok(message) = messages.recv() {
        match message {
          Message::LoadBuffer { src, kind, result } => {
            let src = src.as_bytes();

            let dest = match Buffer::new(&context, kind, src.len() as u64) {
              Ok(b) => b,

              err => {
                let _ = result.send(err);

                continue;
              }
            };

            unsafe {
              std::ptr::copy_nonoverlapping(
                &src[0] as *const u8,
                &mut staging_buffer.as_mut()[0] as *mut u8,
                src.len(),
              );
            }

            let mut cmd_list = cmd::List::new(&command_pool);
            let mut cmd = cmd_list.begin();

            cmd.copy_buffer(&staging_buffer, 0..dest.len(), &dest, 0);
            cmd.finish();

            submission.command_buffers.push(cmd_list);

            context.queues().submit(&submission, &fence);
            fence.wait_and_reset();

            submission.clear();

            let _ = result.send(Ok(dest));
          }
        }
      }
    });

    Loader {
      messages: messages_send,
    }
  }

  pub fn load_buffer<T: Copy + Send + 'static>(
    &self,
    kind: BufferKind,
    data: impl Into<Vec<T>>,
  ) -> channel::Receiver<Result<Buffer, BufferCreationError>> {
    let (result, result_recv) = channel::bounded(0);
    let src = Box::new(DynamicSrc(data.into()));

    debug_assert!(src.as_bytes().len() < Self::STAGING_BUFFER_LEN);

    let _ = self
      .messages
      .send(Message::LoadBuffer { src, kind, result });

    result_recv
  }
}

enum Message {
  LoadBuffer {
    src: Box<AsBytes + Send>,
    kind: BufferKind,
    result: channel::Sender<Result<Buffer, BufferCreationError>>,
  },
}

trait AsBytes {
  fn as_bytes(&self) -> &[u8];
}

struct DynamicSrc<T>(Vec<T>);

impl<T: Copy + Send> AsBytes for DynamicSrc<T> {
  fn as_bytes(&self) -> &[u8] {
    unsafe {
      slice::from_raw_parts(
        &self.0[0] as *const T as *const u8,
        self.0.len() * mem::size_of::<T>(),
      )
    }
  }
}
