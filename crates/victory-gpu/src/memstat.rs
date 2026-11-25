//! Once-a-second memory breakdown in diagnostics builds.
//!
//! `VICTORY_MEM_STATS=1` logs the process's working set and private bytes,
//! the system's free RAM and commit, and the emulator's own known caches, so
//! a run that grows to tens of GB shows which category is missing from the
//! ones we can already count. Called from the presenter's one-second tick.

use windows_sys::Win32::System::ProcessStatus::{
    GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS_EX,
};
use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
use windows_sys::Win32::System::Threading::GetCurrentProcess;

/// Logs the breakdown, once a second, when `VICTORY_MEM_STATS` is on.
pub(crate) fn log() {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if !*ON.get_or_init(|| crate::switch::on("VICTORY_MEM_STATS")) {
        return;
    }

    let mut counters = PROCESS_MEMORY_COUNTERS_EX::default();
    counters.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32;
    // SAFETY: plain queries with valid pointers; the struct reports its size.
    let (working, private, peak, faults) = unsafe {
        if GetProcessMemoryInfo(GetCurrentProcess(), (&raw mut counters).cast(), counters.cb) != 0 {
            (
                counters.WorkingSetSize,
                counters.PrivateUsage,
                counters.PeakWorkingSetSize,
                counters.PageFaultCount,
            )
        } else {
            (0, 0, 0, 0)
        }
    };

    let mut status = MEMORYSTATUSEX::default();
    status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
    // SAFETY: a plain query with a valid, correctly sized struct.
    let (avail_phys, commit_used, commit_limit) = unsafe {
        if GlobalMemoryStatusEx(&mut status) != 0 {
            (
                status.ullAvailPhys,
                status.ullTotalPageFile - status.ullAvailPageFile,
                status.ullTotalPageFile,
            )
        } else {
            (0, 0, 0)
        }
    };

    let (textures, texture_bytes, used, _, _, _) = crate::resource::texture_counts();
    let (shaders, shader_bytes) = crate::pipeline::shader_stats();
    let (made, loaded, created, distinct, library_bytes) = crate::pipeline::library_stats();
    let snapshot_bytes = crate::shadow::pool_bytes();
    let (shadow_buffers, shadow_bytes, shadow_made) = crate::shadow::buffers_stats();
    let small_bytes = crate::host::host().map_or(0, |h| h.small_buffer_bytes());

    let mb = |bytes: u64| bytes as f64 / 1e6;
    // Everything the emulator counts. VRAM textures are not part of the
    // process's private bytes, so they are logged but not subtracted.
    let counted = shader_bytes + library_bytes + snapshot_bytes + small_bytes;
    tracing::info!(
        ws_mb = mb(working as u64),
        private_mb = mb(private as u64),
        peak_ws_mb = mb(peak as u64),
        faults,
        avail_phys_mb = mb(avail_phys),
        commit_used_mb = mb(commit_used),
        commit_limit_mb = mb(commit_limit),
        textures,
        tex_vram_mb = mb(texture_bytes),
        tex_used_mb = mb(used),
        shaders,
        shader_mb = mb(shader_bytes),
        made,
        loaded,
        created,
        distinct,
        library_mb = mb(library_bytes),
        snapshot_mb = mb(snapshot_bytes),
        shadow_buffers,
        shadow_mb = mb(shadow_bytes),
        shadow_made,
        small_mb = mb(small_bytes),
        counted_mb = mb(counted),
        "memory breakdown"
    );
}
