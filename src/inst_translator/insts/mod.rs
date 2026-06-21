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
mod const_val;
mod load;
mod memcpy;
mod memset;
mod store;

use crate::inst_translator::block_translator::InstTranslator;
use kasl_ir::Inst;

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
                self.inst_memcpy(size, src_ptr, dst_ptr);
            }
            Inst::Memset {
                size,
                value,
                dst_ptr,
            } => {
                self.inst_memset(size, value, dst_ptr);
            }
            Inst::Const { value, dst } => {
                self.inst_const(value, dst);
            }
            Inst::Assign { var, src } => {
                self.wasm_func
                    .instructions()
                    .local_get(self.ctx.val_map[src]);
                self.wasm_func
                    .instructions()
                    .local_set(self.ctx.var_map[var]);
            }
            Inst::LoadVar { var, dst } => {}
            Inst::Jump { block, args } => {}
            Inst::Brif {
                cond,
                then_block,
                then_args,
                else_block,
                else_args,
            } => {}
            Inst::Return { vals } => {}
            Inst::Select {
                cond,
                then_val,
                else_val,
                dst,
            } => {}
            Inst::IResize { src, dst_ty, dst } => {}
            Inst::FResize { src, dst_ty, dst } => {}
            Inst::IToF { src, dst_ty, dst } => {}
            Inst::FToI { src, dst_ty, dst } => {}
            Inst::PtrAdd { ptr, offset, dst } => {}
            Inst::IBinOp { op, lhs, rhs, dst } => {}
            Inst::FBinOp { op, lhs, rhs, dst } => {}
            Inst::IUnaryOp { op, operand, dst } => {}
            Inst::FUnaryOp { op, operand, dst } => {}
            Inst::ICmp { cmp, lhs, rhs, dst } => {}
            Inst::FCmp { cmp, lhs, rhs, dst } => {}
            Inst::ICmpImm { cmp, lhs, rhs, dst } => {}
        }
    }
}
