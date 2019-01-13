// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{LocalWaker, Poll, Waker};

pub struct CompletionSource<R> {
  inner: Arc<Mutex<Inner<R>>>,
}

impl<R> CompletionSource<R> {
  pub fn new() -> Self {
    CompletionSource {
      inner: Arc::new(Mutex::new(Inner {
        waker: None,
        result: None,
      })),
    }
  }

  pub fn as_future(&self) -> Completion<R> {
    Completion {
      source: CompletionSource {
        inner: self.inner.clone(),
      },
    }
  }

  pub fn complete(&self, result: R) {
    let mut inner = self.inner.lock().unwrap();

    inner.result = Some(result);

    if let Some(ref waker) = inner.waker {
      waker.wake();
    }
  }
}

impl<R> Default for CompletionSource<R> {
  fn default() -> Self {
    Self::new()
  }
}

pub struct Completion<R> {
  source: CompletionSource<R>,
}

struct Inner<R> {
  waker: Option<Waker>,
  result: Option<R>,
}

impl<R> Future for Completion<R> {
  type Output = R;

  fn poll(self: Pin<&mut Self>, waker: &LocalWaker) -> Poll<R> {
    let mut source = self.source.inner.lock().unwrap();

    if let Some(result) = source.result.take() {
      return Poll::Ready(result);
    }

    source.waker = Some(waker.as_waker().clone());

    Poll::Pending
  }
}
