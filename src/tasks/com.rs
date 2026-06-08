use crate::categories::Categories;
use crate::tasks::{ScratchBuffer, TaskDescriptor, TaskParams};
use crate::workdata::WorkData;
use rand::rngs::ThreadRng;
use std::hint::black_box;
use windows::core::BSTR;
use windows::Win32::System::Com::*;
use windows::Win32::System::Wmi::*;

pub fn register() -> Vec<TaskDescriptor> {
    vec![
        TaskDescriptor { name: "wmi_processes", category: Categories::COM, func: wmi_processes },
        TaskDescriptor { name: "wmi_os_info", category: Categories::COM, func: wmi_os_info },
        TaskDescriptor { name: "wmi_computer_system", category: Categories::COM, func: wmi_computer_system },
        TaskDescriptor { name: "wmi_network_adapters", category: Categories::COM, func: wmi_network_adapters },
        TaskDescriptor { name: "wmi_logical_disks", category: Categories::COM, func: wmi_logical_disks },
        TaskDescriptor { name: "wmi_services", category: Categories::COM, func: wmi_services },
        TaskDescriptor { name: "wmi_bios_info", category: Categories::COM, func: wmi_bios_info },
        TaskDescriptor { name: "wmi_processor_info", category: Categories::COM, func: wmi_processor_info },
    ]
}

unsafe fn init_com() -> bool {
    let hr = CoInitializeEx(None, COINIT_MULTITHREADED);
    if hr.is_err() {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
    }
    let _ = CoInitializeSecurity(
        None,
        -1,
        None,
        None,
        RPC_C_AUTHN_LEVEL_DEFAULT,
        RPC_C_IMP_LEVEL_IMPERSONATE,
        None,
        EOAC_NONE,
        None,
    );
    true
}

unsafe fn wmi_query(query: &str, max_results: usize, scratch: &mut ScratchBuffer) {
    init_com();

    let locator: IWbemLocator = match CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER) {
        Ok(l) => l,
        Err(_) => return,
    };

    let server: IWbemServices = match locator.ConnectServer(
        &BSTR::from("ROOT\\CIMV2"),
        &BSTR::from(""),
        &BSTR::from(""),
        &BSTR::from(""),
        0,
        &BSTR::from(""),
        None,
    ) {
        Ok(s) => s,
        Err(_) => return,
    };

    let flags = WBEM_GENERIC_FLAG_TYPE(
        WBEM_FLAG_FORWARD_ONLY.0 | WBEM_FLAG_RETURN_IMMEDIATELY.0,
    );

    let enumerator: IEnumWbemClassObject = match server.ExecQuery(
        &BSTR::from("WQL"),
        &BSTR::from(query),
        flags,
        None,
    ) {
        Ok(e) => e,
        Err(_) => return,
    };

    let mut count = 0usize;
    loop {
        if count >= max_results {
            break;
        }
        let mut row = [None; 1];
        let mut returned = 0u32;
        let hr = enumerator.Next(-1, &mut row, &mut returned);
        if hr.is_err() || returned == 0 {
            break;
        }
        if let Some(ref obj) = row[0] {
            black_box(obj);
        }
        count += 1;
    }
    scratch.absorb(&count.to_ne_bytes());
}

fn wmi_processes(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let max_results = params.iterations.min(200).saturating_add(work.derive_usize(0) % 8);
    unsafe { wmi_query("SELECT ProcessId, Name FROM Win32_Process", max_results, scratch); }
}

fn wmi_os_info(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let max_results = params.call_depth.saturating_add(work.blend_seed() as usize % 2);
    unsafe { wmi_query("SELECT Caption, Version, BuildNumber FROM Win32_OperatingSystem", max_results, scratch); }
}

fn wmi_computer_system(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let max_results = params.call_depth.saturating_add(work.blend_seed() as usize % 2);
    unsafe { wmi_query("SELECT Name, Domain, TotalPhysicalMemory FROM Win32_ComputerSystem", max_results, scratch); }
}

fn wmi_network_adapters(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let max_results = params.iterations.min(50).saturating_add(work.derive_usize(1) % 4);
    unsafe { wmi_query("SELECT Description, MACAddress FROM Win32_NetworkAdapterConfiguration", max_results, scratch); }
}

fn wmi_logical_disks(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let max_results = params.iterations.min(30).saturating_add(work.derive_usize(0) % 4);
    unsafe { wmi_query("SELECT DeviceID, Size, FreeSpace FROM Win32_LogicalDisk", max_results, scratch); }
}

fn wmi_services(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let max_results = params.iterations.min(200).saturating_add(work.derive_usize(0) % 8);
    unsafe { wmi_query("SELECT Name, State, StartMode FROM Win32_Service", max_results, scratch); }
}

fn wmi_bios_info(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let max_results = params.call_depth.saturating_add(work.blend_seed() as usize % 2);
    unsafe { wmi_query("SELECT Manufacturer, Version FROM Win32_BIOS", max_results, scratch); }
}

fn wmi_processor_info(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let max_results = params.call_depth.saturating_add(work.blend_seed() as usize % 2);
    unsafe { wmi_query("SELECT Name, NumberOfCores, MaxClockSpeed FROM Win32_Processor", max_results, scratch); }
}
