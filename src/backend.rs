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

use crate::{WasmBackendError, inst_translator::construct_cfg, ir_type::ir_type_to_val_type};
use kasl_ir::Function;
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, FunctionSection, Module, TypeSection, ValType,
};

/// Compiles the function to Wasm binary.
pub fn compile(kasl_func: Function) -> Result<Vec<u8>, WasmBackendError> {
    let mut module = Module::new();

    let mut types = TypeSection::new();
    module.section(&types);

    // Get the entry block and its parameters to determine the function type
    let Some(entry_block) = kasl_func
        .entry_block()
        .and_then(|b| kasl_func.get_block(&b))
    else {
        return Err(WasmBackendError::NoEntryBlock);
    };

    let params: Vec<ValType> = entry_block
        .get_params()
        .iter()
        .map(|p| ir_type_to_val_type(kasl_func.get_val_type(*p)))
        .collect();

    // KASL-IR does not support return values
    let mut functions = FunctionSection::new();
    types.ty().function(params, vec![]);
    functions.function(0);
    module.section(&functions);

    let mut exports = ExportSection::new();
    exports.export("f", ExportKind::Func, 0);
    module.section(&exports);

    // Construct the function body and add it to the code section
    let mut codes = CodeSection::new();
    let wasm_func = construct_cfg(&kasl_func);
    codes.function(&wasm_func);
    module.section(&codes);

    Ok(module.clone().finish())
}
