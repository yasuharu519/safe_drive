// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::super::*;
use crate::msg::common_interfaces::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn composition_interfaces__srv__ListNodes_Request__init(msg: *mut ListNodesRequest) -> bool;
    fn composition_interfaces__srv__ListNodes_Request__fini(msg: *mut ListNodesRequest);
    fn composition_interfaces__srv__ListNodes_Request__Sequence__init(
        msg: *mut ListNodesRequestSeqRaw,
        size: usize,
    ) -> bool;
    fn composition_interfaces__srv__ListNodes_Request__Sequence__fini(
        msg: *mut ListNodesRequestSeqRaw,
    );
    fn composition_interfaces__srv__ListNodes_Response__init(msg: *mut ListNodesResponse) -> bool;
    fn composition_interfaces__srv__ListNodes_Response__fini(msg: *mut ListNodesResponse);
    fn composition_interfaces__srv__ListNodes_Response__Sequence__init(
        msg: *mut ListNodesResponseSeqRaw,
        size: usize,
    ) -> bool;
    fn composition_interfaces__srv__ListNodes_Response__Sequence__fini(
        msg: *mut ListNodesResponseSeqRaw,
    );
    fn rosidl_typesupport_c__get_service_type_support_handle__composition_interfaces__srv__ListNodes(
    ) -> *const rcl::rosidl_service_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct ListNodesRequest {
    _unused: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct ListNodesResponse {
    pub full_node_names: crate::msg::RosStringSeq<0, 0>,
    pub unique_ids: crate::msg::U64Seq<0>,
}

impl ListNodesRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { composition_interfaces__srv__ListNodes_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for ListNodesRequest {
    fn drop(&mut self) {
        unsafe { composition_interfaces__srv__ListNodes_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct ListNodesRequestSeqRaw {
    data: *mut ListNodesRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of ListNodesRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct ListNodesRequestSeq<const N: usize> {
    data: *mut ListNodesRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> ListNodesRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: ListNodesRequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { composition_interfaces__srv__ListNodes_Request__Sequence__init(&mut msg, size) }
        {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[ListNodesRequest] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [ListNodesRequest] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, ListNodesRequest> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, ListNodesRequest> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for ListNodesRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = ListNodesRequestSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { composition_interfaces__srv__ListNodes_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for ListNodesRequestSeq<N> {}
unsafe impl<const N: usize> Sync for ListNodesRequestSeq<N> {}

impl ListNodesResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { composition_interfaces__srv__ListNodes_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for ListNodesResponse {
    fn drop(&mut self) {
        unsafe { composition_interfaces__srv__ListNodes_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct ListNodesResponseSeqRaw {
    data: *mut ListNodesResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of ListNodesResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct ListNodesResponseSeq<const N: usize> {
    data: *mut ListNodesResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> ListNodesResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: ListNodesResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe {
            composition_interfaces__srv__ListNodes_Response__Sequence__init(&mut msg, size)
        } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[ListNodesResponse] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [ListNodesResponse] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, ListNodesResponse> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, ListNodesResponse> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for ListNodesResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = ListNodesResponseSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { composition_interfaces__srv__ListNodes_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for ListNodesResponseSeq<N> {}
unsafe impl<const N: usize> Sync for ListNodesResponseSeq<N> {}

pub struct ListNodes;

impl ServiceMsg for ListNodes {
    type Request = ListNodesRequest;
    type Response = ListNodesResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__composition_interfaces__srv__ListNodes()
        }
    }
}