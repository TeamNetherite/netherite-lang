macro tar(init_mod=$im:path; $($name:ident ptr_width=$ptr_width:literal init=$init_fn:ident);*$(;)?) {
    pub enum Target {
        $(
        $name,
        )*
    }

    impl Target {
        pub fn ptr_width(&self) -> u32 {
            match self {
                $(Self::$name => $ptr_width,)*
            }
        }

        pub fn init(&self) {
            match self {
                $(Self::$name => $im::$init_fn(),)*
            }
        }
    }
}

tar! {
    init_mod=init;
    X86 ptr_width=8 init=init_x86;
}

mod init {
    const INIT: InitializationConfig = InitializationConfig {
        base: true,
        info: true,
        asm_printer: true,
        asm_parser: true,
        disassembler: false,
        machine_code: true,
    };

    use inkwell::targets::{InitializationConfig, Target as LLVMT};

    fn init_x86() {
        LLVMT::initialize_x86(&INIT);
    }
}
