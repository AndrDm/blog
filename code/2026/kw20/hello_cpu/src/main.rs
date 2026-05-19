use raw_cpuid::CpuId;

fn main() {
    let cpuid = CpuId::new();

    if let Some(vf) = cpuid.get_processor_brand_string() {
        let s = vf.as_str()
            .replace("(R)", "®")
            .replace("(TM)", "™");
        println!("CPU: {}", s);
    }
}

/*
fn main() {
    let cpuid = CpuId::new();

    if let Some(vf) = cpuid.get_processor_brand_string() {
        println!("{:?}", vf);
    }
}
*/
