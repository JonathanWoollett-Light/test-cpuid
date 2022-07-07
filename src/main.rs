#![allow(non_upper_case_globals, dead_code, unused_imports)]
use bitflags::bitflags;
use core::arch::x86_64::{CpuidResult, __cpuid, __cpuid_count, __get_cpuid_max};
use std::{
    fmt,
    fs::File,
    io::Write,
    mem::{size_of, transmute},
    str,
};
// L1S0Ecx refers to the ecx value in leaf 1, sub-leaf 0 of cpuid.
#[rustfmt::skip]
bitflags! {
    /// Feature Information
    #[repr(C)]
    struct L1S0Ecx: u32 {
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
    struct L1S0Edx: u32 {
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
    struct L6S0Eax: u32 {
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
    struct L6S0Ecx: u32 {
        const hardware_coordination_feedback_capability =   1 << 0;
        const acnt2_capability =                            1 << 1;
        // 2nd bit reserved
        const performance_energy_bias_capability =          1 << 3;
        // 4th to 31st bits reserved
    }
    /// Extended Features
    #[repr(C)]
    struct L7S0Ebx: u32 {
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
    struct L7S0Ecx: u32 {
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
    struct L7S0Edx: u32 {
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
    #[repr(C)]
    struct L7S1Eax: u32 {
        // 0 to 3rd bits reserved
        const avx_vnni =                    1 << 4;
        const avx512_bf16 =                 1 << 5;
        // 6th to 9th bits reserved
        const fast_zero_rep_movsb =         1 << 10;
        const fast_short_rep_stosb =        1 << 11;
        const fast_short_rep_cmpsb_scasb =  1 << 12;
        // 13th to 16th bits reserved
        const fred =                        1 << 17;
        const lkgs =                        1 << 18;
        // 19th to 21th bits reserved
        const hreset =                      1 << 22;
        // 23rd to 31th bits reserved
    }
    #[repr(C)]
    struct L13S1Eax: u32 {
        const xsaveopt =    1 << 0;
        const xsavec =      1 << 1;
        const xgetbv_ecx1 = 1 << 2;
        const xss =         1 << 3;
        // 4th to 31st bits reserved.
    }
    #[repr(C)]
    struct L18S0Eax: u32 {
        const sgx1 = 1 << 0;
        const sgx2 = 1 << 1;
        // 2nd to 4th bits reserved.
        const oss = 1 << 5;
        const encls = 1 << 6;
        // 7th to 31st bits reserved.
    }
    #[repr(C)]
    struct L20S0Ebx: u32 {
        // 0 to 3rd bits reserved.
        const ptwrite = 1 << 4;
        // 5th to 31st bits reserved.
    }
    #[repr(C)]
    struct L25S0Ebx: u32 {
        const aes_kle = 1 << 0;
        // 1st bit reserved.
        const aes_wide_kl = 1 << 2;
        // 3rd bit reserved.
        const kl_msrs = 1 << 4;
        // 5th to 31st bits reserved.
    }
    #[repr(C)]
    struct L1GBS0Edx: u32 {
        // Duplicates are from leaf 1 sub-leaf 0 edx.
        // 0 to 9nth bits are duplicates.
        // 10th bit is reserved.
        const syscall =     1 << 11;
        // 12th to 17th bits are duplicates.
        // 18th bit is reserved.
        const mp =          1 << 19;
        const nx =          1 << 20;
        // 21st bit is reserved.
        const mmxext =      1 << 22;
        // 23rd and 24th bits are duplicates.
        const fxsr_opt =    1 << 25;
        const pdpe1gb =     1 << 26;
        const rdtscp =      1 << 27;
        // 28th bit is reserved
        const lm =          1 << 29;
        const _3dnowext =   1 << 30;
        const _3dnow =      1 << 31;
    }
    #[repr(C)]
    struct L1GBS0Ecx: u32 {
        const lahf =            1 << 0;
        const cmp_legacy =      1 << 1;
        const svm =             1 << 2;
        const extapic =         1 << 3;
        const cr8_legacy =      1 << 4;
        const abm =             1 << 5;
        const sse4a =           1 << 6;
        const missalignsse =    1 << 7;
        const _3dnowprefetch =  1 << 8;
        const osvw =            1 << 9;
        const ibs =             1 << 10;
        const xop =             1 << 11;
        const skinit =          1 << 12;
        const wdt =             1 << 13;
        // 14th bit reserved
        const lwp =             1 << 15;
        const fma4 =            1 << 16;
        const tce =             1 << 17;
        // 18th bit reserved
        const nodeid_msr =      1 << 19;
        const tbm =             1 << 21;
        const topoext =         1 << 22;
        const perfctr_core =    1 << 23;
        const perfctr_nb =      1 << 24;
        // 25th bit reserved
        const dbx =             1 << 26;
        const perftsc =         1 << 27;
        const pcx_l2i =         1 << 28;
        const monitorx =        1 << 29;
        const addr_mask_ext =   1 << 30;
        // 31st bit reserved
    }
    #[repr(C)]
    struct L0X80000008S0Ebx: u32 {
        const clzero = 1 << 0;
        const retired_instr = 1 << 1;
        const xrstor_fp_err = 1 << 2;
        const invlpgb = 1 << 3;
        const rdpru = 1 << 4;
        // 5th to 7th bits reserved.
        const mcommit = 1 << 8;
        const wbnoinvd = 1 << 9;
        // 10th and 11th bits reserved.
        const ibpb = 1 << 12;
        const wbinvd_int = 1 << 13;
        const ibrs = 1 << 14;
        const single_thread_ibp = 1 << 15;
        // 16th bit reserved.
        const single_thread_ibp_ao = 1 << 17;
        // 18th and 19th bits reserved.
        const no_efer_lmsle = 1 << 20;
        const invlpgb_nested = 1 << 21;
        // 22nd bit reserved.
        const ppin = 1 << 23;
        const ssbd = 1 << 24;
        const virt_ssbd = 1 << 25;
        const ssb_no = 1 << 26;
        //26th to 31st bits reserved.
    }
    #[repr(C)]
    struct L0X80000001FS0Eax: u32 {
        const sme = 1 << 0;
        const sev = 1 << 1;
        const page_flush = 1 << 2;
        const sev_es = 1 << 3;
        const sev_snp = 1 << 4;
        const vmpl = 1 << 5;
        // 6th to 9th bits are reserved.
        const hw_cache_coherency = 1 << 10;
        const _64_host = 1 << 11;
        const restricted_injection = 1 << 12;
        const alternative_injection = 1 << 13;
        const debug_swap = 1 << 14;
        const prevent_host_ibs = 1 << 15;
        const vte = 1 << 16;
        // 17th to 31st bits reserved.
    }
}
#[repr(C)]
struct Cpuid {
    /// leaf 0
    manufacturer_id: [u8; 12],
    /// leaf 1
    process_info_and_feature_bits: ProcessorInfoAndFeatureBits,
    /// leaf 6
    thermal_and_power_management: ThermalAndPowerManagement,
    /// leaf 7
    extended_features: ExtendedFeatures,
    /// leaf 13
    cpuid_feature_bits1: L13S1Eax,
    /// leaf 18
    cpuid_feature_bits2: L18S0Eax,
    /// leaf 20
    cpuid_feature_bits3: L20S0Ebx,
    /// leaf 25
    cpuid_feature_bits4: L25S0Ebx,
    /// leaf 0x80000001
    extended_processor_info_and_feature_bits: ExtendedProcessorInfoAndFeatureBits,
    /// leaf 0x80000008
    virtual_and_physical_address_sizes: VirtualAndPhysicalAddressSizes,
    /// leaf 0x8000001F
    cpuid_feature_bits5: L0X80000001FS0Eax,
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
                let CpuidResult { eax, ebx, ecx, edx } = unsafe { __cpuid_count(1, 0) };
                ProcessorInfoAndFeatureBits {
                    processor_version_information: ProcessorVersionInformation(eax),
                    additional_information: unsafe { transmute::<_, AdditionalInformation>(ebx) },
                    feature_information: FeatureInformation {
                        ecx: L1S0Ecx { bits: ecx },
                        edx: L1S0Edx { bits: edx },
                    },
                }
            },
            thermal_and_power_management: {
                let CpuidResult {
                    eax,
                    ebx,
                    ecx,
                    edx: _,
                } = unsafe { __cpuid_count(6, 0) };
                ThermalAndPowerManagement {
                    features: ThermalAndPowerManagementFeatures {
                        eax: L6S0Eax { bits: eax },
                        ecx: L6S0Ecx { bits: ecx },
                    },
                    number_of_interrupt_thresholds: L6S0Ebx(ebx),
                }
            },
            extended_features: {
                let CpuidResult {
                    eax: _,
                    ebx: ebx0,
                    ecx: ecx0,
                    edx: edx0,
                } = unsafe { __cpuid_count(7, 0) };
                let CpuidResult {
                    eax: eax1,
                    ebx: _,
                    ecx: _,
                    edx: _,
                } = unsafe { __cpuid_count(7, 1) };
                ExtendedFeatures {
                    ebx: L7S0Ebx { bits: ebx0 },
                    ecx: L7S0Ecx { bits: ecx0 },
                    edx: L7S0Edx { bits: edx0 },
                    eax: L7S1Eax { bits: eax1 },
                }
            },
            cpuid_feature_bits1: {
                let CpuidResult {
                    eax,
                    ebx: _,
                    ecx: _,
                    edx: _,
                } = unsafe { __cpuid_count(13, 1) };
                L13S1Eax { bits: eax }
            },
            cpuid_feature_bits2: {
                let CpuidResult {
                    eax,
                    ebx: _,
                    ecx: _,
                    edx: _,
                } = unsafe { __cpuid_count(18, 0) };
                L18S0Eax { bits: eax }
            },
            cpuid_feature_bits3: {
                let CpuidResult {
                    eax: _,
                    ebx,
                    ecx: _,
                    edx: _,
                } = unsafe { __cpuid_count(20, 0) };
                L20S0Ebx { bits: ebx }
            },
            cpuid_feature_bits4: {
                let CpuidResult {
                    eax: _,
                    ebx,
                    ecx: _,
                    edx: _,
                } = unsafe { __cpuid_count(25, 0) };
                L25S0Ebx { bits: ebx }
            },
            extended_processor_info_and_feature_bits: {
                let CpuidResult {
                    eax: _,
                    ebx: _,
                    ecx,
                    edx,
                } = unsafe { __cpuid_count(0x80000001, 0) };
                ExtendedProcessorInfoAndFeatureBits {
                    edx: L1GBS0Edx { bits: edx },
                    ecx: L1GBS0Ecx { bits: ecx },
                }
            },
            virtual_and_physical_address_sizes: {
                let CpuidResult {
                    eax,
                    ebx,
                    ecx,
                    edx: _,
                } = unsafe { __cpuid_count(0x80000008, 0) };
                VirtualAndPhysicalAddressSizes {
                    eax: L0X80000008S0Eax(eax),
                    ebx: L0X80000008S0Ebx { bits: ebx },
                    ecx: L0X80000008S0Ecx(ecx),
                }
            },
            cpuid_feature_bits5: {
                let CpuidResult {
                    eax,
                    ebx: _,
                    ecx: _,
                    edx: _,
                } = unsafe { __cpuid_count(0x80000008, 0) };
                L0X80000001FS0Eax { bits: eax }
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
            .field("cpuid_feature_bits1", &self.cpuid_feature_bits1)
            .field("cpuid_feature_bits2", &self.cpuid_feature_bits2)
            .field("cpuid_feature_bits3", &self.cpuid_feature_bits3)
            .field("cpuid_feature_bits4", &self.cpuid_feature_bits4)
            .field(
                "extended_processor_info_and_feature_bits",
                &self.extended_processor_info_and_feature_bits,
            )
            .field(
                "virtual_and_physical_address_sizes",
                &self.virtual_and_physical_address_sizes,
            )
            .field("cpuid_feature_bits5", &self.cpuid_feature_bits5)
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
    ecx: L1S0Ecx,
    edx: L1S0Edx,
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
    number_of_interrupt_thresholds: L6S0Ebx,
}
#[repr(C)]
struct ThermalAndPowerManagementFeatures {
    eax: L6S0Eax,
    ecx: L6S0Ecx,
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
struct L6S0Ebx(u32);
impl L6S0Ebx {
    fn number_of_interrupt_thresholds(&self) -> u8 {
        (self.0 & 0b00001111) as u8
    }
}
impl fmt::Debug for L6S0Ebx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.number_of_interrupt_thresholds())
    }
}
#[repr(C)]
struct ExtendedFeatures {
    ebx: L7S0Ebx,
    ecx: L7S0Ecx,
    edx: L7S0Edx,
    eax: L7S1Eax,
}
impl fmt::Debug for ExtendedFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ebx = if !self.ebx.is_empty() {
            write!(f, "{:?}", self.ebx)?;
            true
        } else {
            false
        };
        let ecx = if !self.ecx.is_empty() {
            if ebx {
                write!(f, " | ")?;
            }
            write!(f, "{:?}", self.ecx)?;
            true
        } else {
            false
        };
        let edx = if !self.edx.is_empty() {
            if ecx {
                write!(f, " | ")?;
            }
            write!(f, "{:?}", self.edx)?;
            true
        } else {
            false
        };
        let _eax = if !self.eax.is_empty() {
            if edx {
                write!(f, " | ")?;
            }
            write!(f, "{:?}", self.eax)?;
            true
        } else {
            false
        };
        Ok(())
    }
}

#[repr(C)]
struct ExtendedProcessorInfoAndFeatureBits {
    edx: L1GBS0Edx,
    ecx: L1GBS0Ecx,
}
impl fmt::Debug for ExtendedProcessorInfoAndFeatureBits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.edx.is_empty(), self.ecx.is_empty()) {
            (true, true) => write!(f, "(empty)"),
            (false, true) => write!(f, "{:?}", self.edx),
            (true, false) => write!(f, "{:?}", self.ecx),
            (false, false) => write!(f, "{:?} | {:?}", self.edx, self.ecx),
        }
    }
}
#[repr(C)]
struct VirtualAndPhysicalAddressSizes {
    eax: L0X80000008S0Eax,
    ebx: L0X80000008S0Ebx,
    ecx: L0X80000008S0Ecx,
}
impl fmt::Debug for VirtualAndPhysicalAddressSizes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VirtualAndPhysicalAddressSizes")
            .field(
                "number_of_physical_address_bits",
                &self.eax.number_of_physical_address_bits(),
            )
            .field(
                "number_of_linear_address_bits",
                &self.eax.number_of_linear_address_bits(),
            )
            .field("features", &self.ebx)
            .field(
                "number_of_physical_cores_minus_1",
                &self.ecx.number_of_physical_cores_minus_1(),
            )
            .field(
                "log2_of_maximum_apic_id",
                &self.ecx.log2_of_maximum_apic_id(),
            )
            .field(
                "performance_timestamp_counter_size",
                &self.ecx.performance_timestamp_counter_size(),
            )
            .finish()
    }
}
#[repr(C)]
struct L0X80000008S0Eax(u32);
impl L0X80000008S0Eax {
    fn number_of_physical_address_bits(&self) -> u8 {
        ((self.0 & 0b00000000_00000000_00000000_11111111) >> 0) as u8
    }
    fn number_of_linear_address_bits(&self) -> u8 {
        ((self.0 & 0b00000000_00000000_11111111_00000000) >> 8) as u8
    }
    // 16th to 31st bits reserved
}
#[repr(C)]
struct L0X80000008S0Ecx(u32);
impl L0X80000008S0Ecx {
    fn number_of_physical_cores_minus_1(&self) -> u8 {
        ((self.0 & 0b00000000_00000000_00000000_11111111) >> 0) as u8
    }
    // 8th to 11th bits reserved
    fn log2_of_maximum_apic_id(&self) -> u8 {
        ((self.0 & 0b00000000_00000000_11110000_00000000) >> 12) as u8
    }
    fn performance_timestamp_counter_size(&self) -> u8 {
        ((self.0 & 0b00000000_00000011_00000000_00000000) >> 16) as u8
    }
    // 18th to 31st bits reserved
}

fn main() {
    assert_eq!(size_of::<Cpuid>(), 96);

    // We load cpuid
    let cpuid = Cpuid::new();
    println!("cpuid: {:#?}", cpuid);
    // We transmute/reinterpret-cast it to an array of bytes
    let bytes = unsafe { transmute::<_, [u8; 96]>(cpuid) };
    println!("bytes: {:?}", bytes);
    // We store these bytes in a binary file
    let mut file = File::create("cpuid-x86_64").unwrap();
    file.write_all(&bytes).unwrap();

    // // We get bytes at compile time
    // const cpuid_x86_64_bytes: &'static [u8; 56] = include_bytes!("../cpuid-x86_64");
    // // println!("cpuid_x86_64_bytes: {:?}", cpuid_x86_64_bytes);
    // // assert_eq!(size_of::<Cpuid>(), cpuid_x86_64_bytes.len());

    // // We define a constant exhaustive template based off these bytes
    // const cpuid_x86_64_template: Cpuid =
    //     unsafe { transmute::<[u8; 56], Cpuid>(*cpuid_x86_64_bytes) };
    // // Allowing us to package our cpuid template with our program with
    // println!("cpuid_x86_64_template: {:#?}", cpuid_x86_64_template);
}
