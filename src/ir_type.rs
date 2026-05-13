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

use kasl_ir::IRType;
use wasm_encoder::ValType;

pub fn ir_type_to_val_type(ty: IRType) -> ValType {
    match ty {
        IRType::I8 | IRType::I16 | IRType::I32 => ValType::I32,
        IRType::I64 => ValType::I64,
        IRType::F32 => ValType::F32,
        IRType::F64 => ValType::F64,
        IRType::Ptr => ValType::I32,
        IRType::Void => unreachable!("Void is not a value type"),
    }
}
