// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn nav_msgs__msg__OccupancyGrid__init(msg: *mut OccupancyGrid) -> bool;
    fn nav_msgs__msg__OccupancyGrid__fini(msg: *mut OccupancyGrid);
    fn nav_msgs__msg__OccupancyGrid__are_equal(
        lhs: *const OccupancyGrid,
        rhs: *const OccupancyGrid,
    ) -> bool;
    fn nav_msgs__msg__OccupancyGrid__Sequence__init(
        msg: *mut OccupancyGridSeqRaw,
        size: usize,
    ) -> bool;
    fn nav_msgs__msg__OccupancyGrid__Sequence__fini(msg: *mut OccupancyGridSeqRaw);
    fn nav_msgs__msg__OccupancyGrid__Sequence__are_equal(
        lhs: *const OccupancyGridSeqRaw,
        rhs: *const OccupancyGridSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__OccupancyGrid(
    ) -> *const rcl::rosidl_message_type_support_t;
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
struct OccupancyGridSeqRaw {
    data: *mut OccupancyGrid,
    size: usize,
    capacity: usize,
}

/// Sequence of OccupancyGrid.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct OccupancyGridSeq<const N: usize> {
    data: *mut OccupancyGrid,
    size: usize,
    capacity: usize,
}

impl<const N: usize> OccupancyGridSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: OccupancyGridSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__msg__OccupancyGrid__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: OccupancyGridSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[OccupancyGrid] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [OccupancyGrid] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, OccupancyGrid> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, OccupancyGrid> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for OccupancyGridSeq<N> {
    fn drop(&mut self) {
        let mut msg = OccupancyGridSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { nav_msgs__msg__OccupancyGrid__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for OccupancyGridSeq<N> {}
unsafe impl<const N: usize> Sync for OccupancyGridSeq<N> {}

impl TypeSupport for OccupancyGrid {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__OccupancyGrid()
        }
    }
}

impl PartialEq for OccupancyGrid {
    fn eq(&self, other: &Self) -> bool {
        unsafe { nav_msgs__msg__OccupancyGrid__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for OccupancyGridSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = OccupancyGridSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = OccupancyGridSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            nav_msgs__msg__OccupancyGrid__Sequence__are_equal(&msg1, &msg2)
        }
    }
}