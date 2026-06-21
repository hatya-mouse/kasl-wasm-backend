use crate::inst_translator::{InstTranslator, convert_type};
use kasl_ir::{IRType, Value};
use wasm_encoder::ValType;

impl InstTranslator<'_> {
    pub(super) fn inst_itof(&mut self, src: &Value, dst_ty: &IRType, dst: &Value) {
        // Convert the source and destination types to WebAssembly types
        let src_wasm_ty = convert_type(&self.ctx.val_types[src]);
        let dst_wasm_ty = convert_type(dst_ty);

        self.wasm_func
            .instructions()
            .local_get(self.ctx.val_map[src]);

        match (src_wasm_ty, dst_wasm_ty) {
            (ValType::I32, ValType::F32) => {
                self.wasm_func.instructions().f32_convert_i32_s();
            }
            (ValType::I64, ValType::F32) => {
                self.wasm_func.instructions().f32_convert_i64_s();
            }
            (ValType::I32, ValType::F64) => {
                self.wasm_func.instructions().f64_convert_i32_s();
            }
            (ValType::I64, ValType::F64) => {
                self.wasm_func.instructions().f64_convert_i64_s();
            }
            _ => panic!(
                "Unsupported type conversion from {:?} to {:?}",
                src_wasm_ty, dst_wasm_ty
            ),
        }

        self.wasm_func
            .instructions()
            .local_set(self.ctx.val_map[dst]);
    }

    pub(super) fn inst_ftoi(&mut self, src: &Value, dst_ty: &IRType, dst: &Value) {
        // Convert the source and destination types to WebAssembly types
        let src_wasm_ty = convert_type(&self.ctx.val_types[src]);
        let dst_wasm_ty = convert_type(dst_ty);

        self.wasm_func
            .instructions()
            .local_get(self.ctx.val_map[src]);

        match (src_wasm_ty, dst_wasm_ty) {
            (ValType::F32, ValType::I32) => {
                self.wasm_func.instructions().i32_trunc_f32_s();
            }
            (ValType::F64, ValType::I32) => {
                self.wasm_func.instructions().i32_trunc_f64_s();
            }
            (ValType::F32, ValType::I64) => {
                self.wasm_func.instructions().i64_trunc_f32_s();
            }
            (ValType::F64, ValType::I64) => {
                self.wasm_func.instructions().i64_trunc_f64_s();
            }
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
