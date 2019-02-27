// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Message;
use crossbeam::queue::SegQueue;

#[derive(Debug, Default)]
pub struct MessageQueue {
    inner: SegQueue<Message>,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send(&self, msg: Message) {
        self.inner.push(msg);
    }

    pub(crate) fn take(&self) -> Option<Message> {
        self.inner.pop().ok()
    }
}
