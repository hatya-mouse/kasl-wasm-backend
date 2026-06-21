use crate::inst_translator::{InstTranslator, convert_type};
use kasl_ir::{IRType, Value};
use wasm_encoder::ValType;

impl InstTranslator<'_> {
    pub(super) fn inst_iresize(&mut self, src: &Value, dst_ty: &IRType, dst: &Value) {
        // Convert the source and destination types to WebAssembly types
        let src_wasm_ty = convert_type(&self.ctx.val_types[src]);
        let dst_wasm_ty = convert_type(dst_ty);

        self.wasm_func
            .instructions()
            .local_get(self.ctx.val_map[src]);

        // Match on the source and destination types to determine the appropriate WebAssembly instruction
        match (src_wasm_ty, dst_wasm_ty) {
            (ValType::I32, ValType::I64) => {
                self.wasm_func.instructions().i64_extend_i32_s();
            }
            (ValType::I64, ValType::I32) => {
                self.wasm_func.instructions().i32_wrap_i64();
            }
            (ValType::I32, ValType::I32) => (),
            (ValType::I64, ValType::I64) => (),
            _ => panic!(
                "Unsupported type conversion from {:?} to {:?}",
                src_wasm_ty, dst_wasm_ty
            ),
        };

        self.wasm_func
            .instructions()
            .local_set(self.ctx.val_map[dst]);
    }

    pub(super) fn inst_fresize(&mut self, src: &Value, dst_ty: &IRType, dst: &Value) {
        // Convert the source and destination types to WebAssembly types
        let src_wasm_ty = convert_type(&self.ctx.val_types[src]);
        let dst_wasm_ty = convert_type(dst_ty);

        self.wasm_func
            .instructions()
            .local_get(self.ctx.val_map[src]);

        // Match on the source and destination types to determine the appropriate WebAssembly instruction
        match (src_wasm_ty, dst_wasm_ty) {
            (ValType::F32, ValType::F64) => {
                self.wasm_func.instructions().f64_promote_f32();
            }
            (ValType::F64, ValType::F32) => {
                self.wasm_func.instructions().f32_demote_f64();
            }
            (ValType::F32, ValType::F32) => (),
            (ValType::F64, ValType::F64) => (),
            _ => panic!(
                "Unsupported type conversion from {:?} to {:?}",
                src_wasm_ty, dst_wasm_ty
            ),
        }

        self.wasm_func
            .instructions()
            .local_set(self.ctx.val_map[dst]);
    }
}
