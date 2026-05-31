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

use crate::inst_translator::{TranslationContext, offset_conversion::offset_to_wasm};
use kasl_ir::{IRType, Offset, Value};
use wasm_encoder::MemArg;

pub(super) fn inst_store(
    wasm_func: &mut wasm_encoder::Function,
    context: &TranslationContext,
    src: &Value,
    dst_ptr: &Value,
    dst_offset: &Offset,
) {
    // 1: dst_ptr, 2: src value
    wasm_func.instructions().local_get(context.val_map[dst_ptr]);
    wasm_func.instructions().local_get(context.val_map[src]);

    let ty = context.val_types[src];
    let offset = offset_to_wasm(dst_offset);
    let mem_arg = MemArg {
        offset,
        align: 0,
        memory_index: 0,
    };
    match ty {
        IRType::I8 => wasm_func.instructions().i32_store8(mem_arg),
        IRType::I16 => wasm_func.instructions().i32_store16(mem_arg),
        IRType::I32 => wasm_func.instructions().i32_store(mem_arg),
        IRType::I64 => wasm_func.instructions().i64_store(mem_arg),
        IRType::F32 => wasm_func.instructions().f32_store(mem_arg),
        IRType::F64 => wasm_func.instructions().f64_store(mem_arg),
        IRType::Void => wasm_func.instructions().i32_store(mem_arg),
        IRType::Ptr => wasm_func.instructions().i32_store(mem_arg),
    };
}
