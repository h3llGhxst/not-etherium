mod vm;

use vm::VM;

fn main() {
    let bytecode = [0x61, 0x01, 0x02, 0x60, 0x00, 0x52, 0x60, 0x20, 0x60, 0x00, 0xF3];

    let mut vm = VM::new();
    match vm.execute(&bytecode, vec![]) {
        Ok(data)  => println!("{:?}", data.iter()
            .map(|b| format!("{:02x}",b)).collect::<Vec<_>>()),
        Err(e) => println!("err: {}", e),
    }


}
