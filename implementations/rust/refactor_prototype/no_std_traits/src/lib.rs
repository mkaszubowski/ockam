//! Defines the traits that comprise the API for an Ockam node

#![no_std]
extern crate alloc;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::rc::Rc;
use alloc::string::String;
use core::cell::RefCell;
use core::ops::DerefMut;
use ockam_message::message::{AddressType, Message};

/// MessageHandler trait is for workers to receive and process messages.
///
/// A worker gets messages by registering its address and MessageHandler trait with the Node.
/// handle_message() will be called when its address is the next onward_address in the
/// Message route.
pub trait MessageHandler {
    fn handle_message(
        &mut self,
        message: Message,
        queue: Rc<RefCell<dyn Enqueue<Message>>>,
    ) -> Result<bool, String>;
}

/// Poll trait is for workers to get cpu cycles on a regular basis.
///
/// A worker gets polled by registering its address and Poll trait with the Node.
/// poll() will be called once each polling interval.
pub trait Poll {
    fn poll(&mut self, q_ref: Rc<RefCell<dyn Enqueue<Message>>>) -> Result<bool, String>;
}

/// Enqueue trait is used by Workers to enqueue messages for routing.
///
/// The Enqueue trait is passed to a Worker each time it is send a message or polled.
pub trait Enqueue<T> {
    fn enqueue(&mut self, t: T) -> Result<bool, String>;
}

impl<T> Enqueue<T> for VecDeque<T> {
    fn enqueue(&mut self, t: T) -> Result<bool, String> {
        self.push_back(t);
        Ok(true)
    }
}
