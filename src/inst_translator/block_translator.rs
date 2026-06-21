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
use kasl_ir::BlockData;

pub(super) struct InstTranslator<'a> {
    pub wasm_func: &'a mut wasm_encoder::Function,
    pub(in crate::inst_translator) ctx: &'a TranslationContext,
    pub(in crate::inst_translator) kasl_func: &'a kasl_ir::Function,
}

impl<'a> InstTranslator<'a> {
    /// Creates a new `InstTranslator` instance.
    pub(super) fn new(
        wasm_func: &'a mut wasm_encoder::Function,
        ctx: &'a TranslationContext,
        kasl_func: &'a kasl_ir::Function,
    ) -> Self {
        Self {
            wasm_func,
            ctx,
            kasl_func,
        }
    }

    /// Translates the given block.
    pub(super) fn translate_block(&mut self, block_data: &BlockData) {
        for inst in block_data.get_insts() {
            self.translate_inst(inst);
        }
    }
}
