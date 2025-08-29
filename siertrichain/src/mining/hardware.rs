pub enum Hardware {
    Cpu,
    Gpu,
    Fpga,
}

pub fn detect_hardware() -> Vec<Hardware> {
    // In a real implementation, this would probe the system for available hardware.
    vec![Hardware::Cpu]
}
