use std::path::PathBuf;

// 导入lbafl的组件
use libafl::{
    bolts::{
        current_nanos,
    },
};

// qemu 导入
use libafl_qemu::{
    
};

mod grammar;

fn main() -> Result<(), Error>{
    // 更改当前工作路径到新路径
    std::env::set_current_dir(std::path::Path::new("../Server"));

    // 获取命令行参数
    let do_repro = std::env::args().find(|a| a == "--repro").is_some();

    // 引入基于 nautilus 的上下文
    let context = grammar::get_trackmania_context(10);

    if do_repro {

    }

    let args = vec![
        "qemu-i386".to_string(),
        
    ];

    let env:Vec<(String, String)> = Vec::new(); // ??
    let emu = Emulator::new(&args, &env);  // ??

    emu.set_breakpoint(XML_RPC_CALL);  // ??



    Ok(())
}