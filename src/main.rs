#![feature(stdsimd)]
#![allow(non_upper_case_globals, dead_code, unused_imports)]
use bitflags::bitflags;
use core::arch::x86_64::{__cpuid, __get_cpuid_max, has_cpuid, CpuidResult};
use std::{
    fmt,
    fs::File,
    io::Write,
    mem::{size_of, transmute},
    str,
};

#[rustfmt::skip]
bitflags! {
    /// Feature Information
    #[repr(C)]
    struct Eax1EcxFlags: u32 {
        const sse3 =        1 << 0;
        const pclmulqdq =   1 << 1;
        const dtes64 =      1 << 2;
        const monitor =     1 << 3;
        const ds_cpl =      1 << 4;
        const vmx =         1 << 5;
        const smx =         1 << 6;
        const est =         1 << 7;
        const tm2 =         1 << 8;
        const ssse3 =       1 << 9;
        const cnxt_id =     1 << 10;
        const sdbg =        1 << 11;
        const fma =         1 << 12;
        const cx16 =        1 << 13;
        const xtpr =        1 << 14;
        const pdcm =        1 << 15;
        // 16th bit reserved
        const pcid =        1 << 17;
        const dca =         1 << 18;
        const sse4_1 =      1 << 19;
        const sse4_2 =      1 << 20;
        const x2apic =      1 << 21;
        const movbe =       1 << 22;
        const popcnt =      1 << 23;
        const tsc_deadline =1 << 24;
        const aes =         1 << 25;
        const xsave =       1 << 26;
        const osxsave =     1 << 27;
        const avx =         1 << 28;
        const f16c =        1 << 29;
        const rdrnd =       1 << 30;
        const hypervisor =  1 << 31;
    }
    /// Feature Information
    #[repr(C)]
    struct Eax1EdxFlags: u32 {
        const fpu =         1 << 0;
        const vme =         1 << 1;
        const de =          1 << 2;
        const pse =         1 << 3;
        const tsc =         1 << 4;
        const msr =         1 << 5;
        const pae =         1 << 6;
        const mce =         1 << 7;
        const cx8 =         1 << 8;
        const apic =        1 << 9;
        // 10th bit reserved
        const sep =         1 << 11;
        const mtrr =        1 << 12;
        const pge =         1 << 13;
        const mca =         1 << 14;
        const cmov =        1 << 15;
        const pat =         1 << 16;
        const pse_36 =      1 << 17;
        const psn =         1 << 18;
        const clfsh =       1 << 19;
        // 20th bit reserved
        const ds =          1 << 21;
        const acpi =        1 << 22;
        const mmx =         1 << 23;
        const fxsr =        1 << 24;
        const sse =         1 << 25;
        const sse2 =        1 << 26;
        const ss =          1 << 27;
        const htt =         1 << 28;
        const tm =          1 << 29;
        const ia64 =        1 << 30;
        const pbe =         1 << 31;
    }
    /// Thermal and power management
    #[repr(C)]
    struct Eax6EaxFlags: u32 {
        const digital_thermal_sensor_capability =           1 << 0;
        const intel_turbo_boost_technology_capability =     1 << 1;
        const always_running_apic_timer_capability =        1 << 2;
        // 4th bit reserved
        const power_limit_notification_capability =         1 << 4;
        const extended_clock_modulation_duty_capability =   1 << 5;
        const package_thermal_management_capability =       1 << 6;
        // 7th to 31st bits reserved
    }
    /// Thermal and power management
    #[repr(C)]
    struct Eax6EcxFlags: u32 {
        const hardware_coordination_feedback_capability =   1 << 0;
        const acnt2_capability =                            1 << 1;
        // 2nd bit reserved
        const performance_energy_bias_capability =          1 << 3;
        // 4th to 31st bits reserved
    }
    /// Extended Features
    #[repr(C)]
    struct Eax7EbxFlags: u32 {
        const fsgsbase =                        1 << 0;
        // No short name
        const IA32_TSC_ADJUST =                 1 << 1;
        const sgx =                             1 << 2;
        const bmi1 =                            1 << 3;
        const hle =                             1 << 4;
        const avx2 =                            1 << 5;
        // No short name
        const FDP_EXCPTN_ONLY =                 1 << 6;
        const smep =                            1 << 7;
        const bmi2 =                            1 << 8;
        const erms =                            1 << 9;
        const invpcid =                         1 << 10;
        const rtm =                             1 << 11;
        const pqdm =                            1 << 12;
        // No short name
        const FPU_CS_and_FPU_DS_deprecated =    1 << 13;
        const mpx =                             1 << 14;
        const pqe =                             1 << 15;
        const avx512_f =                        1 << 16;
        const avx512_dq =                       1 << 17;
        const rdseed =                          1 << 18;
        const adx =                             1 << 19;
        const smap =                            1 << 20;
        const avx512_ifma =                     1 << 21;
        const pccommit =                        1 << 22;
        const clflushopt =                      1 << 23;
        const clwb =                            1 << 24;
        const intel_pt =                        1 << 25;
        const avx512_pf =                       1 << 26;
        const avx512_er =                       1 << 27;
        const avx512_cd =                       1 << 28;
        const sha =                             1 << 29;
        const avx512_bw =                       1 << 30;
        const avx512_vl =                       1 << 31;
    }
    /// Extended Features
    #[repr(C)]
    struct Eax7EcxFlags: u32 {
        const prefetchwt1 =         1 << 0;
        const avx512_vbmi =         1 << 1;
        const umip =                1 << 2;
        const pku =                 1 << 3;
        const ospke =               1 << 4;
        const waitpkg =             1 << 5;
        const avx512_vbmi2 =        1 << 6;
        const cet_ss =              1 << 7;
        const gfni =                1 << 8;
        const vaes =                1 << 9;
        const vpclmulqdq =          1 << 10;
        const avx512_vnni =         1 << 11;
        const avx512_bitalg =       1 << 12;
        const TIME_END =            1 << 13;
        const avx512_vpopcntdq =    1 << 14;
        // 15th bit reserved
        const _5_level_paging =     1 << 16;
        // mawau uses 17th to 21th bits (inclusive)
        const rdpid =               1 << 22;
        const KL =                  1 << 23;
        // 24th bit reserved
        const cldemote =            1 << 25;
        // 26th bit reserved
        const MOVDIRI =             1 << 27;
        const MOVDIR64B =           1 << 28;
        const ENQCMD =              1 << 29;
        const sgx_lc =              1 << 30;
        const pks =                 1 << 31;
    }
    /// Extended Features
    #[repr(C)]
    struct Eax7EdxFlags: u32 {
        // 1st bit reserved
        // 2nd bit reserved
        const avx512_4vnniw = 1 << 2;
        const avx512_4fmaps = 1 << 3;
        const fsrm = 1 << 4;
        const uintr = 1 << 5;
        // 6th bit reserved
        // 7th bit reserved
        const avx512_vp2intersect = 1 << 8;
        const SRBDS_CTRL = 1 << 9;
        const md_clear = 1 << 10;
        const RMT_ALWAYS_ABORT = 1 << 11;
        // 12th bit reserved
        const TSX_FORCE_ABORT = 1 << 13;
        const SERIALIZE = 1 << 14;
        const Hybrid = 1 << 15;
        const TSXLDTRK = 1 << 16;
        // 17th bit reserved
        const pcconfig = 1 << 18;
        const lbr = 1 << 19;
        const cet_ibt = 1 << 20;
        // 21st bit reserved
        const amx_bf16 = 1 << 22;
        const AVX512_FP16 = 1 << 23;
        const amx_tile = 1 << 24;
        const amx_int8 = 1 << 25;
        const IBRS_IBPB_spec_ctrl = 1 << 26;
        const stibp = 1 << 27;
        const L1D_FLUSH = 1 << 28;
        const IA32_ARCH_CAPABILITIES = 1 << 29;
        const IA32_CORE_CAPABILITIES = 1 << 30;
        const ssbd = 1 << 31;
    }
}
#[repr(C)]
struct Cpuid {
    /// eax 0
    manufacturer_id: [u8; 12],
    /// eax 1
    process_info_and_feature_bits: ProcessorInfoAndFeatureBits,
    /// eax 6
    thermal_and_power_management: ThermalAndPowerManagement,
    /// eax 7
    extended_features: ExtendedFeatures,
}
impl Cpuid {
    pub fn new() -> Self {
        Self {
            manufacturer_id: {
                let CpuidResult {
                    eax: _,
                    ebx,
                    ecx,
                    edx,
                } = unsafe { __cpuid(0) };
                let (ebx_bytes, edx_bytes, ecx_bytes) = unsafe {
                    (
                        transmute::<_, [u8; 4]>(ebx),
                        transmute::<_, [u8; 4]>(edx),
                        transmute::<_, [u8; 4]>(ecx),
                    )
                };
                [ebx_bytes, edx_bytes, ecx_bytes]
                    .concat()
                    .try_into()
                    .unwrap()
            },
            process_info_and_feature_bits: {
                let CpuidResult { eax, ebx, ecx, edx } = unsafe { __cpuid(1) };
                ProcessorInfoAndFeatureBits {
                    processor_version_information: ProcessorVersionInformation(eax),
                    additional_information: unsafe { transmute::<_, AdditionalInformation>(ebx) },
                    feature_information: FeatureInformation {
                        ecx: Eax1EcxFlags { bits: ecx },
                        edx: Eax1EdxFlags { bits: edx },
                    },
                }
            },
            thermal_and_power_management: {
                let CpuidResult {
                    eax,
                    ebx,
                    ecx,
                    edx: _,
                } = unsafe { __cpuid(6) };
                ThermalAndPowerManagement {
                    features: ThermalAndPowerManagementFeatures {
                        eax: Eax6EaxFlags { bits: eax },
                        ecx: Eax6EcxFlags { bits: ecx },
                    },
                    number_of_interrupt_thresholds: Eax6EbxFlags(ebx),
                }
            },
            extended_features: {
                let CpuidResult {
                    eax: _,
                    ebx,
                    ecx,
                    edx,
                } = unsafe { __cpuid(7) };
                ExtendedFeatures {
                    ebx: Eax7EbxFlags { bits: ebx },
                    ecx: Eax7EcxFlags { bits: ecx },
                    edx: Eax7EdxFlags { bits: edx },
                }
            },
        }
    }
}
impl fmt::Debug for Cpuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cpuid")
            .field(
                "manufacturer_id",
                &str::from_utf8(&self.manufacturer_id).unwrap(),
            )
            .field(
                "process_info_and_feature_bits",
                &self.process_info_and_feature_bits,
            )
            .field(
                "thermal_and_power_management",
                &self.thermal_and_power_management,
            )
            .field("extended_features", &self.extended_features)
            .finish()
    }
}
#[derive(Debug)]
#[repr(C)]
struct ProcessorInfoAndFeatureBits {
    processor_version_information: ProcessorVersionInformation,
    additional_information: AdditionalInformation,
    feature_information: FeatureInformation,
}
#[repr(C)]
struct ProcessorVersionInformation(u32);
impl ProcessorVersionInformation {
    fn stepping_id(&self) -> u8 {
        (self.0 & 0b00000000_00000000_00000000_00001111) as u8
    }
    fn model(&self) -> u8 {
        ((self.0 & 0b00000000_00000000_00000000_11110000) >> 4) as u8
    }
    fn family_id(&self) -> u8 {
        ((self.0 & 0b00000000_00000000_00001111_00000000) >> 8) as u8
    }
    fn processor_type(&self) -> u8 {
        ((self.0 & 0b00000000_00000000_00110000_00000000) >> 12) as u8
    }
    fn extended_model_id(&self) -> u8 {
        ((self.0 & 0b00000000_00001111_00000000_00000000) >> 16) as u8
    }
    fn extended_family_id(&self) -> u8 {
        ((self.0 & 0b00001111_11110000_00000000_00000000) >> 20) as u8
    }
}
impl fmt::Debug for ProcessorVersionInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProcessorVersionInformation")
            .field("stepping_id", &self.stepping_id())
            .field("model", &self.model())
            .field("family_id", &self.family_id())
            .field("processor_type", &self.processor_type())
            .field("extended_model_id", &self.extended_model_id())
            .field("extended_family_id", &self.extended_family_id())
            .finish()
    }
}

#[derive(Debug)]
#[repr(C)]
struct AdditionalInformation {
    brand_index: u8,
    clflush_line_size: u8,
    maximum_addressable_logical_processor_ids: u8,
    local_apic_id: u8,
}
#[repr(C)]
struct FeatureInformation {
    ecx: Eax1EcxFlags,
    edx: Eax1EdxFlags,
}
impl fmt::Debug for FeatureInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.ecx.is_empty(), self.edx.is_empty()) {
            (true, true) => write!(f, "(empty)"),
            (false, true) => write!(f, "{:?}", self.ecx),
            (true, false) => write!(f, "{:?}", self.edx),
            (false, false) => write!(f, "{:?} | {:?}", self.ecx, self.edx),
        }
    }
}
#[derive(Debug)]
#[repr(C)]
struct ThermalAndPowerManagement {
    features: ThermalAndPowerManagementFeatures,
    number_of_interrupt_thresholds: Eax6EbxFlags,
}
#[repr(C)]
struct ThermalAndPowerManagementFeatures {
    eax: Eax6EaxFlags,
    ecx: Eax6EcxFlags,
}
impl fmt::Debug for ThermalAndPowerManagementFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.eax.is_empty(), self.ecx.is_empty()) {
            (true, true) => write!(f, "(empty)"),
            (false, true) => write!(f, "{:?}", self.eax),
            (true, false) => write!(f, "{:?}", self.ecx),
            (false, false) => write!(f, "{:?} | {:?}", self.eax, self.ecx),
        }
    }
}
#[repr(C)]
struct Eax6EbxFlags(u32);
impl Eax6EbxFlags {
    fn number_of_interrupt_thresholds(&self) -> u8 {
        (self.0 & 0b00001111) as u8
    }
}
impl fmt::Debug for Eax6EbxFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.number_of_interrupt_thresholds())
    }
}
#[repr(C)]
struct ExtendedFeatures {
    ebx: Eax7EbxFlags,
    ecx: Eax7EcxFlags,
    edx: Eax7EdxFlags,
}
impl fmt::Debug for ExtendedFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (
            self.ebx.is_empty(),
            self.ecx.is_empty(),
            self.edx.is_empty(),
        ) {
            (true, true, true) => write!(f, "(empty)"),
            (false, true, true) => write!(f, "{:?}", self.ebx),
            (true, false, true) => write!(f, "{:?}", self.ecx),
            (true, true, false) => write!(f, "{:?}", self.edx),
            (false, false, true) => write!(f, "{:?} | {:?}", self.ebx, self.ecx),
            (true, false, false) => write!(f, "{:?} | {:?}", self.ecx, self.edx),
            (false, true, false) => write!(f, "{:?} | {:?}", self.ebx, self.edx),
            (false, false, false) => write!(f, "{:?} | {:?} | {:?}", self.ebx, self.ecx, self.edx),
        }
    }
}

fn main() {
    // We load cpuid
    let cpuid = Cpuid::new();
    println!("cpuid: {:#?}", cpuid);
    // We transmute/reinterpret-cast it to an array of bytes
    let bytes = unsafe { transmute::<_, [u8; size_of::<Cpuid>()]>(cpuid) };
    println!("bytes: {:?}", bytes);
    // We store these bytes in a binary file
    let mut file = File::create("cpuid-x86_64").unwrap();
    file.write_all(&bytes).unwrap();

    // We can then define a constant based off these bytes
    const cpuid_bytes: &'static [u8; size_of::<Cpuid>()] = include_bytes!("../cpuid-x86_64");
    println!("cpuid_bytes: {:?}", cpuid_bytes);
    assert_eq!(size_of::<Cpuid>(), cpuid_bytes.len());
    const cpuid_from_bytes: Cpuid =
        unsafe { transmute::<[u8; size_of::<Cpuid>()], Cpuid>(*cpuid_bytes) };
    // Allowing us to package our cpuid template with our program with
    println!("cpuid_from_bytes: {:#?}", cpuid_from_bytes);
}
