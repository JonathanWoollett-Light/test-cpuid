# test-cpuid

Printing
```rust
#[test]
fn print() {
    SimpleLogger::new().init().unwrap();
    let cpuid = Cpuid::new();
    println!("cpuid: {:#?}",cpuid);
}
```
```
cpuid: Cpuid {
    highest_function_parameter_an_manufacturer_id: HighestFunctionParameterAndManufacturerID {
        manufacturer_id: "AuthenticAMD",
        highest_calling_parameter: 13,
    },
    process_info_and_feature_bits: ProcessorInfoAndFeatureBits {
        processor_version_information: ProcessorVersionInformation {
            stepping_id: 1,
            model: 0,
            family_id: 15,
            processor_type: 0,
            extended_model_id: 6,
            extended_family_id: 8,
        },
        additional_information: AdditionalInformation {
            brand_index: 0,
            clflush_line_size: 8,
            maximum_addressable_logical_processor_ids: 12,
            local_apic_id: 0,
        },
        feature_information: sse3 | pclmulqdq | ssse3 | fma | cx16 | sse4_1 | sse4_2 | movbe | popcnt | aes | xsave | osxsave | avx | f16c | rdrnd | hypervisor | fpu | vme | de | pse | tsc | msr | pae | mce | cx8 | apic | sep | mtrr | pge | mca | cmov | pat | pse_36 | clfsh | mmx | fxsr | sse | sse2 | htt,
    },
    thermal_and_power_management: ThermalAndPowerManagement {
        features: hardware_coordination_feedback_capability,
        number_of_interrupt_thresholds: 0,
    },
    extended_features: fsgsbase | bmi1 | avx2 | smep | bmi2 | pqdm | pqe | rdseed | adx | smap | clflushopt | clwb | sha | umip | rdpid,
    cpuid_feature_bitSubLeaf0x1: xsaveopt | xsavec | xgetbv_ecx1 | xss,
    cpuid_feature_bits2: (empty),
    cpuid_feature_bits3: (empty),
    cpuid_feature_bits4: (empty),
    extended_processor_info_and_feature_bits: syscall | nx | mmxext | fxsr_opt | pdpe1gb | rdtscp | lm | 0x183f3ff | lahf | cmp_legacy | cr8_legacy | abm | sse4a | missalignsse | _3dnowprefetch | osvw | wdt | topoext,
    virtual_and_physical_address_sizes: VirtualAndPhysicalAddressSizes {
        number_of_physical_address_bits: 48,
        number_of_linear_address_bits: 48,
        features: clzero | xrstor_fp_err | ibpb | ibrs | single_thread_ibp | virt_ssbd | 0x0x8000000,
        number_of_physical_cores_minus_1: 11,
        log2_of_maximum_apic_id: 7,
        performance_timestamp_counter_size: 0,
    },
    cpuid_feature_bits5: sev_snp | vmpl | restricted_injection | alternative_injection,
}
```

Saving and loading cpuid.
```rust
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
```
Directly accessing registers
```rust
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
```
