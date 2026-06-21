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

use crate::inst_translator::{insts::InstTranslator, utils::offset_to_wasm};
use kasl_ir::{IRType, Offset, Value};
use wasm_encoder::MemArg;

impl InstTranslator<'_> {
    pub(super) fn inst_load(
        &mut self,
        ty: &IRType,
        src_ptr: &Value,
        src_offset: &Offset,
        dst: &Value,
    ) {
        // 1: src_ptr
        self.wasm_func
            .instructions()
            .local_get(self.ctx.val_map[src_ptr]);

        let offset = offset_to_wasm(src_offset);
        let mem_arg = MemArg {
            offset,
            align: 0,
            memory_index: 0,
        };
        match ty {
            IRType::I8 => self.wasm_func.instructions().i32_load8_s(mem_arg),
            IRType::I16 => self.wasm_func.instructions().i32_load16_s(mem_arg),
            IRType::I32 => self.wasm_func.instructions().i32_load(mem_arg),
            IRType::I64 => self.wasm_func.instructions().i64_load(mem_arg),
            IRType::F32 => self.wasm_func.instructions().f32_load(mem_arg),
            IRType::F64 => self.wasm_func.instructions().f64_load(mem_arg),
            IRType::Void => self.wasm_func.instructions().i32_load(mem_arg),
            IRType::Ptr => self.wasm_func.instructions().i32_load(mem_arg),
        };
        self.wasm_func
            .instructions()
            .local_set(self.ctx.val_map[dst]);
    }
}
