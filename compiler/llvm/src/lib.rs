#![feature(default_free_fn)]
#![feature(decl_macro)]
#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used)]

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{AnyType, BasicType, BasicTypeEnum, FunctionType, IntType};
use std::default::default;
use topaz_ast::item::func::Func;
use topaz_ast::token::Arrow;
use topaz_ast::types::{PrimitiveType, Type};
use topaz_ast::visit::Visit;
use topaz_ast::Token;

pub mod target;

struct LLVMCodegen<'cx> {
    cx: Context,
    b: Builder<'cx>,
    m: Module<'cx>,
    target: target::Target
}

impl Visit for LLVMCodegen {
    fn visit_func(&mut self, func: &Func) {
        let func = self.m.add_function(
            func.2.value(),
            make_type(&func.4, &self.cx, self.target.ptr_width()),
            None,
        );

        self.cx.append_basic_block(func, func.2.value());

    }
}

fn make_type(ast: &Option<(Arrow, Type)>, cx: &Context, ptr_width: u32) -> impl AnyType {
    let ast = if let Some((_, ty)) = ast {
        ty
    } else {
        &Type::Primitive(PrimitiveType::Void)
    };

    match ast {
        Type::Primitive(primitive) => match primitive {
            PrimitiveType::Byte | PrimitiveType::Ubyte => cx.i8_type(),
            PrimitiveType::Short | PrimitiveType::Ushort => cx.i16_type(),
            PrimitiveType::Int | PrimitiveType::Uint => cx.i32_type(),
            PrimitiveType::Long | PrimitiveType::Ulong => cx.i64_type(),
            PrimitiveType::Explod | PrimitiveType::Uexplod => cx.i128_type(),
            PrimitiveType::String => todo!(),
            PrimitiveType::Void => cx.void_type(),
            PrimitiveType::Char => cx.i8_type(),
            PrimitiveType::Isize | PrimitiveType::Usize => cx.custom_width_int_type(ptr_width),
        },
        _ => todo!(),
    }
}

impl LLVMCodegen {
    fn new(mod_name: &str, target: target::Target) -> Self {
        let cx = Context::create();
        let m = cx.create_module(mod_name);
        let bld = cx.create_builder();
        Self {
            cx,
            m,
            b: bld,
            target
        }
    }

    fn init(&mut self) {
        self.target.init()
    }
}
