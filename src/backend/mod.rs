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

mod ctx;
mod translator;
mod wasm_scope;

use crate::backend::ctx::TranslationCtx;
use kasl_ir::Block;
use std::collections::{HashMap, HashSet};
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, FunctionSection, Module, TypeSection, ValType,
};

pub struct WasmBackend;

impl Default for WasmBackend {
    fn default() -> Self {
        Self {}
    }
}

impl WasmBackend {
    /// Translates the function to cranelift IR.
    pub fn compile(&mut self, func: kasl_ir::Function) {
        let mut module = Module::new();

        // Encode the type section
        let mut types = TypeSection::new();
        let params = vec![ValType::I32, ValType::I32];
        let results = vec![ValType::I32];
        types.ty().function(params, results);
        module.section(&types);

        // Encode the function section
        let mut functions = FunctionSection::new();
        let type_index = 0;
        functions.function(type_index);
        module.section(&functions);

        // Encode the export section
        let mut exports = ExportSection::new();
        exports.export("f", ExportKind::Func, 0);
        module.section(&exports);

        // Translate the function body
        let mut codes = CodeSection::new();
        let locals = vec![];
        let mut f = wasm_encoder::Function::new(locals);
        codes.function(&f);

        self.translate(&mut f, func);

        module.section(&codes);

        let wasm_bytes = module.finish();
    }

    fn translate(&self, f: &mut wasm_encoder::Function, func: kasl_ir::Function) {
        // Sort the blocks and collect the predecessors
        let rpo_blocks = func.sorted_blocks();
        let rpo_indices: HashMap<Block, usize> = rpo_blocks
            .iter()
            .enumerate()
            .map(|(i, b)| (*b, i))
            .collect();

        // Collect the predecessors and back edges
        let mut predecessors: HashMap<Block, Vec<Block>> = HashMap::new();
        let mut back_edges: HashSet<(Block, Block)> = HashSet::new();

        for &block in &rpo_blocks {
            for succ in func.get_block(&block).unwrap().get_successors() {
                predecessors.entry(succ).or_default().push(block);
                if rpo_indices[&succ] <= rpo_indices[&block] {
                    back_edges.insert((block, succ)); // blockからsuccへのback edge
                }
            }
        }

        let mut ctx = TranslationCtx {
            func: &func,
            rpo_indices,
            predecessors,
            back_edges,
            scope_stack: Vec::new(),
        };

        // Loop over the instruction and translate to WASM
        for block in rpo_blocks {
            self.translate_block(f, block, &mut ctx);
        }
    }
}
