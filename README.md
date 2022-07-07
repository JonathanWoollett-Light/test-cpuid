# test-cpuid
```rust
let cpuid = Cpuid::new();
println!("cpuid: {:#?}", cpuid);
```
outputs:
```
cpuid: Cpuid {
    manufacturer_id: "AuthenticAMD",
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
            local_apic_id: 3,
        },
        feature_information: sse3 | pclmulqdq | ssse3 | fma | cx16 | sse4_1 | sse4_2 | movbe | popcnt | aes | xsave | osxsave | avx | f16c | rdrnd | hypervisor | fpu | vme | de | pse | tsc | msr | pae | mce | cx8 | apic | sep | mtrr | pge | mca | cmov | pat | pse_36 | clfsh | mmx | fxsr 
| sse | sse2 | htt,
    },
    thermal_and_power_management: ThermalAndPowerManagement {
        features: hardware_coordination_feedback_capability,
        number_of_interrupt_thresholds: 0,
    },
    extended_features: fsgsbase | bmi1 | avx2 | smep | bmi2 | pqdm | pqe | rdseed | adx | smap | clflushopt | clwb | sha | umip | rdpid,
}
```
