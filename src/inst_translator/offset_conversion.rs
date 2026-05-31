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

use kasl_ir::Offset;

/// Converts the given KASL-IR offset to a u64 value which the PointerScaled offset is scaled by 4.
pub(super) fn offset_to_wasm(offset: &Offset) -> u64 {
    match offset {
        Offset::Immediate(imm) => *imm as u64,
        Offset::PointerScaled(scale) => *scale as u64 * 4,
    }
}
