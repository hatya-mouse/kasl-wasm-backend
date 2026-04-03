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

use crate::backend::wasm_scope::WasmScope;
use kasl_ir::Block;
use std::collections::{HashMap, HashSet};

pub(super) struct TranslationCtx<'a> {
    pub func: &'a kasl_ir::Function,
    pub rpo_indices: HashMap<Block, usize>,
    pub predecessors: HashMap<Block, Vec<Block>>,
    pub back_edges: HashSet<(Block, Block)>,
    pub scope_stack: Vec<WasmScope>,
}
