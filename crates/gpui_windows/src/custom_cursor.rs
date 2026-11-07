//! Native image cursors (Snafu): an owned HCURSOR built from a GPUI `CustomCursor`'s PNG,
//! keeping its alpha and hotspot.

use std::ptr::{copy_nonoverlapping, null_mut};

use anyhow::{Context as _, Result, anyhow};
use gpui::CustomCursor;
use windows::Win32::{
    Graphics::Gdi::{
        BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateBitmap, CreateDIBSection, DIB_RGB_COLORS,
        DeleteObject, HBITMAP, HGDIOBJ,
    },
    UI::WindowsAndMessaging::{CreateIconIndirect, HCURSOR, ICONINFO},
};

/// Build an owned HCURSOR; the caller destroys it with `DestroyCursor`.
pub(crate) fn create_custom_cursor(cursor: &CustomCursor) -> Result<HCURSOR> {
    let bitmap = image::load_from_memory(cursor.png_data())?.to_rgba8();
    let (width, height) = bitmap.dimensions();
    if width == 0 || height == 0 || width > i32::MAX as u32 || height > i32::MAX as u32 {
        return Err(anyhow!("invalid custom cursor bitmap size {width}x{height}"));
    }

    let info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width as i32,
            // Top-down rows keep the RGBA source order aligned with the DIB memory.
            biHeight: -(height as i32),
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            ..Default::default()
        },
        ..Default::default()
    };
    let mut dib_bits = null_mut();
    let color_bitmap = unsafe {
        CreateDIBSection(None, &info, DIB_RGB_COLORS, &mut dib_bits, None, 0)
            .context("Creating custom cursor colour bitmap")?
    };
    if dib_bits.is_null() {
        delete_bitmap(color_bitmap);
        return Err(anyhow!("custom cursor colour bitmap has no pixel buffer"));
    }

    // Premultiplied BGRA, as Windows expects for 32-bit cursors.
    let mut bgra = Vec::with_capacity(bitmap.as_raw().len());
    for pixel in bitmap.pixels() {
        let alpha = u16::from(pixel[3]);
        bgra.extend([
            ((u16::from(pixel[2]) * alpha + 127) / 255) as u8,
            ((u16::from(pixel[1]) * alpha + 127) / 255) as u8,
            ((u16::from(pixel[0]) * alpha + 127) / 255) as u8,
            pixel[3],
        ]);
    }
    unsafe { copy_nonoverlapping(bgra.as_ptr(), dib_bits.cast(), bgra.len()) };

    // The monochrome mask marks fully transparent pixels; alpha carries smooth edges.
    let mask_stride = width.div_ceil(16) as usize * 2;
    let mut mask_bits = vec![0u8; mask_stride * height as usize];
    for (index, pixel) in bitmap.pixels().enumerate() {
        if pixel[3] == 0 {
            let x = index as u32 % width;
            let y = index as u32 / width;
            mask_bits[y as usize * mask_stride + x as usize / 8] |= 0x80 >> (x % 8);
        }
    }
    let mask_bitmap = unsafe {
        CreateBitmap(
            width as i32,
            height as i32,
            1,
            1,
            Some(mask_bits.as_ptr().cast()),
        )
    };
    if mask_bitmap.is_invalid() {
        delete_bitmap(color_bitmap);
        return Err(anyhow!("Creating custom cursor transparency mask failed"));
    }

    let logical = cursor.size();
    let hotspot = cursor.hotspot();
    let hotspot_x = (f32::from(hotspot.x) / f32::from(logical.width) * width as f32)
        .round()
        .clamp(0., width.saturating_sub(1) as f32) as u32;
    let hotspot_y = (f32::from(hotspot.y) / f32::from(logical.height) * height as f32)
        .round()
        .clamp(0., height.saturating_sub(1) as f32) as u32;
    let info = ICONINFO {
        fIcon: false.into(),
        xHotspot: hotspot_x,
        yHotspot: hotspot_y,
        hbmMask: mask_bitmap,
        hbmColor: color_bitmap,
    };
    let result =
        unsafe { CreateIconIndirect(&info) }.context("Creating custom Windows cursor from bitmaps");
    delete_bitmap(mask_bitmap);
    delete_bitmap(color_bitmap);
    result.map(|icon| HCURSOR(icon.0))
}

fn delete_bitmap(bitmap: HBITMAP) {
    unsafe {
        let _ = DeleteObject(HGDIOBJ(bitmap.0));
    }
}
