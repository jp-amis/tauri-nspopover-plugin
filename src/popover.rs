use std::any::Any;
use objc2::{ffi::YES, msg_send, rc::Id};
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_app_kit::{NSColor, NSPopover, NSPopoverBehavior, NSView, NSViewController, NSWindow};
use objc2_foundation::MainThreadMarker;

pub struct PopoverController {
    popover: Id<NSPopover>,
}

impl PopoverController {
    pub fn new(window: &NSWindow) -> Self {
        let popover = Self::create_popover(window);
        return PopoverController { popover };
    }

    pub fn popover(&self) -> Id<NSPopover> {
        self.popover.clone()
    }

    fn get_target_view(ns_window: &NSWindow) -> Id<NSView> {
        let view = ns_window.contentView().unwrap();
        view.setWantsLayer(true);
        unsafe {
            let color = NSColor::clearColor();
            let _: () = msg_send![<Retained<NSView> as AsRef<AnyObject>>::as_ref(&view), setBackgroundColor: <Retained<NSColor> as AsRef<AnyObject>>::as_ref(&color)];
            let _: () = msg_send![<Retained<NSView> as AsRef<AnyObject>>::as_ref(&view), setOpaque: YES];
        }

        return view;
    }

    fn create_popover(window: &NSWindow) -> Id<NSPopover> {
        let view = Self::get_target_view(window);
        unsafe {
            let mtm = MainThreadMarker::new().unwrap();
            let ctrl = NSViewController::new(mtm);

            ctrl.setView(view.as_ref());

            let popover = NSPopover::new(mtm);
            popover.setBehavior(NSPopoverBehavior::Transient);
            popover.setContentViewController(Some(ctrl.as_ref()));
            let content_size = window.frame().size;
            popover.setContentSize(content_size);

            popover
        }
    }
}
