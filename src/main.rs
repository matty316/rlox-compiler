use std::{env::args, process::exit, io, fs};

use vm::{VM, InterpretResult};

mod chunk;
mod debug;
mod vm;
mod compiler;
mod scanner;

fn main() {
    let mut vm = VM::new();

    if args().len() == 1 {
        repl(&mut vm);
    } else if args().len() == 2 {
        run_file(&args().collect::<Vec<_>>()[1], &mut vm);
    } else {
        eprintln!("Usage: rlox [path]");
        exit(64);
    }
}

fn repl(vm: &mut VM) {
    loop {
        println!("> ");

        let mut line = String::new();
        
        io::stdin()
            .read_line(&mut line)
            .unwrap();

        vm.interpret(&line);
    }
}

fn run_file(filename: &String, vm: &mut VM) {
    let source = fs::read_to_string(filename).unwrap();
    let result = vm.interpret(&source);

    if result == InterpretResult::CompileError { exit(65); }
    if result == InterpretResult::RuntimeError { exit(70); }
}
