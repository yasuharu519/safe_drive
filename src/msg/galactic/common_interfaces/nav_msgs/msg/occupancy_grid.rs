// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn nav_msgs__msg__OccupancyGrid__init(msg: *mut OccupancyGrid) -> bool;
    fn nav_msgs__msg__OccupancyGrid__fini(msg: *mut OccupancyGrid);
    fn nav_msgs__msg__OccupancyGrid__Sequence__init(msg: *mut OccupancyGridSequence, size: usize) -> bool;
    fn nav_msgs__msg__OccupancyGrid__Sequence__fini(msg: *mut OccupancyGridSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__OccupancyGrid() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct OccupancyGrid {
    pub header: std_msgs::msg::Header,
    pub info: MapMetaData,
    pub data: crate::msg::I8Seq<0>,
}

impl OccupancyGrid {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__msg__OccupancyGrid__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for OccupancyGrid {
    fn drop(&mut self) {
        unsafe { nav_msgs__msg__OccupancyGrid__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct OccupancyGridSequence {
    data: *mut OccupancyGrid,
    size: usize,
    capacity: usize,
}

impl OccupancyGridSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__msg__OccupancyGrid__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[OccupancyGrid]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [OccupancyGrid]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for OccupancyGridSequence {
    fn drop(&mut self) {
        unsafe { nav_msgs__msg__OccupancyGrid__Sequence__fini(self) };
    }
}

impl TopicMsg for OccupancyGrid {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__OccupancyGrid()
        }
    }
}