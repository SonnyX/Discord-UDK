//! This module contains functionality relevant to UDK logging.
use crate::dll::get_udk_slice;

/// Offset from the beginning of UDK64.exe to the debug log object.
#[cfg(target_arch = "x86_64")]
const DEBUG_LOG_OFFSET: usize = 0x0355_1720;
/// Address of UDK's log function.
#[cfg(target_arch = "x86_64")]
const DEBUG_FN_OFFSET: usize = 0x0024_6A20;

/// Offset from the beginning of UDK64.exe to the debug log object.
#[cfg(target_arch = "x86")]
const DEBUG_LOG_OFFSET: usize = 0x029a_31a8;
/// Address of UDK's log function.
#[cfg(target_arch = "x86")]
const DEBUG_FN_OFFSET: usize = 0x0050_78b0;

/// This is the type signature of UDK's log function.
type UDKLogFn = unsafe extern "C" fn(usize, u32, *const widestring::WideChar);

/// This enum represents the UDK message types.
#[repr(u32)]
pub enum LogType {
    Memory = 0xCA, // Clearing out Variable, saving
    CodeExecution = 0xD7, // OneBigger than Other
    MapCompilation = 0x2f4, // missing DecalMaterial
    Trace = 0x2f8, // Saving, imported
    Info = 0x2f9, // Unknown code token
    Init = 0x2FA,
    Warning = 0x2FF,
    CompileError = 0x301, // end of non-void function
    Something = 0x307, // No Chirp sound
    Error = 0x315,
    Timing = 0x45f, // timing
}

/// Log a message via the UDK logging framework.
pub fn log(typ: LogType, msg: &str) {
    let udk_slice = get_udk_slice();
    let log_obj = unsafe { udk_slice.as_ptr().add(DEBUG_LOG_OFFSET) };
    let log_fn: UDKLogFn = unsafe { std::mem::transmute(udk_slice.as_ptr().add(DEBUG_FN_OFFSET)) };

    // Convert the UTF-8 Rust string into an OS wide string.
    let wmsg = widestring::WideCString::from_str(&msg).unwrap();

    unsafe {
        (log_fn)(log_obj as usize, typ as u32, wmsg.as_ptr());
    }
}