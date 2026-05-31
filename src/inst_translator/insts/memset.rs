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

use crate::inst_translator::TranslationContext;
use kasl_ir::Value;

pub(super) fn inst_memset(
    wasm_func: &mut wasm_encoder::Function,
    context: &TranslationContext,
    size: &u32,
    value: &u8,
    dst_ptr: &Value,
) {
    // 1: dst_ptr, 2: value, 3: size
    wasm_func.instructions().local_get(context.val_map[dst_ptr]);
    wasm_func.instructions().i32_const(*value as i32);
    wasm_func.instructions().i32_const(*size as i32);
    wasm_func.instructions().memory_fill(0);
}
