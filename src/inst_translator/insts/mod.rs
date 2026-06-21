//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

mod alloc;
mod bin_op;
mod cmp;
mod cmp_imm;
mod const_val;
mod conversion;
mod jump;
mod load;
mod resize;
mod store;
mod unary_op;

use crate::inst_translator::{block_translator::InstTranslator, utils::offset_to_i32};
use kasl_ir::Inst;
use wasm_encoder::BlockType;

impl<'a> InstTranslator<'a> {
    /// Translates the instruction in the given block.
    pub(super) fn translate_inst(&mut self, inst: &Inst) {
        match inst {
            Inst::Alloc {
                size,
                align: _,
                dst,
            } => {
                self.inst_alloc(size, dst);
            }
            Inst::Load {
                ty,
                src_ptr,
                src_offset,
                dst,
            } => {
                self.inst_load(ty, src_ptr, src_offset, dst);
            }
            Inst::Store {
                src,
                dst_ptr,
                dst_offset,
            } => {
                self.inst_store(src, dst_ptr, dst_offset);
            }
            Inst::Memcpy {
                size,
                src_ptr,
                dst_ptr,
            } => {
                // 1: dst_ptr, 2: src_ptr, 3: size
                self.wasm_func
                    .instructions()
                    .local_get(self.ctx.val_map[dst_ptr]);
                self.wasm_func
                    .instructions()
                    .local_get(self.ctx.val_map[src_ptr]);
                self.wasm_func.instructions().i32_const(*size as i32);
                self.wasm_func.instructions().memory_copy(0, 0);
            }
            Inst::Memset {
                size,
                value,
                dst_ptr,
            } => {
                // 1: dst_ptr, 2: value, 3: size
                self.wasm_func
                    .instructions()
                    .local_get(self.ctx.val_map[dst_ptr]);
                self.wasm_func.instructions().i32_const(*value as i32);
                self.wasm_func.instructions().i32_const(*size as i32);
                self.wasm_func.instructions().memory_fill(0);
            }
            Inst::Const { value, dst } => {
                self.inst_const(value, dst);
            }
            Inst::Assign { var, src } => {
                // Load the value in the src local to the stack and store it in the destination local
                self.wasm_func
                    .instructions()
                    .local_get(self.ctx.val_map[src]);
                self.wasm_func
                    .instructions()
                    .local_set(self.ctx.var_map[var]);
            }
            Inst::LoadVar { var, dst } => {
                // Load the value in the source local to the stack and store it in the destination local
                self.wasm_func
                    .instructions()
                    .local_get(self.ctx.var_map[var]);
                self.wasm_func
                    .instructions()
                    .local_set(self.ctx.val_map[dst]);
            }
            Inst::Jump { block, args } => {
                self.inst_jump(block, args);
            }
            Inst::Brif {
                cond,
                then_block,
                then_args,
                else_block,
                else_args,
            } => {
                // Get the evaluated condition value and create the if instruction
                self.wasm_func
                    .instructions()
                    .local_get(self.ctx.val_map[cond]);
                // Jump to the then block if the condition is true
                self.wasm_func.instructions().if_(BlockType::Empty);
                self.inst_jump(then_block, then_args);
                // Jump to the else block if the condition is false
                self.wasm_func.instructions().else_();
                self.inst_jump(else_block, else_args);
                self.wasm_func.instructions().end();
            }
            Inst::Return { vals } => {
                for val in vals {
                    self.wasm_func
                        .instructions()
                        .local_get(self.ctx.val_map[val]);
                }
                self.wasm_func.instructions().return_();
            }
            Inst::Select {
                cond,
                then_val,
                else_val,
                dst,
            } => {
                // 1. then_val, 2. else_val, 3. cond
                self.wasm_func
                    .instructions()
                    .local_get(self.ctx.val_map[then_val]);
                self.wasm_func
                    .instructions()
                    .local_get(self.ctx.val_map[else_val]);
                self.wasm_func
                    .instructions()
                    .local_get(self.ctx.val_map[cond]);
                self.wasm_func.instructions().select();
                // Write the output to the destination local
                self.wasm_func
                    .instructions()
                    .local_set(self.ctx.val_map[dst]);
            }
            Inst::IResize { src, dst_ty, dst } => {
                self.inst_iresize(src, dst_ty, dst);
            }
            Inst::FResize { src, dst_ty, dst } => {
                self.inst_fresize(src, dst_ty, dst);
            }
            Inst::IToF { src, dst_ty, dst } => {
                self.inst_itof(src, dst_ty, dst);
            }
            Inst::FToI { src, dst_ty, dst } => {
                self.inst_ftoi(src, dst_ty, dst);
            }
            Inst::PtrAdd { ptr, offset, dst } => {
                self.wasm_func
                    .instructions()
                    .local_get(self.ctx.val_map[ptr]);
                self.wasm_func
                    .instructions()
                    .i32_const(offset_to_i32(offset));
                self.wasm_func.instructions().i32_add();
                self.wasm_func
                    .instructions()
                    .local_set(self.ctx.val_map[dst]);
            }
            Inst::IBinOp { op, lhs, rhs, dst } => {
                self.inst_ibop(op, lhs, rhs, dst);
            }
            Inst::FBinOp { op, lhs, rhs, dst } => {
                self.inst_fbop(op, lhs, rhs, dst);
            }
            Inst::IUnaryOp { op, operand, dst } => {
                self.inst_iuop(op, operand, dst);
            }
            Inst::FUnaryOp { op, operand, dst } => {
                self.inst_fuop(op, operand, dst);
            }
            Inst::ICmp { cmp, lhs, rhs, dst } => {
                self.inst_icmp(cmp, lhs, rhs, dst);
            }
            Inst::FCmp { cmp, lhs, rhs, dst } => {
                self.inst_fcmp(cmp, lhs, rhs, dst);
            }
            Inst::ICmpImm { cmp, lhs, rhs, dst } => {
                self.inst_icmp_imm(cmp, lhs, rhs, dst);
            }
        }
    }
}
