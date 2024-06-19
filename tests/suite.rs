use std::{fs::{self, read_dir, File, ReadDir}, path::Path};

use phoenix::VM;
use serde::Deserialize;
use serde_json::Error;

#[derive(Deserialize)]
struct TestCase {
	name: String,
	initial: State,
	r#final: State,
	// length: usize
}


#[derive(Deserialize)]
struct State {
	d0: u32,
	d1: u32,
	d2: u32,
	d3: u32,
	d4: u32,
	d5: u32,
	d6: u32,
	d7: u32,
	a0: u32,
	a1: u32,
	a2: u32,
	a3: u32,
	a4: u32,
	a5: u32,
	a6: u32,		
	usp: u32,
	ssp: u32,
	sr: u32,
	pc: u32,
	// prefetch: Vec<u32>,
	ram: Vec<(u32, u32)>
}

fn read_json(file: &Path) -> Result<Vec<TestCase>, Error> {
	let f = File::open(file).unwrap();
	serde_json::from_reader(f)	
}

fn run_case(test: TestCase) -> Result<usize, usize> {
	let mut vm = VM::new();
	let (inst, _) = test.name.split_at(4);
	init_vm(&mut vm, test.initial, inst);
	vm.step();
	check_vm_state(&vm, test.r#final);
	Ok(0)
}

fn init_vm(vm: &mut VM, state: State, inst: &str) {
	vm.cpu.write_ar(0, state.a0);
	vm.cpu.write_ar(1, state.a1);
	vm.cpu.write_ar(2, state.a2);
	vm.cpu.write_ar(3, state.a3);
	vm.cpu.write_ar(4, state.a4);
	vm.cpu.write_ar(5, state.a5);
	vm.cpu.write_ar(6, state.a6);
	vm.cpu.write_dr_long(0, state.d0);
	vm.cpu.write_dr_long(1, state.d1);
	vm.cpu.write_dr_long(2, state.d2);
	vm.cpu.write_dr_long(3, state.d3);
	vm.cpu.write_dr_long(4, state.d4);
	vm.cpu.write_dr_long(5, state.d5);
	vm.cpu.write_dr_long(6, state.d6);
	vm.cpu.write_dr_long(7, state.d7);
	vm.cpu.write_usp(state.usp);
	vm.cpu.write_ssp(state.ssp);
	vm.cpu.write_sr(state.sr as u16);
	vm.cpu.write_pc(state.pc);
	for (addr, val) in state.ram {
		vm.cpu.mmu.write_byte(addr, val as u8);
	}
	let (a,b) = inst.split_at(2);
	let a = u8::from_str_radix(a, 16).unwrap();	
	let b = u8::from_str_radix(b, 16).unwrap();
	vm.cpu.mmu.write_byte(state.pc, a);
	vm.cpu.mmu.write_byte(state.pc + 1, b);

} 

fn check_vm_state(vm: &VM, state: State) {
	assert_eq!(vm.cpu.read_ar(0), state.a0);
	assert_eq!(vm.cpu.read_ar(1), state.a1);
	assert_eq!(vm.cpu.read_ar(2), state.a2);
	assert_eq!(vm.cpu.read_ar(3), state.a3);
	assert_eq!(vm.cpu.read_ar(4), state.a4);
	assert_eq!(vm.cpu.read_ar(5), state.a5);
	assert_eq!(vm.cpu.read_ar(6), state.a6);
	assert_eq!(vm.cpu.read_dr(0), state.d0);
	assert_eq!(vm.cpu.read_dr(1), state.d1);
	assert_eq!(vm.cpu.read_dr(2), state.d2);
	assert_eq!(vm.cpu.read_dr(3), state.d3);
	assert_eq!(vm.cpu.read_dr(4), state.d4);
	assert_eq!(vm.cpu.read_dr(5), state.d5);
	assert_eq!(vm.cpu.read_dr(6), state.d6);	
	assert_eq!(vm.cpu.read_dr(7), state.d7);
	assert_eq!(vm.cpu.read_usp(), state.usp);
	assert_eq!(vm.cpu.read_ssp(), state.ssp);
	assert_eq!(vm.cpu.read_sr(), state.sr as u16);
	assert_eq!(vm.cpu.read_pc(), state.pc);
	for (addr, val) in state.ram {
		assert_eq!(val, vm.cpu.mmu.read_long(addr));
	}
}

#[test]
fn all() {
	let mut contents = vec![];
	for entry in read_dir("tests/json/").unwrap().take(5) {
		let path = entry.unwrap().path();
		if path.extension().unwrap_or_default() == "json" {
			contents.push(path);
		}
	}
	for inst in contents.iter().skip(1) {
		println!("{inst:?}");
		let op = read_json(inst).unwrap();
		for test in op {
			let res = run_case(test);
			println!("{res:?}");
			panic!()
		}
	}
}