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

mod inst;

use crate::backend::{TranslationCtx, WasmBackend, wasm_scope::WasmScope};
use kasl_ir::Block;
use wasm_encoder::BlockType;

impl WasmBackend {
    pub(super) fn translate_block(
        &self,
        f: &mut wasm_encoder::Function,
        block: Block,
        ctx: &mut TranslationCtx,
    ) {
        // If there's a incoming back edge to this block, create a new loop
        if ctx.back_edges.iter().any(|edge| edge.1 == block) {
            f.instructions().loop_(BlockType::Empty);
            // Push the current block to the scope stack
            ctx.scope_stack.push(WasmScope::Loop(block));
        }

        // Create a block if there's a forward predecessor
        if ctx.predecessors.get(&block).is_some_and(|preds| {
            preds
                .iter()
                .any(|pred| !ctx.back_edges.contains(&(*pred, block)))
        }) {
            f.instructions().block(BlockType::Empty);
            // Push the current block to the scope stack
            ctx.scope_stack.push(WasmScope::Block(block));
        }

        // Translate the instructions one by one
        if let Some(block_data) = ctx.func.get_block(&block) {
            for inst in block_data.get_insts() {
                self.translate_inst(f, inst, ctx);
            }
        }
    }
}
