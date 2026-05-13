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

use crate::ir_type::ir_type_to_val_type;
use kasl_ir::Function;
use wasm_encoder::{CodeSection, FunctionSection, Module, TypeSection, ValType};

pub struct WasmBackend {
    module: Module,
}

impl Default for WasmBackend {
    fn default() -> Self {
        Self {
            module: Module::new(),
        }
    }
}

impl WasmBackend {
    /// Compiles the function to Wasm binary.
    pub fn compile(&mut self, func: Function) -> Result<Vec<u8>, String> {
        let mut types = TypeSection::new();
        let mut functions = FunctionSection::new();
        let mut codes = CodeSection::new();

        // Get the entry block and its parameters to determine the function type
        let Some(entry_block) = func.entry_block().and_then(|b| func.get_block(&b)) else {
            return Err("No entry block".to_string());
        };

        let params: Vec<ValType> = entry_block
            .get_params()
            .iter()
            .map(|p| ir_type_to_val_type(func.get_val_type(*p)))
            .collect();

        // KASL-IR does not support return values
        types.ty().function(params, vec![]);
        functions.function(0);

        // TODO: Add function to the codes section

        self.module.section(&types);
        self.module.section(&functions);
        self.module.section(&codes);

        Ok(self.module.clone().finish())
    }
}
