use crate::inst_translator::insts::InstTranslator;
use kasl_ir::{Const, Value};
use wasm_encoder::{Ieee32, Ieee64};

impl InstTranslator<'_> {
    pub(super) fn inst_const(&mut self, value: &Const, dst: &Value) {
        match value {
            Const::I8(val) => self.wasm_func.instructions().i32_const(*val as i32),
            Const::I16(val) => self.wasm_func.instructions().i32_const(*val as i32),
            Const::I32(val) => self.wasm_func.instructions().i32_const(*val),
            Const::I64(val) => self.wasm_func.instructions().i64_const(*val),
            Const::F32(val) => self
                .wasm_func
                .instructions()
                .f32_const(Ieee32::new(val.to_bits())),
            Const::F64(val) => self
                .wasm_func
                .instructions()
                .f64_const(Ieee64::new(val.to_bits())),
            Const::Ptr(val) => self.wasm_func.instructions().i32_const(*val as i32),
        };
        self.wasm_func
            .instructions()
            .local_set(self.ctx.val_map[dst]);
    }
}
