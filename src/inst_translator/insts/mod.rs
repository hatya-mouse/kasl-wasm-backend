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
mod load;
mod store;

use std::iter::empty;

use crate::inst_translator::{
    TranslationContext,
    insts::{load::inst_load, store::inst_store},
};
use alloc::inst_alloc;
use kasl_ir::Inst;

/// Translates the instruction in the given block.
pub(super) fn translate_inst(
    wasm_func: &mut wasm_encoder::Function,
    context: &TranslationContext,
    inst: &Inst,
) {
    match inst {
        Inst::Alloc {
            size,
            align: _,
            dst,
        } => {
            inst_alloc(wasm_func, context, size, dst);
        }
        Inst::Load {
            ty,
            src_ptr,
            src_offset,
            dst,
        } => {
            inst_load(wasm_func, context, ty, src_ptr, src_offset, dst);
        }
        Inst::Store {
            src,
            dst_ptr,
            dst_offset,
        } => {
            inst_store(wasm_func, context, src, dst_ptr, dst_offset);
        }
        _ => (),
    }
}
