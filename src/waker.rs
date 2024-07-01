//! This mod specific the waker related with coroutine
//!

use crate::task::TaskInner;
use core::task::{RawWaker, RawWakerVTable, Waker};
extern crate alloc;
use alloc::boxed::Box;

const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake, drop);

unsafe fn clone(p: *const ()) -> RawWaker {
    RawWaker::new(p, &VTABLE)
}

/// nop
unsafe fn wake(p: *const ()) { 
    let task_inner = p as *const TaskInner as *mut TaskInner;
    Box::from_raw((*task_inner).get_wake_fn().unwrap())();
}

unsafe fn drop(_p: *const ()) {
    // nop
}

/// 
pub unsafe fn from_task(task_ref: *const TaskInner) -> Waker {
    Waker::from_raw(RawWaker::new(task_ref as _, &VTABLE))
}