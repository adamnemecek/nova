// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::future::Future;
use std::pin::Pin;
use std::task::LocalWaker;
use std::task::Poll;

pub struct YieldOnce {
  polled: bool,
}

impl Future for YieldOnce {
  type Output = ();

  fn poll(self: Pin<&mut Self>, waker: &LocalWaker) -> Poll<Self::Output> {
    if self.polled {
      return Poll::Ready(());
    }

    self.get_mut().polled = true;
    waker.wake();

    Poll::Pending
  }
}

pub fn yield_once() -> YieldOnce {
  YieldOnce { polled: false }
}
