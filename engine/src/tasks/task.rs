// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::AtomicWake;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{local_waker_from_nonlocal, LocalWaker, Poll};

pub struct Task<F> {
  future: Pin<Box<F>>,
  wake: Arc<AtomicWake>,
  local_waker: LocalWaker,
}

impl<F: Future + 'static> Task<F> {
  pub fn new(future: F) -> Self {
    let wake = Arc::new(AtomicWake::new());

    Task {
      future: Box::pin(future),
      wake: wake.clone(),
      local_waker: local_waker_from_nonlocal(wake),
    }
  }

  pub fn poll(&mut self) -> Poll<F::Output> {
    if !self.wake.is_awake() {
      return Poll::Pending;
    }

    self.future.as_mut().poll(&self.local_waker)
  }
}
