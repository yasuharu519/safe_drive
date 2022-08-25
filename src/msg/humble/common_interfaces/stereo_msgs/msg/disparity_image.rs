// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn stereo_msgs__msg__DisparityImage__init(msg: *mut DisparityImage) -> bool;
    fn stereo_msgs__msg__DisparityImage__fini(msg: *mut DisparityImage);
    fn stereo_msgs__msg__DisparityImage__are_equal(
        lhs: *const DisparityImage,
        rhs: *const DisparityImage,
    ) -> bool;
    fn stereo_msgs__msg__DisparityImage__Sequence__init(
        msg: *mut DisparityImageSeqRaw,
        size: usize,
    ) -> bool;
    fn stereo_msgs__msg__DisparityImage__Sequence__fini(msg: *mut DisparityImageSeqRaw);
    fn stereo_msgs__msg__DisparityImage__Sequence__are_equal(
        lhs: *const DisparityImageSeqRaw,
        rhs: *const DisparityImageSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__stereo_msgs__msg__DisparityImage(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct DisparityImage {
    pub header: std_msgs::msg::Header,
    pub image: sensor_msgs::msg::Image,
    pub f: f32,
    pub t: f32,
    pub valid_window: sensor_msgs::msg::RegionOfInterest,
    pub min_disparity: f32,
    pub max_disparity: f32,
    pub delta_d: f32,
}

impl DisparityImage {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { stereo_msgs__msg__DisparityImage__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for DisparityImage {
    fn drop(&mut self) {
        unsafe { stereo_msgs__msg__DisparityImage__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct DisparityImageSeqRaw {
    data: *mut DisparityImage,
    size: usize,
    capacity: usize,
}

/// Sequence of DisparityImage.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct DisparityImageSeq<const N: usize> {
    data: *mut DisparityImage,
    size: usize,
    capacity: usize,
}

impl<const N: usize> DisparityImageSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: DisparityImageSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { stereo_msgs__msg__DisparityImage__Sequence__init(&mut msg, size) } {
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
        let msg: DisparityImageSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[DisparityImage] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [DisparityImage] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, DisparityImage> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, DisparityImage> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for DisparityImageSeq<N> {
    fn drop(&mut self) {
        let mut msg = DisparityImageSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { stereo_msgs__msg__DisparityImage__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for DisparityImageSeq<N> {}
unsafe impl<const N: usize> Sync for DisparityImageSeq<N> {}

impl TopicMsg for DisparityImage {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__stereo_msgs__msg__DisparityImage(
            )
        }
    }
}

impl PartialEq for DisparityImage {
    fn eq(&self, other: &Self) -> bool {
        unsafe { stereo_msgs__msg__DisparityImage__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for DisparityImageSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = DisparityImageSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = DisparityImageSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            stereo_msgs__msg__DisparityImage__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
