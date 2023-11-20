// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__JoyFeedbackArray__init(msg: *mut JoyFeedbackArray) -> bool;
    fn sensor_msgs__msg__JoyFeedbackArray__fini(msg: *mut JoyFeedbackArray);
    fn sensor_msgs__msg__JoyFeedbackArray__are_equal(
        lhs: *const JoyFeedbackArray,
        rhs: *const JoyFeedbackArray,
    ) -> bool;
    fn sensor_msgs__msg__JoyFeedbackArray__Sequence__init(
        msg: *mut JoyFeedbackArraySeqRaw,
        size: usize,
    ) -> bool;
    fn sensor_msgs__msg__JoyFeedbackArray__Sequence__fini(msg: *mut JoyFeedbackArraySeqRaw);
    fn sensor_msgs__msg__JoyFeedbackArray__Sequence__are_equal(
        lhs: *const JoyFeedbackArraySeqRaw,
        rhs: *const JoyFeedbackArraySeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__JoyFeedbackArray(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct JoyFeedbackArray {
    pub array: JoyFeedbackSeq<0>,
}

impl JoyFeedbackArray {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__JoyFeedbackArray__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for JoyFeedbackArray {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__JoyFeedbackArray__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct JoyFeedbackArraySeqRaw {
    data: *mut JoyFeedbackArray,
    size: usize,
    capacity: usize,
}

/// Sequence of JoyFeedbackArray.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct JoyFeedbackArraySeq<const N: usize> {
    data: *mut JoyFeedbackArray,
    size: usize,
    capacity: usize,
}

impl<const N: usize> JoyFeedbackArraySeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: JoyFeedbackArraySeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__JoyFeedbackArray__Sequence__init(&mut msg, size) } {
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
        let msg: JoyFeedbackArraySeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[JoyFeedbackArray] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [JoyFeedbackArray] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, JoyFeedbackArray> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, JoyFeedbackArray> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for JoyFeedbackArraySeq<N> {
    fn drop(&mut self) {
        let mut msg = JoyFeedbackArraySeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { sensor_msgs__msg__JoyFeedbackArray__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for JoyFeedbackArraySeq<N> {}
unsafe impl<const N: usize> Sync for JoyFeedbackArraySeq<N> {}

impl TypeSupport for JoyFeedbackArray {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__JoyFeedbackArray()
        }
    }
}

impl PartialEq for JoyFeedbackArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sensor_msgs__msg__JoyFeedbackArray__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for JoyFeedbackArraySeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = JoyFeedbackArraySeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = JoyFeedbackArraySeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            sensor_msgs__msg__JoyFeedbackArray__Sequence__are_equal(&msg1, &msg2)
        }
    }
}