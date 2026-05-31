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

use crate::inst_translator::{TranslationContext, insts::translate_inst};
use kasl_ir::BlockData;

/// Translates the given block.
pub(super) fn translate_block(
    wasm_func: &mut wasm_encoder::Function,
    context: &TranslationContext,
    block_data: &BlockData,
) {
    for inst in block_data.get_insts() {
        translate_inst(wasm_func, context, inst);
    }
}
