use crate::inst_translator::{InstTranslator, convert_type};
use kasl_ir::{FloatBinOp, IntBinOp, Value};
use wasm_encoder::ValType;

impl InstTranslator<'_> {
    pub(super) fn inst_ibop(&mut self, op: &IntBinOp, lhs: &Value, rhs: &Value, dst: &Value) {
        let wasm_ty = convert_type(&self.ctx.val_types[lhs]);
        self.wasm_func
            .instructions()
            .local_get(self.ctx.val_map[lhs]);
        self.wasm_func
            .instructions()
            .local_get(self.ctx.val_map[rhs]);

        match wasm_ty {
            ValType::I32 => match op {
                IntBinOp::Add => {
                    self.wasm_func.instructions().i32_add();
                }
                IntBinOp::Sub => {
                    self.wasm_func.instructions().i32_sub();
                }
                IntBinOp::Mul => {
                    self.wasm_func.instructions().i32_mul();
                }
                IntBinOp::Div => {
                    self.wasm_func.instructions().i32_div_s();
                }
                IntBinOp::SRem => {
                    self.wasm_func.instructions().i32_rem_s();
                }
                IntBinOp::IShL => {
                    self.wasm_func.instructions().i32_shl();
                }
                IntBinOp::SShR => {
                    self.wasm_func.instructions().i32_shr_s();
                }
                IntBinOp::UShR => {
                    self.wasm_func.instructions().i32_shr_u();
                }
                IntBinOp::Min => {
                    // Push two more values to the stack and call the min function
                    self.wasm_func.instructions().i32_const(i32::MAX);
                    self.wasm_func.instructions().i32_const(i32::MAX);
                    self.wasm_func.instructions().i32x4_min_s();
                }
                IntBinOp::Max => {
                    // Push two more values to the stack and call the max function
                    self.wasm_func.instructions().i32_const(i32::MIN);
                    self.wasm_func.instructions().i32_const(i32::MIN);
                    self.wasm_func.instructions().i32x4_max_s();
                }
                IntBinOp::BAnd => {
                    self.wasm_func.instructions().i32_and();
                }
                IntBinOp::BOr => {
                    self.wasm_func.instructions().i32_or();
                }
                IntBinOp::BXor => {
                    self.wasm_func.instructions().i32_xor();
                }
                IntBinOp::BNand => {
                    self.wasm_func.instructions().i32_and();
                    self.wasm_func.instructions().i32_const(-1);
                    self.wasm_func.instructions().i32_xor();
                }
                IntBinOp::BNor => {
                    self.wasm_func.instructions().i32_or();
                    self.wasm_func.instructions().i32_const(-1);
                    self.wasm_func.instructions().i32_xor();
                }
                IntBinOp::BXnor => {
                    self.wasm_func.instructions().i32_xor();
                    self.wasm_func.instructions().i32_const(-1);
                    self.wasm_func.instructions().i32_xor();
                }
            },
            ValType::I64 => match op {
                IntBinOp::Add => {
                    self.wasm_func.instructions().i64_add();
                }
                IntBinOp::Sub => {
                    self.wasm_func.instructions().i64_sub();
                }
                IntBinOp::Mul => {
                    self.wasm_func.instructions().i64_mul();
                }
                IntBinOp::Div => {
                    self.wasm_func.instructions().i64_div_s();
                }
                IntBinOp::SRem => {
                    self.wasm_func.instructions().i64_rem_s();
                }
                IntBinOp::IShL => {
                    self.wasm_func.instructions().i64_shl();
                }
                IntBinOp::SShR => {
                    self.wasm_func.instructions().i64_shr_s();
                }
                IntBinOp::UShR => {
                    self.wasm_func.instructions().i64_shr_u();
                }
                IntBinOp::Min => {
                    // Push lhs and rhs to the stack and compare them, then select the smaller one
                    self.wasm_func
                        .instructions()
                        .local_get(self.ctx.val_map[lhs]);
                    self.wasm_func
                        .instructions()
                        .local_get(self.ctx.val_map[rhs]);
                    self.wasm_func.instructions().i64_lt_s();
                    self.wasm_func.instructions().select();
                }
                IntBinOp::Max => {
                    // Push lhs and rhs to the stack and compare them, then select the greater one
                    self.wasm_func
                        .instructions()
                        .local_get(self.ctx.val_map[lhs]);
                    self.wasm_func
                        .instructions()
                        .local_get(self.ctx.val_map[rhs]);
                    self.wasm_func.instructions().i64_gt_s();
                    self.wasm_func.instructions().select();
                }
                IntBinOp::BAnd => {
                    self.wasm_func.instructions().i64_and();
                }
                IntBinOp::BOr => {
                    self.wasm_func.instructions().i64_or();
                }
                IntBinOp::BXor => {
                    self.wasm_func.instructions().i64_xor();
                }
                IntBinOp::BNand => {
                    self.wasm_func.instructions().i64_and();
                    self.wasm_func.instructions().i64_const(-1);
                    self.wasm_func.instructions().i64_xor();
                }
                IntBinOp::BNor => {
                    self.wasm_func.instructions().i64_or();
                    self.wasm_func.instructions().i64_const(-1);
                    self.wasm_func.instructions().i64_xor();
                }
                IntBinOp::BXnor => {
                    self.wasm_func.instructions().i64_xor();
                    self.wasm_func.instructions().i64_const(-1);
                    self.wasm_func.instructions().i64_xor();
                }
            },
            _ => panic!("Invalid type for integer binary operation"),
        }

        self.wasm_func
            .instructions()
            .local_set(self.ctx.val_map[dst]);
    }

    pub(super) fn inst_fbop(&mut self, op: &FloatBinOp, lhs: &Value, rhs: &Value, dst: &Value) {
        let wasm_ty = convert_type(&self.ctx.val_types[lhs]);
        self.wasm_func
            .instructions()
            .local_get(self.ctx.val_map[lhs]);
        self.wasm_func
            .instructions()
            .local_get(self.ctx.val_map[rhs]);

        match wasm_ty {
            ValType::F32 => match op {
                FloatBinOp::Add => {
                    self.wasm_func.instructions().f32_add();
                }
                FloatBinOp::Sub => {
                    self.wasm_func.instructions().f32_sub();
                }
                FloatBinOp::Mul => {
                    self.wasm_func.instructions().f32_mul();
                }
                FloatBinOp::Div => {
                    self.wasm_func.instructions().f32_div();
                }
                FloatBinOp::Rem => {
                    // Calculate the remainder using the formula: a - b * floor(a / b)
                    self.wasm_func
                        .instructions()
                        .local_get(self.ctx.val_map[lhs]);
                    self.wasm_func
                        .instructions()
                        .local_get(self.ctx.val_map[rhs]);
                    self.wasm_func.instructions().f32_div();
                    self.wasm_func.instructions().f32_trunc();
                    self.wasm_func.instructions().f32_mul();
                    self.wasm_func.instructions().f32_sub();
                }
                FloatBinOp::Pow => {}
            },
        }
    }
}
