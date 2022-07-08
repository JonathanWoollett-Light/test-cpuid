#![allow(non_upper_case_globals, dead_code, unused_imports)]
//! Example
//! ```
//! let cpuid = Cpuid::new();
//! let highest_calling_parameter = cpuid.highest_function_parameter_an_manufacturer_id.highest_calling_parameter;
//! let _register0_0_eax = cpuid.leaf::<0>().sub_leaf::<0>().eax();
//! let _register0_0_ebx = cpuid.leaf::<0>().sub_leaf::<0>().ebx();
//! let _register0_0_ecx = cpuid.leaf::<0>().sub_leaf::<0>().ecx();
//! let _register0_0_edx = cpuid.leaf::<0>().sub_leaf::<0>().edx();
//! ```

use bitflags::bitflags;
use core::arch::x86_64::{CpuidResult, __cpuid, __cpuid_count, __get_cpuid_max};
use log_derive::{logfn, logfn_inputs};
use std::{
    fmt,
    fs::File,
    io::Write,
    mem::{size_of, transmute},
    ops::Index,
    str,
};

// -----------------------------------------------------------------------------
// Bit flag definitions
// -----------------------------------------------------------------------------
// Leaf0x1SubLeaf0Ecx refers to the ecx value in leaf 1, sub-leaf 0 of cpuid.
#[rustfmt::skip]
bitflags! {
    // Feature Information
    #[repr(C)]
    pub struct Leaf0x1_SubLeaf0_Ecx: u32 {
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
    // Feature Information
    #[repr(C)]
    pub struct Leaf0x1_SubLeaf0_Edx: u32 {
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
    // Thermal and power management
    #[repr(C)]
    pub struct Leaf0x6_SubLeaf0_Eax: u32 {
        const digital_thermal_sensor_capability =           1 << 0;
        const intel_turbo_boost_technology_capability =     1 << 1;
        const always_running_apic_timer_capability =        1 << 2;
        // 4th bit reserved
        const power_limit_notification_capability =         1 << 4;
        const extended_clock_modulation_duty_capability =   1 << 5;
        const package_thermal_management_capability =       1 << 6;
        // 7th to 31st bits reserved
    }
    // Thermal and power management
    #[repr(C)]
    pub struct Leaf0x6_SubLeaf0_Ecx: u32 {
        const hardware_coordination_feedback_capability =   1 << 0;
        const acnt2_capability =                            1 << 1;
        // 2nd bit reserved
        const performance_energy_bias_capability =          1 << 3;
        // 4th to 31st bits reserved
    }
    // Extended Features
    #[repr(C)]
    pub struct Leaf0x7_SubLeaf0_Ebx: u32 {
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
    // Extended Features
    #[repr(C)]
    pub struct Leaf0x7_SubLeaf0_Ecx: u32 {
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
    // Extended Features
    #[repr(C)]
    pub struct Leaf0x7_SubLeaf0_Edx: u32 {
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
    pub struct Leaf0x7_SubLeaf1_Eax: u32 {
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
    /// https://en.wikipedia.org/wiki/CPUID#EAX=0Dh,_ECX=1
    #[repr(C)]
    pub struct Leaf0xD_SubLeaf1_Eax: u32 {
        const xsaveopt =    1 << 0;
        const xsavec =      1 << 1;
        const xgetbv_ecx1 = 1 << 2;
        const xss =         1 << 3;
        // 4th to 31st bits reserved.
    }
    /// https://en.wikipedia.org/wiki/CPUID#EAX=12h,_ECX=0:_SGX_Leaf_Functions
    #[repr(C)]
    pub struct Leaf0x12_SubLeaf0_Eax: u32 {
        const sgx1 = 1 << 0;
        const sgx2 = 1 << 1;
        // 2nd to 4th bits reserved.
        const oss = 1 << 5;
        const encls = 1 << 6;
        // 7th to 31st bits reserved.
    }
    /// https://en.wikipedia.org/wiki/CPUID#EAX=14h,_ECX=0
    #[repr(C)]
    pub struct Leaf0x14_SubLeaf0_Ebx: u32 {
        // 0 to 3rd bits reserved.
        const ptwrite = 1 << 4;
        // 5th to 31st bits reserved.
    }
    /// https://en.wikipedia.org/wiki/CPUID#EAX=19h
    #[repr(C)]
    pub struct Leaf0x19_SubLeaf0_Ebx: u32 {
        const aes_kle = 1 << 0;
        // 1st bit reserved.
        const aes_wide_kl = 1 << 2;
        // 3rd bit reserved.
        const kl_msrs = 1 << 4;
        // 5th to 31st bits reserved.
    }
    #[repr(C)]
    pub struct Leaf0x80000001_SubLeaf0_Edx: u32 {
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
    pub struct Leaf0x80000001_SubLeaf0_Ecx: u32 {
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
    pub struct Leaf0x80000008_SubLeaf0_Ebx: u32 {
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
    /// https://en.wikipedia.org/wiki/CPUID#EAX=8000001Fh
    #[repr(C)]
    pub struct Leaf0x8000001F_SubLeaf0_Eax: u32 {
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

// -----------------------------------------------------------------------------
// Bitflags sub leaf impls
// -----------------------------------------------------------------------------

impl Leaf0xD_SubLeaf1_Eax {
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl Leaf0x12_SubLeaf0_Eax {
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl Leaf0x14_SubLeaf0_Ebx {
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl Leaf0x19_SubLeaf0_Ebx {
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl Leaf0x8000001F_SubLeaf0_Eax {
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}

// -----------------------------------------------------------------------------
// Bit flag registers impls
// -----------------------------------------------------------------------------

impl Leaf0x7_SubLeaf1_Eax {
    pub fn eax(&self) -> u32 {
        self.bits()
    }
}
impl Leaf0xD_SubLeaf1_Eax {
    pub fn eax(&self) -> u32 {
        self.bits()
    }
}
impl Leaf0x12_SubLeaf0_Eax {
    pub fn eax(&self) -> u32 {
        self.bits()
    }
}
impl Leaf0x14_SubLeaf0_Ebx {
    pub fn ebx(&self) -> u32 {
        self.bits()
    }
}
impl Leaf0x19_SubLeaf0_Ebx {
    pub fn ebx(&self) -> u32 {
        self.bits()
    }
}
impl Leaf0x8000001F_SubLeaf0_Eax {
    pub fn eax(&self) -> u32 {
        self.bits()
    }
}

// -----------------------------------------------------------------------------
// Cpuid definition
// -----------------------------------------------------------------------------

/// https://en.wikipedia.org/wiki/CPUID
#[repr(C)]
pub struct Cpuid {
    /// leaf 0
    pub highest_function_parameter_an_manufacturer_id: HighestFunctionParameterAndManufacturerID,
    /// leaf 1
    pub process_info_and_feature_bits: ProcessorInfoAndFeatureBits,
    /// leaf 6
    pub thermal_and_power_management: ThermalAndPowerManagement,
    /// leaf 7
    pub extended_features: ExtendedFeatures,
    /// leaf 13 / 0x0D
    pub cpuid_feature_bits1: Leaf0xD_SubLeaf1_Eax,
    /// leaf 18 / 0x12h
    pub cpuid_feature_bits2: Leaf0x12_SubLeaf0_Eax,
    /// leaf 20 / 0x14h
    pub cpuid_feature_bits3: Leaf0x14_SubLeaf0_Ebx,
    /// leaf 25 / 0x19h
    pub cpuid_feature_bits4: Leaf0x19_SubLeaf0_Ebx,
    /// leaf 0x80000001
    pub extended_processor_info_and_feature_bits: ExtendedProcessorInfoAndFeatureBits,
    /// leaf 0x80000008
    pub virtual_and_physical_address_sizes: VirtualAndPhysicalAddressSizes,
    /// leaf 0x8000001F
    pub cpuid_feature_bits5: Leaf0x8000001F_SubLeaf0_Eax,
}
impl Cpuid {
    pub fn new() -> Self {
        Self {
            highest_function_parameter_an_manufacturer_id: {
                let CpuidResult { eax, ebx, ecx, edx } = unsafe { __cpuid(0) };
                let (ebx_bytes, edx_bytes, ecx_bytes) =
                    (ebx.to_ne_bytes(), edx.to_ne_bytes(), ecx.to_ne_bytes());
                HighestFunctionParameterAndManufacturerID {
                    manufacturer_id: [ebx_bytes, edx_bytes, ecx_bytes]
                        .concat()
                        .try_into()
                        .unwrap(),
                    highest_calling_parameter: eax,
                }
            },
            process_info_and_feature_bits: {
                let CpuidResult { eax, ebx, ecx, edx } = unsafe { __cpuid_count(1, 0) };
                ProcessorInfoAndFeatureBits {
                    processor_version_information: ProcessorVersionInformation(eax),
                    additional_information: unsafe { transmute::<_, AdditionalInformation>(ebx) },
                    feature_information: FeatureInformation {
                        ecx: Leaf0x1_SubLeaf0_Ecx { bits: ecx },
                        edx: Leaf0x1_SubLeaf0_Edx { bits: edx },
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
                        eax: Leaf0x6_SubLeaf0_Eax { bits: eax },
                        ecx: Leaf0x6_SubLeaf0_Ecx { bits: ecx },
                    },
                    number_of_interrupt_thresholds: Leaf6SubLeaf0Ebx(ebx),
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
                    sub_leaf0: ExtendedFeaturesSubLeaf0 {
                        ebx: Leaf0x7_SubLeaf0_Ebx { bits: ebx0 },
                        ecx: Leaf0x7_SubLeaf0_Ecx { bits: ecx0 },
                        edx: Leaf0x7_SubLeaf0_Edx { bits: edx0 },
                    },
                    sub_leaf1: Leaf0x7_SubLeaf1_Eax { bits: eax1 },
                }
            },
            cpuid_feature_bits1: {
                let CpuidResult {
                    eax,
                    ebx: _,
                    ecx: _,
                    edx: _,
                } = unsafe { __cpuid_count(13, 1) };
                Leaf0xD_SubLeaf1_Eax { bits: eax }
            },
            cpuid_feature_bits2: {
                let CpuidResult {
                    eax,
                    ebx: _,
                    ecx: _,
                    edx: _,
                } = unsafe { __cpuid_count(18, 0) };
                Leaf0x12_SubLeaf0_Eax { bits: eax }
            },
            cpuid_feature_bits3: {
                let CpuidResult {
                    eax: _,
                    ebx,
                    ecx: _,
                    edx: _,
                } = unsafe { __cpuid_count(20, 0) };
                Leaf0x14_SubLeaf0_Ebx { bits: ebx }
            },
            cpuid_feature_bits4: {
                let CpuidResult {
                    eax: _,
                    ebx,
                    ecx: _,
                    edx: _,
                } = unsafe { __cpuid_count(25, 0) };
                Leaf0x19_SubLeaf0_Ebx { bits: ebx }
            },
            extended_processor_info_and_feature_bits: {
                let CpuidResult {
                    eax: _,
                    ebx: _,
                    ecx,
                    edx,
                } = unsafe { __cpuid_count(0x80000001, 0) };
                ExtendedProcessorInfoAndFeatureBits {
                    edx: Leaf0x80000001_SubLeaf0_Edx { bits: edx },
                    ecx: Leaf0x80000001_SubLeaf0_Ecx { bits: ecx },
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
                    eax: Leaf0x80000008_SubLeaf0_Eax(eax),
                    ebx: Leaf0x80000008_SubLeaf0_Ebx { bits: ebx },
                    ecx: Leaf0x80000008_SubLeaf0_Ecx(ecx),
                }
            },
            cpuid_feature_bits5: {
                let CpuidResult {
                    eax,
                    ebx: _,
                    ecx: _,
                    edx: _,
                } = unsafe { __cpuid_count(0x80000008, 0) };
                Leaf0x8000001F_SubLeaf0_Eax { bits: eax }
            },
        }
    }
    // If the feature set of `self` covers the feature set of `other`.
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn covers(&self, other: &Self) -> bool {
        // We first check they have the same manufacturer
        self.highest_function_parameter_an_manufacturer_id
            .covers(&other.highest_function_parameter_an_manufacturer_id)
            && self
                .process_info_and_feature_bits
                .covers(&other.process_info_and_feature_bits)
            && self
                .thermal_and_power_management
                .covers(&other.thermal_and_power_management)
            && self.extended_features.covers(&other.extended_features)
            && self.cpuid_feature_bits1.contains(other.cpuid_feature_bits1)
            && self.cpuid_feature_bits2.contains(other.cpuid_feature_bits2)
            && self.cpuid_feature_bits3.contains(other.cpuid_feature_bits3)
            && self.cpuid_feature_bits4.contains(other.cpuid_feature_bits4)
            && self
                .extended_processor_info_and_feature_bits
                .covers(&other.extended_processor_info_and_feature_bits)
            && self
                .virtual_and_physical_address_sizes
                .covers(&other.virtual_and_physical_address_sizes)
            && self.cpuid_feature_bits5.contains(other.cpuid_feature_bits5)
    }
    pub fn leaf<const N: usize>(&self) -> &<Cpuid as Leaf<N>>::Output
    where
        Cpuid: Leaf<N>,
    {
        <Cpuid as Leaf<N>>::leaf(self)
    }
}

pub trait Leaf<const INDEX: usize> {
    type Output;
    fn leaf(&self) -> &Self::Output;
}
impl Leaf<0> for Cpuid {
    type Output = HighestFunctionParameterAndManufacturerID;
    fn leaf(&self) -> &Self::Output {
        &self.highest_function_parameter_an_manufacturer_id
    }
}
impl Leaf<1> for Cpuid {
    type Output = ProcessorInfoAndFeatureBits;
    fn leaf(&self) -> &Self::Output {
        &self.process_info_and_feature_bits
    }
}
impl Leaf<6> for Cpuid {
    type Output = ThermalAndPowerManagement;
    fn leaf(&self) -> &Self::Output {
        &self.thermal_and_power_management
    }
}
impl Leaf<7> for Cpuid {
    type Output = ExtendedFeatures;
    fn leaf(&self) -> &Self::Output {
        &self.extended_features
    }
}
impl Leaf<13> for Cpuid {
    type Output = Leaf0xD_SubLeaf1_Eax;
    fn leaf(&self) -> &Self::Output {
        &self.cpuid_feature_bits1
    }
}
impl Leaf<18> for Cpuid {
    type Output = Leaf0x12_SubLeaf0_Eax;
    fn leaf(&self) -> &Self::Output {
        &self.cpuid_feature_bits2
    }
}
impl Leaf<20> for Cpuid {
    type Output = Leaf0x14_SubLeaf0_Ebx;
    fn leaf(&self) -> &Self::Output {
        &self.cpuid_feature_bits3
    }
}
impl Leaf<25> for Cpuid {
    type Output = Leaf0x19_SubLeaf0_Ebx;
    fn leaf(&self) -> &Self::Output {
        &self.cpuid_feature_bits4
    }
}
impl Leaf<0x80000001> for Cpuid {
    type Output = ExtendedProcessorInfoAndFeatureBits;
    fn leaf(&self) -> &Self::Output {
        &self.extended_processor_info_and_feature_bits
    }
}
impl Leaf<0x80000008> for Cpuid {
    type Output = VirtualAndPhysicalAddressSizes;
    fn leaf(&self) -> &Self::Output {
        &self.virtual_and_physical_address_sizes
    }
}
impl Leaf<0x8000001F> for Cpuid {
    type Output = Leaf0x8000001F_SubLeaf0_Eax;
    fn leaf(&self) -> &Self::Output {
        &self.cpuid_feature_bits5
    }
}

pub trait SubLeaf<const INDEX: usize> {
    type Output;
    fn sub_leaf(&self) -> &Self::Output;
}
impl SubLeaf<0> for HighestFunctionParameterAndManufacturerID {
    type Output = Self;
    fn sub_leaf(&self) -> &Self::Output {
        &self
    }
}
impl SubLeaf<0> for ProcessorInfoAndFeatureBits {
    type Output = Self;
    fn sub_leaf(&self) -> &Self::Output {
        &self
    }
}
impl SubLeaf<0> for ThermalAndPowerManagement {
    type Output = Self;
    fn sub_leaf(&self) -> &Self::Output {
        &self
    }
}
impl SubLeaf<0> for ExtendedFeatures {
    type Output = ExtendedFeaturesSubLeaf0;
    fn sub_leaf(&self) -> &Self::Output {
        &self.sub_leaf0
    }
}
impl SubLeaf<1> for ExtendedFeatures {
    type Output = Leaf0x7_SubLeaf1_Eax;
    fn sub_leaf(&self) -> &Self::Output {
        &self.sub_leaf1
    }
}
impl SubLeaf<1> for Leaf0xD_SubLeaf1_Eax {
    type Output = Self;
    fn sub_leaf(&self) -> &Self::Output {
        &self
    }
}
impl SubLeaf<0> for Leaf0x12_SubLeaf0_Eax {
    type Output = Self;
    fn sub_leaf(&self) -> &Self::Output {
        &self
    }
}
impl SubLeaf<0> for Leaf0x14_SubLeaf0_Ebx {
    type Output = Self;
    fn sub_leaf(&self) -> &Self::Output {
        &self
    }
}
impl SubLeaf<0> for Leaf0x19_SubLeaf0_Ebx {
    type Output = Self;
    fn sub_leaf(&self) -> &Self::Output {
        &self
    }
}
impl SubLeaf<0> for ExtendedProcessorInfoAndFeatureBits {
    type Output = Self;
    fn sub_leaf(&self) -> &Self::Output {
        &self
    }
}
impl SubLeaf<0> for VirtualAndPhysicalAddressSizes {
    type Output = Self;
    fn sub_leaf(&self) -> &Self::Output {
        &self
    }
}
impl SubLeaf<0> for Leaf0x8000001F_SubLeaf0_Eax {
    type Output = Self;
    fn sub_leaf(&self) -> &Self::Output {
        &self
    }
}


impl Default for Cpuid {
    /// Sets the default CPU template based off the current machine
    fn default() -> Self {
        // let mut new = Cpuid::new();
        // new
        Cpuid::new()
    }
}
impl fmt::Debug for Cpuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cpuid")
            .field(
                "highest_function_parameter_an_manufacturer_id",
                &&self.highest_function_parameter_an_manufacturer_id,
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

/// https://en.wikipedia.org/wiki/CPUID#EAX=0:_Highest_Function_Parameter_and_Manufacturer_ID
#[repr(C)]
pub struct HighestFunctionParameterAndManufacturerID {
    pub manufacturer_id: [u8; 12],
    pub highest_calling_parameter: u32,
}
impl HighestFunctionParameterAndManufacturerID {
    pub fn ebx(&self) -> u32 {
        u32::from_ne_bytes(self.manufacturer_id[0..4].try_into().unwrap())
    }
    pub fn edx(&self) -> u32 {
        u32::from_ne_bytes(self.manufacturer_id[4..8].try_into().unwrap())
    }
    pub fn ecx(&self) -> u32 {
        u32::from_ne_bytes(self.manufacturer_id[8..12].try_into().unwrap())
    }
    pub fn eax(&self) -> u32 {
        self.highest_calling_parameter
    }
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.manufacturer_id == other.manufacturer_id
            && self.highest_calling_parameter >= other.highest_calling_parameter
    }
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl fmt::Debug for HighestFunctionParameterAndManufacturerID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HighestFunctionParameterAndManufacturerID")
            .field(
                "manufacturer_id",
                &str::from_utf8(&self.manufacturer_id).unwrap(),
            )
            .field("highest_calling_parameter", &self.highest_calling_parameter)
            .finish()
    }
}
/// https://en.wikipedia.org/wiki/CPUID#EAX=1:_Processor_Info_and_Feature_Bits
#[derive(Debug)]
#[repr(C)]
pub struct ProcessorInfoAndFeatureBits {
    pub processor_version_information: ProcessorVersionInformation,
    pub additional_information: AdditionalInformation,
    pub feature_information: FeatureInformation,
}
impl ProcessorInfoAndFeatureBits {
    pub fn ebx(&self) -> u32 {
        u32::from_ne_bytes([
            self.additional_information.brand_index,
            self.additional_information.clflush_line_size,
            self.additional_information.maximum_addressable_logical_processor_ids,
            self.additional_information.local_apic_id
        ])
    }
    pub fn edx(&self) -> u32 {
        self.feature_information.edx.bits
    }
    pub fn ecx(&self) -> u32 {
        self.feature_information.ecx.bits
    }
    pub fn eax(&self) -> u32 {
        self.processor_version_information.0
    }
    // If the feature set of `self` covers the feature set of `other`.
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.processor_version_information
            .covers(&other.processor_version_information)
            && self
                .additional_information
                .covers(&other.additional_information)
            && self.feature_information.covers(&other.feature_information)
    }
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
#[repr(C)]
pub struct ProcessorVersionInformation(u32);
impl ProcessorVersionInformation {
    fn stepping_id(&self) -> u8 {
        (self.0 & 0b0000_0000_0000_0000_0000_0000_0000_1111) as u8
    }
    fn model(&self) -> u8 {
        ((self.0 & 0b0000_0000_0000_0000_0000_0000_1111_0000) >> 4) as u8
    }
    fn family_id(&self) -> u8 {
        ((self.0 & 0b0000_0000_0000_0000_0000_1111_0000_0000) >> 8) as u8
    }
    fn processor_type(&self) -> u8 {
        ((self.0 & 0b0000_0000_0000_0000_0011_0000_0000_0000) >> 12) as u8
    }
    fn extended_model_id(&self) -> u8 {
        ((self.0 & 0b0000_0000_0000_1111_0000_0000_0000_0000) >> 16) as u8
    }
    fn extended_family_id(&self) -> u8 {
        ((self.0 & 0b0000_1111_1111_0000_0000_0000_0000_0000) >> 20) as u8
    }
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.0 == other.0
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
pub struct AdditionalInformation {
    pub brand_index: u8,
    pub clflush_line_size: u8,
    /// Local APIC ID
    pub maximum_addressable_logical_processor_ids: u8,
    pub local_apic_id: u8,
}
impl AdditionalInformation {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.brand_index == other.brand_index
            && self.clflush_line_size == other.clflush_line_size
            && self.maximum_addressable_logical_processor_ids
                >= other.maximum_addressable_logical_processor_ids
        // This value doesn't directly relate to available functionlity
        // && self.local_apic_id == other.local_apic_id
    }
}
#[repr(C)]
pub struct FeatureInformation {
    pub ecx: Leaf0x1_SubLeaf0_Ecx,
    pub edx: Leaf0x1_SubLeaf0_Edx,
}
impl FeatureInformation {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.ecx.contains(other.ecx) && self.edx.contains(other.edx)
    }
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
/// https://en.wikipedia.org/wiki/CPUID#EAX=6:_Thermal_and_power_management
#[derive(Debug)]
#[repr(C)]
pub struct ThermalAndPowerManagement {
    pub features: ThermalAndPowerManagementFeatures,
    pub number_of_interrupt_thresholds: Leaf6SubLeaf0Ebx,
}
impl ThermalAndPowerManagement {
    pub fn ebx(&self) -> u32 {
        self.number_of_interrupt_thresholds.0
    }
    pub fn ecx(&self) -> u32 {
        self.features.ecx.bits()
    }
    pub fn eax(&self) -> u32 {
        self.features.eax.bits()
    }
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.features.covers(&other.features)
            && self
                .number_of_interrupt_thresholds
                .covers(&other.number_of_interrupt_thresholds)
    }
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
#[repr(C)]
pub struct ThermalAndPowerManagementFeatures {
    pub eax: Leaf0x6_SubLeaf0_Eax,
    pub ecx: Leaf0x6_SubLeaf0_Ecx,
}
impl ThermalAndPowerManagementFeatures {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.eax.contains(other.eax) && self.ecx.contains(other.ecx)
    }
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
pub struct Leaf6SubLeaf0Ebx(u32);
impl Leaf6SubLeaf0Ebx {
    fn number_of_interrupt_thresholds(&self) -> u8 {
        (self.0 & 0b0000_1111) as u8
    }
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.number_of_interrupt_thresholds() >= other.number_of_interrupt_thresholds()
    }
}
impl fmt::Debug for Leaf6SubLeaf0Ebx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.number_of_interrupt_thresholds())
    }
}
/// https://en.wikipedia.org/wiki/CPUID#EAX=7,_ECX=0:_Extended_Features & https://en.wikipedia.org/wiki/CPUID#EAX=7,_ECX=1:_Extended_Features
#[repr(C)]
pub struct ExtendedFeatures {
    pub sub_leaf0: ExtendedFeaturesSubLeaf0,
    pub sub_leaf1: Leaf0x7_SubLeaf1_Eax,
}
impl ExtendedFeatures {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.sub_leaf0.covers(&other.sub_leaf0) && self.sub_leaf1.contains(other.sub_leaf1)
    }
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
#[derive(Debug)]
#[repr(C)]
pub struct ExtendedFeaturesSubLeaf0 {
    pub ebx: Leaf0x7_SubLeaf0_Ebx,
    pub ecx: Leaf0x7_SubLeaf0_Ecx,
    pub edx: Leaf0x7_SubLeaf0_Edx,
}
impl ExtendedFeaturesSubLeaf0 {
    pub fn ebx(&self) -> u32 {
        self.ebx.bits()
    }
    pub fn ecx(&self) -> u32 {
        self.ecx.bits()
    }
    pub fn edx(&self) -> u32 {
        self.edx.bits()
    }
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.ebx.contains(other.ebx) && self.ecx.contains(other.ecx) && self.edx.contains(other.edx)
    }
}
impl fmt::Debug for ExtendedFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ebx = if !self.sub_leaf0.ebx.is_empty() {
            write!(f, "{:?}", self.sub_leaf0.ebx)?;
            true
        } else {
            false
        };
        let ecx = if !self.sub_leaf0.ecx.is_empty() {
            if ebx {
                write!(f, " | ")?;
            }
            write!(f, "{:?}", self.sub_leaf0.ecx)?;
            true
        } else {
            false
        };
        let edx = if !self.sub_leaf0.edx.is_empty() {
            if ecx {
                write!(f, " | ")?;
            }
            write!(f, "{:?}", self.sub_leaf0.edx)?;
            true
        } else {
            false
        };
        let _eax = if !self.sub_leaf1.is_empty() {
            if edx {
                write!(f, " | ")?;
            }
            write!(f, "{:?}", self.sub_leaf1)?;
            true
        } else {
            false
        };
        Ok(())
    }
}
/// https://en.wikipedia.org/wiki/CPUID#EAX=80000001h:_Extended_Processor_Info_and_Feature_Bits
#[repr(C)]
pub struct ExtendedProcessorInfoAndFeatureBits {
    pub edx: Leaf0x80000001_SubLeaf0_Edx,
    pub ecx: Leaf0x80000001_SubLeaf0_Ecx,
}
impl ExtendedProcessorInfoAndFeatureBits {
    pub fn edx(&self) -> u32 {
        self.edx.bits()
    }
    pub fn ecx(&self) -> u32 {
        self.ecx.bits()
    }
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.edx.contains(other.edx) && self.ecx.contains(other.ecx)
    }
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
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
/// https://en.wikipedia.org/wiki/CPUID#EAX=80000008h:_Virtual_and_Physical_address_Sizes
#[repr(C)]
pub struct VirtualAndPhysicalAddressSizes {
    pub eax: Leaf0x80000008_SubLeaf0_Eax,
    pub ebx: Leaf0x80000008_SubLeaf0_Ebx,
    pub ecx: Leaf0x80000008_SubLeaf0_Ecx,
}
impl VirtualAndPhysicalAddressSizes {
    pub fn eax(&self) -> u32 {
        self.eax.0
    }
    pub fn ebx(&self) -> u32 {
        self.ebx.bits()
    }
    pub fn ecx(&self) -> u32 {
        self.ecx.0
    }
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.eax.covers(&other.eax) && self.ebx.contains(other.ebx) && self.ecx.covers(&other.ecx)
    }
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
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
#[derive(Debug)]
#[repr(C)]
pub struct Leaf0x80000008_SubLeaf0_Eax(u32);
impl Leaf0x80000008_SubLeaf0_Eax {
    fn number_of_physical_address_bits(&self) -> u8 {
        (self.0 & 0b0000_0000_0000_0000_0000_0000_1111_1111) as u8
    }
    fn number_of_linear_address_bits(&self) -> u8 {
        ((self.0 & 0b0000_0000_0000_0000_1111_1111_0000_0000) >> 8) as u8
    }
    // 16th to 31st bits reserved
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.number_of_physical_address_bits() >= other.number_of_physical_address_bits()
            && self.number_of_linear_address_bits() >= other.number_of_linear_address_bits()
    }
}
#[derive(Debug)]
#[repr(C)]
pub struct Leaf0x80000008_SubLeaf0_Ecx(u32);
impl Leaf0x80000008_SubLeaf0_Ecx {
    fn number_of_physical_cores_minus_1(&self) -> u8 {
        (self.0 & 0b0000_0000_0000_0000_0000_0000_1111_1111) as u8
    }
    // 8th to 11th bits reserved
    fn log2_of_maximum_apic_id(&self) -> u8 {
        ((self.0 & 0b0000_0000_0000_0000_1111_0000_0000_0000) >> 12) as u8
    }
    fn performance_timestamp_counter_size(&self) -> u8 {
        ((self.0 & 0b0000_0000_0000_0011_0000_0000_0000_0000) >> 16) as u8
    }
    // 18th to 31st bits reserved
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.number_of_physical_cores_minus_1() >= other.number_of_physical_cores_minus_1()
            && self.log2_of_maximum_apic_id() >= other.log2_of_maximum_apic_id()
            && self.performance_timestamp_counter_size()
                >= other.performance_timestamp_counter_size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_logger::SimpleLogger;
    #[test]
    fn print() {
        SimpleLogger::new().init().unwrap();
        let cpuid = Cpuid::new();
        println!("cpuid: {:#?}",cpuid);
    }
    #[test]
    fn save_load() {
        SimpleLogger::new().init().unwrap();

        assert_eq!(size_of::<Cpuid>(), 100);

        // We load cpuid
        let cpuid = Cpuid::new();
        // println!("cpuid: {:#?}", cpuid);

        // We transmute/reinterpret-cast it to an array of bytes
        let bytes = unsafe { transmute::<_, [u8; 100]>(cpuid) };
        println!("a: {:?}", bytes);

        // We store these bytes in a binary file
        let mut file = File::create("cpuid-x86_64").unwrap();
        file.write_all(&bytes).unwrap();

        // We get bytes at compile time
        const cpuid_x86_64_bytes: &'static [u8; 100] = include_bytes!("../cpuid-x86_64");
        println!("b: {:?}", cpuid_x86_64_bytes);

        // We define a constant exhaustive template based off these bytes
        const cpuid_x86_64_template: Cpuid =
            unsafe { transmute::<[u8; 100], Cpuid>(*cpuid_x86_64_bytes) };

        // Allowing us to package our cpuid template with our program with
        // println!("cpuid_x86_64_template: {:#?}", cpuid_x86_64_template);
        assert!(cpuid_x86_64_template.covers(&Cpuid::new()));
    }

    #[test]
    fn leaf_trait_index() {
        let cpuid = Cpuid::new();
        let _leaf0 = Leaf::<0>::leaf(&cpuid);
        let _leaf1 = Leaf::<1>::leaf(&cpuid);
        let _leaf6 = Leaf::<6>::leaf(&cpuid);
        let _leaf7 = Leaf::<7>::leaf(&cpuid);
        let _leaf13 = Leaf::<13>::leaf(&cpuid);
        let _leaf18 = Leaf::<18>::leaf(&cpuid);
        let _leaf20 = Leaf::<20>::leaf(&cpuid);
        let _leaf25 = Leaf::<25>::leaf(&cpuid);
        let _leaf0x80000001 = Leaf::<0x80000001>::leaf(&cpuid);
        let _leaf0x80000008 = Leaf::<0x80000008>::leaf(&cpuid);
        let _leaf0x8000001f = Leaf::<0x8000001F>::leaf(&cpuid);
    }
    #[test]
    fn leaf_fn_index() {
        let cpuid = Cpuid::new();
        let _leaf0 = cpuid.leaf::<0>();
        let _leaf1 = cpuid.leaf::<1>();
        let _leaf6 = cpuid.leaf::<6>();
        let _leaf7 = cpuid.leaf::<7>();
        let _leaf13 = cpuid.leaf::<13>();
        let _leaf18 = cpuid.leaf::<18>();
        let _leaf20 = cpuid.leaf::<20>();
        let _leaf25 = cpuid.leaf::<25>();
        let _leaf0x80000001 = cpuid.leaf::<0x80000001>();
        let _leaf0x80000008 = cpuid.leaf::<0x80000008>();
        let _leaf0x8000001f = cpuid.leaf::<0x8000001F>();
    }
    #[test]
    fn sub_leaf_fn_index() {
        let cpuid = Cpuid::new();
        let _sub_leaf0_0 = cpuid.leaf::<0>().sub_leaf::<0>();
        let _sub_leaf1_0 = cpuid.leaf::<1>().sub_leaf::<0>();
        let _sub_leaf6_0 = cpuid.leaf::<6>().sub_leaf::<0>();
        let _sub_leaf7_0 = cpuid.leaf::<7>().sub_leaf::<0>();
        let _sub_leaf7_1 = cpuid.leaf::<7>().sub_leaf::<1>();
        let _sub_leaf13_1 = cpuid.leaf::<13>().sub_leaf::<1>();
        let _sub_leaf18_0 = cpuid.leaf::<18>().sub_leaf::<0>();
        let _sub_leaf20_0 = cpuid.leaf::<20>().sub_leaf::<0>();
        let _sub_leaf0x80000001_0 = cpuid.leaf::<0x80000001>().sub_leaf::<0>();
        let _sub_leaf0x80000008_0 = cpuid.leaf::<0x80000008>().sub_leaf::<0>();
        let _sub_leaf0x8000001f_0 = cpuid.leaf::<0x8000001F>().sub_leaf::<0>();
    }
    #[test]
    fn registers_access() {
        let cpuid = Cpuid::new();

        let _register0_0_eax = cpuid.leaf::<0>().sub_leaf::<0>().eax();
        let _register0_0_ebx = cpuid.leaf::<0>().sub_leaf::<0>().ebx();
        let _register0_0_ecx = cpuid.leaf::<0>().sub_leaf::<0>().ecx();
        let _register0_0_edx = cpuid.leaf::<0>().sub_leaf::<0>().edx();

        let _register1_0_eax = cpuid.leaf::<1>().sub_leaf::<0>().eax();
        let _register1_0_ebx = cpuid.leaf::<1>().sub_leaf::<0>().ebx();
        let _register1_0_ecx = cpuid.leaf::<1>().sub_leaf::<0>().ecx();
        let _register1_0_edx = cpuid.leaf::<1>().sub_leaf::<0>().edx();


        let _register6_0_eax = cpuid.leaf::<6>().sub_leaf::<0>().eax();
        let _register6_0_ebx = cpuid.leaf::<6>().sub_leaf::<0>().ebx();
        let _register6_0_ecx = cpuid.leaf::<6>().sub_leaf::<0>().ecx();


        let _register7_0_ebx = cpuid.leaf::<7>().sub_leaf::<0>().ebx();
        let _register7_0_ecx = cpuid.leaf::<7>().sub_leaf::<0>().ecx();
        let _register7_0_edx = cpuid.leaf::<7>().sub_leaf::<0>().edx();

        let _register7_1_eax = cpuid.leaf::<7>().sub_leaf::<1>().eax();


        let _register_13_1_eax = cpuid.leaf::<13>().sub_leaf::<1>().eax();

        let _register18_0_eax = cpuid.leaf::<18>().sub_leaf::<0>().eax();

        let _register20_0_ebx = cpuid.leaf::<20>().sub_leaf::<0>().ebx();

        let _register0x80000001_0_ecx = cpuid.leaf::<0x80000001>().sub_leaf::<0>().ecx();
        let _register0x80000001_0_edx = cpuid.leaf::<0x80000001>().sub_leaf::<0>().edx();


        let _register0x80000008_0_eax = cpuid.leaf::<0x80000008>().sub_leaf::<0>().eax();
        let _register0x80000008_0_ebx = cpuid.leaf::<0x80000008>().sub_leaf::<0>().ebx();
        let _register0x80000008_0_ecx = cpuid.leaf::<0x80000008>().sub_leaf::<0>().ecx();

        let _register0x8000001f_0_eax = cpuid.leaf::<0x8000001F>().sub_leaf::<0>().eax();
    }
}
