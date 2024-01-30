// Generated with
//
//    bindgen --no-layout-tests \
//            --allowlist-var RUSAGE_INFO_V4 \
//            --allowlist-function proc_pid_rusage \
//            --allowlist-type rusage_info_v4 \
//             -o rusagev4.rs \
//              /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/libproc.h

#![allow(non_camel_case_types)]

/* automatically generated by rust-bindgen 0.60.1 */

pub const RUSAGE_INFO_V4: u32 = 4;
pub type rusage_info_t = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct rusage_info_v4 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
}
extern "C" {
    pub fn proc_pid_rusage(
        pid: ::std::os::raw::c_int,
        flavor: ::std::os::raw::c_int,
        buffer: *mut rusage_info_t,
    ) -> ::std::os::raw::c_int;
}