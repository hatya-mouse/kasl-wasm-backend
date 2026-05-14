//
//  Copyright 2026 Shuntaro Kasatani
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

use crate::stackifier::GraphTranslator;
use kasl_ir::Block;
use wasm_encoder::BlockType;

/// Represents the kind of nest scope, used to track the currently open scopes in the stack while building the graph.
enum ScopeKind {
    Block(Block),
    Loop(Block),
}

impl GraphTranslator<'_> {
    pub(super) fn build_nests(&mut self, sorted_blocks: Vec<Block>) {
        // Classify edges and get the loop header blocks and the target blocks
        let (loop_headers, target_blocks) = self.classify_edges(&sorted_blocks);

        // Stack to keep track of the currently open scopes
        let mut scope_stack: Vec<ScopeKind> = Vec::new();
        // Loop through the sorted blocks and build a graph
        for block in &sorted_blocks {
            if loop_headers.contains(block) {
                // If the block is a loop header, add continue statement at the end of the block
                self.wasm_func.instructions().loop_(BlockType::Empty);
                scope_stack.push(ScopeKind::Loop(*block));
            } else if target_blocks.contains(block) {
                // If the block is a target of a back edge, add a block scope
                self.wasm_func.instructions().block(BlockType::Empty);
                scope_stack.push(ScopeKind::Block(*block));
            }
        }
    }
}
