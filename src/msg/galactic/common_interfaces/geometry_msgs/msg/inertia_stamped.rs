// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__InertiaStamped__init(msg: *mut InertiaStamped) -> bool;
    fn geometry_msgs__msg__InertiaStamped__fini(msg: *mut InertiaStamped);
    fn geometry_msgs__msg__InertiaStamped__Sequence__init(msg: *mut InertiaStampedSequence, size: usize) -> bool;
    fn geometry_msgs__msg__InertiaStamped__Sequence__fini(msg: *mut InertiaStampedSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__InertiaStamped() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct InertiaStamped {
    pub header: std_msgs::msg::Header,
    pub inertia: Inertia,
}

impl InertiaStamped {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__InertiaStamped__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for InertiaStamped {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__InertiaStamped__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct InertiaStampedSequence {
    data: *mut InertiaStamped,
    size: usize,
    capacity: usize,
}

impl InertiaStampedSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__InertiaStamped__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[InertiaStamped]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [InertiaStamped]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for InertiaStampedSequence {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__InertiaStamped__Sequence__fini(self) };
    }
}

impl TopicMsg for InertiaStamped {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__InertiaStamped()
        }
    }
}