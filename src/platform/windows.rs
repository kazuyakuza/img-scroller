//! Windows implementation of the platform facade: native open dialog and WIC
//! decoding. Compiled only on Windows targets; first-compile validation is the
//! cross-target check step of this task's implementation.

use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::{
    GENERIC_READ, HWND, RPC_E_CHANGED_MODE, S_FALSE, S_OK, WINCODEC_ERR_COMPONENTNOTFOUND,
};
use windows::Win32::Graphics::Imaging::{
    CLSID_WICImagingFactory, GUID_WICPixelFormat32bppRGBA, IWICBitmapDecoder, IWICBitmapFrameDecode,
    IWICFormatConverter, IWICImagingFactory, IWICPalette, WICBitmapDitherTypeNone,
    WICBitmapPaletteTypeCustom, WICDecodeMetadataCacheOnDemand,
};
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoTaskMemFree,
};
use windows::Win32::UI::Shell::{
    FOS_FILEMUSTEXIST, FOS_FORCEFILESYSTEM, FOS_PATHMUSTEXIST, FileOpenDialog, IFileOpenDialog,
    IShellItem, SIGDN_FILESYSPATH,
};
use windows::core::{Error, PCWSTR};

use crate::image_loading::{DecodedImage, ImageLoadFailure, pixel_buffer_size};

pub(crate) fn show_open_image_dialog(frame: &eframe::Frame) -> Option<PathBuf> {
    if !ensure_com_initialized() {
        return None;
    }
    run_open_dialog(owner_window(frame)).ok()
}

pub(crate) fn decode_image(path: &Path) -> Result<DecodedImage, ImageLoadFailure> {
    if !ensure_com_initialized() {
        return Err(ImageLoadFailure::Unloadable);
    }
    decode_with_factory(path)
}

fn ensure_com_initialized() -> bool {
    let com_state = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    match com_state {
        S_OK | S_FALSE | RPC_E_CHANGED_MODE => true,
        _ => false,
    }
}

fn owner_window(frame: &eframe::Frame) -> Option<HWND> {
    match frame.window_handle().ok()?.as_raw() {
        RawWindowHandle::Win32(handle) => Some(HWND(handle.hwnd.get() as *mut _)),
        _ => None,
    }
}

fn run_open_dialog(owner: Option<HWND>) -> Result<PathBuf, Error> {
    unsafe {
        let dialog: IFileOpenDialog =
            CoCreateInstance(&FileOpenDialog, None, CLSCTX_INPROC_SERVER)?;
        let options =
            dialog.GetOptions()? | FOS_FORCEFILESYSTEM | FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST;
        dialog.SetOptions(options)?;
        dialog.Show(owner)?;
        selected_dialog_path(dialog)
    }
}

unsafe fn selected_dialog_path(dialog: IFileOpenDialog) -> Result<PathBuf, Error> {
    unsafe {
        let item: IShellItem = dialog.GetResult()?;
        let wide_path = item.GetDisplayName(SIGDN_FILESYSPATH)?;
        let file_name = wide_path
            .to_string()
            .map_err(|_| Error::from_hresult(windows::Win32::Foundation::E_FAIL))?;
        CoTaskMemFree(Some(wide_path.as_ptr().cast::<core::ffi::c_void>()));
        Ok(PathBuf::from(file_name))
    }
}

fn decode_with_factory(path: &Path) -> Result<DecodedImage, ImageLoadFailure> {
    unsafe {
        let factory: IWICImagingFactory =
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)
                .map_err(classify_failure)?;
        let frame = frame_from_path(&factory, path)?;
        decoded_image(&factory, &frame)
    }
}

unsafe fn frame_from_path(
    factory: &IWICImagingFactory,
    path: &Path,
) -> Result<IWICBitmapFrameDecode, ImageLoadFailure> {
    unsafe {
        let file_path: Vec<u16> = path.as_os_str().encode_wide().chain([0]).collect();
        let decoder: IWICBitmapDecoder = factory
            .CreateDecoderFromFilename(
                PCWSTR(file_path.as_ptr()),
                None,
                GENERIC_READ,
                WICDecodeMetadataCacheOnDemand,
            )
            .map_err(classify_failure)?;
        decoder.GetFrame(0).map_err(classify_failure)
    }
}

unsafe fn decoded_image(
    factory: &IWICImagingFactory,
    frame: &IWICBitmapFrameDecode,
) -> Result<DecodedImage, ImageLoadFailure> {
    unsafe {
        let mut width = 0u32;
        let mut height = 0u32;
        frame
            .GetSize(&mut width, &mut height)
            .map_err(classify_failure)?;
        let byte_length = pixel_buffer_size(width, height).ok_or(ImageLoadFailure::Unloadable)?;
        let converter: IWICFormatConverter =
            factory.CreateFormatConverter().map_err(classify_failure)?;
        converter
            .Initialize(
                frame,
                &GUID_WICPixelFormat32bppRGBA,
                WICBitmapDitherTypeNone,
                None::<&IWICPalette>,
                0.0,
                WICBitmapPaletteTypeCustom,
            )
            .map_err(classify_failure)?;
        let mut pixels = vec![0u8; byte_length];
        let row_stride = width.checked_mul(4).ok_or(ImageLoadFailure::Unloadable)?;
        frame
            .CopyPixels(std::ptr::null(), row_stride, &mut pixels)
            .map_err(classify_failure)?;
        Ok(DecodedImage {
            width,
            height,
            pixels,
        })
    }
}

fn classify_failure(error: Error) -> ImageLoadFailure {
    match error.code() {
        WINCODEC_ERR_COMPONENTNOTFOUND => ImageLoadFailure::Unsupported,
        _ => ImageLoadFailure::Unloadable,
    }
}
