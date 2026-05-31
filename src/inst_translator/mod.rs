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

mod block_translator;
mod insts;
mod offset_conversion;

use crate::inst_translator::block_translator::translate_block;
use kasl_ir::{Block, IRType, Value, Variable};
use std::collections::HashMap;
use wasm_encoder::{BlockType, ValType};

#[derive(Default)]
struct TranslationContext {
    val_map: HashMap<Value, u32>,
    var_map: HashMap<Variable, u32>,
    val_types: HashMap<Value, IRType>,
    var_types: HashMap<Variable, IRType>,
    l_current_block: u32,
    l_stack_ptr: u32,
}

pub(super) fn construct_cfg(kasl_func: &kasl_ir::Function) -> wasm_encoder::Function {
    let blocks = kasl_func.get_blocks();
    let block_ids: HashMap<Block, u32> =
        blocks.iter().cloned().zip(0..blocks.len() as u32).collect();

    // Initialize local variables for SSA values and variables
    let (locals, context) = initialize_locals(kasl_func);
    let mut wasm_func = wasm_encoder::Function::new(locals);

    // Create the outermost loop to encompass the entire function body
    wasm_func.instructions().loop_(BlockType::Empty);

    // Add br_table instruction to jump to the appropriate block based on the current block index
    for _ in 0..blocks.len() {
        wasm_func.instructions().block(BlockType::Empty);
    }
    wasm_func.instructions().local_get(context.l_current_block);
    wasm_func
        .instructions()
        .br_table(block_ids.values().cloned().collect::<Vec<u32>>(), 0);

    // Translate each blocks
    let mut nests_to_loop = blocks.len() as u32;
    for block in blocks {
        let Some(block_data) = kasl_func.get_block(&block) else {
            continue;
        };

        // Translate the body of the block
        translate_block(&mut wasm_func, &context, block_data);

        // Jump back to the outermost loop to select the next block
        wasm_func.instructions().br(nests_to_loop);
        wasm_func.instructions().end();
        nests_to_loop -= 1;
    }

    wasm_func
}

/// Initializes the local variables.
fn initialize_locals(kasl_func: &kasl_ir::Function) -> (Vec<(u32, ValType)>, TranslationContext) {
    let mut context = TranslationContext::default();
    let mut locals = vec![];
    let mut local_index = 0;

    // Add SSA values as WASM locals
    for (val, ir_ty) in kasl_func.get_val_types() {
        let wasm_ty = convert_type(ir_ty);
        context.val_map.insert(*val, local_index);
        locals.push((local_index, wasm_ty));
        local_index += 1;
    }

    // Add variables as WASM locals
    for (var, ir_ty) in kasl_func.get_var_types() {
        let wasm_ty = convert_type(ir_ty);
        context.var_map.insert(*var, local_index);
        locals.push((local_index, wasm_ty));
        local_index += 1;
    }

    // Add a local variable to track the current block index and the stack pointer
    context.l_current_block = local_index;
    locals.push((local_index, ValType::I64));
    context.l_stack_ptr = local_index + 1;
    locals.push((local_index, ValType::I32));

    (locals, context)
}

/// Converts the KASL-IR type to the corresponding WebAssembly type.
fn convert_type(ty: &IRType) -> ValType {
    match ty {
        IRType::I8 => ValType::I32,
        IRType::I16 => ValType::I32,
        IRType::I32 => ValType::I32,
        IRType::I64 => ValType::I64,
        IRType::F32 => ValType::F32,
        IRType::F64 => ValType::F64,
        IRType::Void => ValType::I32,
        IRType::Ptr => ValType::I32,
    }
}
