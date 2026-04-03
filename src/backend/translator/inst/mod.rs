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

mod jump;

use crate::{WasmBackend, backend::TranslationCtx};
use kasl_ir::Inst;

impl WasmBackend {
    pub(super) fn translate_inst(
        &self,
        f: &mut wasm_encoder::Function,
        inst: &Inst,
        ctx: &mut TranslationCtx,
    ) {
        match inst {
            Inst::Jump { block, args } => self.translate_jump(f, block, &args, ctx),
            _ => (),
        }
    }
}
