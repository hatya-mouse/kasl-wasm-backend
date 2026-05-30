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
use std::collections::HashSet;

#[derive(PartialEq)]
pub(super) enum EdgeType {
    BackEdge,
    ForwardEdge,
}

impl GraphTranslator<'_> {
    /// Collect the loop header blocks and target blocks.
    pub(super) fn classify_edges(
        &self,
        sorted_blocks: &[Block],
    ) -> (HashSet<Block>, HashSet<Block>) {
        // Blocks with at least one back edge targeting them
        let mut loop_headers = HashSet::new();
        // Blocks with at least one forward edges targeting them
        let mut target_blocks = HashSet::new();

        for block in &self.ir_func.get_blocks() {
            let Some(block_data) = self.ir_func.get_block(block) else {
                continue;
            };
            let successors = block_data.get_successors();

            // Classify edges and identify loop headers
            for succ in &successors {
                let edge_type = self.classify_edge(block, succ, sorted_blocks);
                match edge_type {
                    EdgeType::BackEdge => {
                        loop_headers.insert(*succ);
                    }
                    EdgeType::ForwardEdge => {
                        target_blocks.insert(*succ);
                    }
                }
            }
        }

        (loop_headers, target_blocks)
    }

    /// Determines whether the given block is a back edge or a forward edge.
    pub(super) fn classify_edge(
        &self,
        from: &Block,
        to: &Block,
        sorted_blocks: &[Block],
    ) -> EdgeType {
        let from_index = sorted_blocks.iter().position(|b| b == from).unwrap();
        let to_index = sorted_blocks.iter().position(|b| b == to).unwrap();

        if to_index <= from_index {
            EdgeType::BackEdge
        } else {
            EdgeType::ForwardEdge
        }
    }
}
