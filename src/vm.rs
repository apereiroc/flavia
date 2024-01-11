pub struct VirtualMachine {
    registers: [i32; 32],
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine { registers: [0; 32] }
    }
}

#[test]
fn test_create_vm() {
    let vm = VirtualMachine::new();
    assert_eq!(vm.registers[0], 0);
}
