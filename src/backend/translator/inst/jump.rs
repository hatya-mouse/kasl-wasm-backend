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

use crate::{
    WasmBackend,
    backend::{TranslationCtx, wasm_scope::WasmScope},
};
use kasl_ir::{Block, Value};

impl WasmBackend {
    pub(super) fn translate_jump(
        &self,
        f: &mut wasm_encoder::Function,
        block: &Block,
        args: &[Value],
        ctx: &mut TranslationCtx,
    ) {
        if let Some(br_index) = if ctx.back_edges.iter().any(|edge| &edge.1 == block) {
            // If the jump target is in the back edges, get the loop index
            ctx.scope_stack
                .iter()
                .position(|scope| scope == &WasmScope::Loop(*block))
        } else {
            // Else get the index of the block to jump to
            ctx.scope_stack
                .iter()
                .position(|scope| scope == &WasmScope::Block(*block))
        } {
            // Calculate the index to branch to
            let depth = ctx.scope_stack.len() - 1 - br_index;
            // Branch to the depth
            f.instructions().br(depth as u32);
        }
    }
}
