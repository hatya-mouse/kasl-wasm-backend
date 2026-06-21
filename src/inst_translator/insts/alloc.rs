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

use crate::inst_translator::insts::InstTranslator;
use kasl_ir::Value;

impl InstTranslator<'_> {
    pub(super) fn inst_alloc(&mut self, size: &u32, dst: &Value) {
        // Allocate memory for the variable
        self.wasm_func
            .instructions()
            .local_get(self.ctx.l_stack_ptr);
        self.wasm_func
            .instructions()
            .local_set(self.ctx.val_map[dst]);
        // Update the stack pointer
        self.wasm_func
            .instructions()
            .local_get(self.ctx.l_stack_ptr);
        self.wasm_func.instructions().i32_const(*size as i32);
        self.wasm_func.instructions().i32_add();
        self.wasm_func
            .instructions()
            .local_set(self.ctx.l_stack_ptr);
    }
}
