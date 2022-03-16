use std::marker::PhantomData;
use std::thread;

use crate::sys;
use crate::Ui;

/// Used to render only the visible items when displaying a
/// long list of items in a scrollable area.
///
/// For example, you can have a huge list of checkboxes.
/// Without the clipper you have to call `ui.checkbox(...)`
/// for every one, even if 99% of of them are not visible in
/// the current frame. Using the `ListClipper`, you can only
/// call `ui.checkbox(...)` for the currently visible items.
///
/// Note the efficiency of list clipper relies on the height
/// of each item being cheaply calculated. The current rust
/// bindings only works with a fixed height for all items.
pub struct ListClipper {
    items_count: i32,
    items_height: f32,
}

impl ListClipper {
    pub const fn new(items_count: i32) -> Self {
        ListClipper {
            items_count,
            items_height: -1.0,
        }
    }

    /// Manually set item height. If not set, the height of the first item is used for all subsequent rows.
    pub const fn items_height(mut self, items_height: f32) -> Self {
        self.items_height = items_height;
        self
    }

    pub fn begin(self, ui: &Ui) -> ListClipperToken<'_> {
        let list_clipper = unsafe {
            let list_clipper = sys::ImGuiListClipper_ImGuiListClipper();
            sys::ImGuiListClipper_Begin(list_clipper, self.items_count, self.items_height);
            list_clipper
        };
        ListClipperToken::new(ui, list_clipper)
    }
}

pub struct ListClipperToken<'ui> {
    list_clipper: *mut sys::ImGuiListClipper,
    _phantom: PhantomData<&'ui Ui>,
}

impl<'ui> ListClipperToken<'ui> {
    fn new(_: &Ui, list_clipper: *mut sys::ImGuiListClipper) -> Self {
        Self {
            list_clipper,
            _phantom: PhantomData,
        }
    }

    pub fn step(&mut self) -> bool {
        unsafe { sys::ImGuiListClipper_Step(self.list_clipper) }
    }

    pub fn end(&mut self) {
        unsafe {
            sys::ImGuiListClipper_End(self.list_clipper);
        }
    }

    pub fn display_start(&self) -> i32 {
        unsafe { (*self.list_clipper).DisplayStart }
    }

    pub fn display_end(&self) -> i32 {
        unsafe { (*self.list_clipper).DisplayEnd }
    }
}

impl<'ui> Drop for ListClipperToken<'ui> {
    fn drop(&mut self) {
        self.end();
        unsafe {
            sys::ImGuiListClipper_destroy(self.list_clipper);
        };
    }
}
