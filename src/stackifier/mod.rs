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

mod block_nesting;
mod edge_classify;

use std::collections::HashMap;

/// A struct to translate a function into Structured Control Flow using the Stackifier algorithm.
pub struct GraphTranslator<'a> {
    wasm_func: &'a mut wasm_encoder::Function,
    ir_func: kasl_ir::Function,
}

impl<'a> GraphTranslator<'a> {
    /// Creates a new Stackifier instance.
    pub fn new(wasm_func: &'a mut wasm_encoder::Function, ir_func: kasl_ir::Function) -> Self {
        Self { wasm_func, ir_func }
    }

    /// Translates the function into Structured Control Flow.
    pub fn translate(&mut self) {
        // Sort the blocks in reverse post-order
        let sorted_blocks = self.ir_func.sorted_blocks();
    }
}
