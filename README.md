# test-cpuid

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
