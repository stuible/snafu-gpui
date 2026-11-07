//! Owned native objects handed from a platform crate to the app (Snafu): one retain
//! each, released exactly once on drop. They hold raw pointers, so they are neither
//! `Send` nor `Sync` and stay on the thread that received them.

use std::ffi::c_void;

/// Retain and release functions for a native reference-counted object (for example
/// Objective-C `retain`/`release`), supplied by the platform crate that creates it.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct NativeObjectOps {
    /// Takes one extra retain and returns the same object.
    pub retain: unsafe fn(*mut c_void) -> *mut c_void,
    /// Gives up one retain.
    pub release: unsafe fn(*mut c_void),
}

struct NativeObject {
    raw: *mut c_void,
    ops: NativeObjectOps,
}

impl NativeObject {
    unsafe fn retain(raw: *mut c_void, ops: NativeObjectOps) -> Self {
        Self {
            raw: unsafe { (ops.retain)(raw) },
            ops,
        }
    }
}

impl Clone for NativeObject {
    fn clone(&self) -> Self {
        unsafe { Self::retain(self.raw, self.ops) }
    }
}

impl Drop for NativeObject {
    fn drop(&mut self) {
        unsafe { (self.ops.release)(self.raw) }
    }
}

/// An owned +1 retain of an `NSURL` delivered by AppKit's `application:openURLs:`.
/// Dropping it releases that retain exactly once.
pub struct NativeUrl(NativeObject);

impl NativeUrl {
    /// Borrows the retained `NSURL` for a synchronous native call. The caller must not
    /// release it or use it after this value drops.
    pub fn as_raw(&self) -> *mut c_void {
        self.0.raw
    }

    /// Takes one extra retain of `url`.
    ///
    /// # Safety
    /// `url` must be a live native object that `ops` can retain and release.
    #[doc(hidden)]
    pub unsafe fn retain(url: *mut c_void, ops: NativeObjectOps) -> Self {
        Self(unsafe { NativeObject::retain(url, ops) })
    }
}

/// An owned +1 retain of the `NSPasteboard` of one macOS drag operation
/// (`NSDraggingInfo.draggingPasteboard`). Dropping it releases that retain once.
///
/// Retaining conveys memory ownership only, not stable contents or a continuing
/// permission to read: AppKit may end the operation and clear the pasteboard once
/// `performDragOperation:` returns, so a consumer reads it synchronously during the
/// Submit dispatch that supplied it.
#[derive(Clone)]
pub struct MacNativeDragPasteboard(NativeObject);

impl MacNativeDragPasteboard {
    /// Borrows the retained `NSPasteboard` for a synchronous native call. The caller
    /// must not release it or use it after this value drops or the Submit dispatch ends.
    pub fn as_raw(&self) -> *mut c_void {
        self.0.raw
    }

    /// Takes one extra retain of `pasteboard`.
    ///
    /// # Safety
    /// `pasteboard` must be a live native object that `ops` can retain and release.
    #[doc(hidden)]
    pub unsafe fn retain(pasteboard: *mut c_void, ops: NativeObjectOps) -> Self {
        Self(unsafe { NativeObject::retain(pasteboard, ops) })
    }
}

#[cfg(test)]
mod tests {
    use super::{MacNativeDragPasteboard, NativeObjectOps, NativeUrl};
    use std::{cell::Cell, ffi::c_void};

    thread_local!(static COUNT: Cell<isize> = const { Cell::new(0) });

    unsafe fn retain(raw: *mut c_void) -> *mut c_void {
        COUNT.with(|count| count.set(count.get() + 1));
        raw
    }

    unsafe fn release(_: *mut c_void) {
        COUNT.with(|count| count.set(count.get() - 1));
    }

    const OPS: NativeObjectOps = NativeObjectOps { retain, release };

    #[test]
    fn each_value_holds_one_retain_released_once() {
        let mut object = 0u8;
        let raw = (&raw mut object).cast::<c_void>();
        {
            let url = unsafe { NativeUrl::retain(raw, OPS) };
            let pasteboard = unsafe { MacNativeDragPasteboard::retain(raw, OPS) };
            let copy = pasteboard.clone();
            assert_eq!(COUNT.with(Cell::get), 3);
            assert_eq!(url.as_raw(), raw);
            drop(copy);
            assert_eq!(COUNT.with(Cell::get), 2);
        }
        assert_eq!(COUNT.with(Cell::get), 0);
    }

    #[test]
    fn neither_type_is_send_or_sync() {
        // The inferred marker becomes ambiguous if a wrapper gains either trait.
        trait AmbiguousSend<A> {
            fn check() {}
        }
        impl<T: ?Sized> AmbiguousSend<()> for T {}
        impl<T: ?Sized + Send> AmbiguousSend<u8> for T {}
        trait AmbiguousSync<A> {
            fn check() {}
        }
        impl<T: ?Sized> AmbiguousSync<()> for T {}
        impl<T: ?Sized + Sync> AmbiguousSync<u8> for T {}
        let _ = <NativeUrl as AmbiguousSend<_>>::check;
        let _ = <NativeUrl as AmbiguousSync<_>>::check;
        let _ = <MacNativeDragPasteboard as AmbiguousSend<_>>::check;
        let _ = <MacNativeDragPasteboard as AmbiguousSync<_>>::check;
    }
}
