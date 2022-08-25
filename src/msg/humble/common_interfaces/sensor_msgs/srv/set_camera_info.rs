// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::super::*;
use crate::msg::common_interfaces::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__srv__SetCameraInfo_Request__init(msg: *mut SetCameraInfoRequest) -> bool;
    fn sensor_msgs__srv__SetCameraInfo_Request__fini(msg: *mut SetCameraInfoRequest);
    fn sensor_msgs__srv__SetCameraInfo_Request__Sequence__init(
        msg: *mut SetCameraInfoRequestSeqRaw,
        size: usize,
    ) -> bool;
    fn sensor_msgs__srv__SetCameraInfo_Request__Sequence__fini(
        msg: *mut SetCameraInfoRequestSeqRaw,
    );
    fn sensor_msgs__srv__SetCameraInfo_Response__init(msg: *mut SetCameraInfoResponse) -> bool;
    fn sensor_msgs__srv__SetCameraInfo_Response__fini(msg: *mut SetCameraInfoResponse);
    fn sensor_msgs__srv__SetCameraInfo_Response__Sequence__init(
        msg: *mut SetCameraInfoResponseSeqRaw,
        size: usize,
    ) -> bool;
    fn sensor_msgs__srv__SetCameraInfo_Response__Sequence__fini(
        msg: *mut SetCameraInfoResponseSeqRaw,
    );
    fn rosidl_typesupport_c__get_service_type_support_handle__sensor_msgs__srv__SetCameraInfo(
    ) -> *const rcl::rosidl_service_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct SetCameraInfoRequest {
    pub camera_info: CameraInfo,
}

#[repr(C)]
#[derive(Debug)]
pub struct SetCameraInfoResponse {
    pub success: bool,
    pub status_message: crate::msg::RosString<0>,
}

impl SetCameraInfoRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__srv__SetCameraInfo_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SetCameraInfoRequest {
    fn drop(&mut self) {
        unsafe { sensor_msgs__srv__SetCameraInfo_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct SetCameraInfoRequestSeqRaw {
    data: *mut SetCameraInfoRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of SetCameraInfoRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct SetCameraInfoRequestSeq<const N: usize> {
    data: *mut SetCameraInfoRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> SetCameraInfoRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: SetCameraInfoRequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__srv__SetCameraInfo_Request__Sequence__init(&mut msg, size) } {
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
        let msg: SetCameraInfoRequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[SetCameraInfoRequest] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [SetCameraInfoRequest] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, SetCameraInfoRequest> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, SetCameraInfoRequest> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for SetCameraInfoRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = SetCameraInfoRequestSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { sensor_msgs__srv__SetCameraInfo_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for SetCameraInfoRequestSeq<N> {}
unsafe impl<const N: usize> Sync for SetCameraInfoRequestSeq<N> {}

impl SetCameraInfoResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__srv__SetCameraInfo_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SetCameraInfoResponse {
    fn drop(&mut self) {
        unsafe { sensor_msgs__srv__SetCameraInfo_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct SetCameraInfoResponseSeqRaw {
    data: *mut SetCameraInfoResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of SetCameraInfoResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct SetCameraInfoResponseSeq<const N: usize> {
    data: *mut SetCameraInfoResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> SetCameraInfoResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: SetCameraInfoResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__srv__SetCameraInfo_Response__Sequence__init(&mut msg, size) } {
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
        let msg: SetCameraInfoResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[SetCameraInfoResponse] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [SetCameraInfoResponse] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, SetCameraInfoResponse> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, SetCameraInfoResponse> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for SetCameraInfoResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = SetCameraInfoResponseSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { sensor_msgs__srv__SetCameraInfo_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for SetCameraInfoResponseSeq<N> {}
unsafe impl<const N: usize> Sync for SetCameraInfoResponseSeq<N> {}

pub struct SetCameraInfo;

impl ServiceMsg for SetCameraInfo {
    type Request = SetCameraInfoRequest;
    type Response = SetCameraInfoResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__sensor_msgs__srv__SetCameraInfo()
        }
    }
}
