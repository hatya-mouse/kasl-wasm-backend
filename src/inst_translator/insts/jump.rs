use crate::inst_translator::InstTranslator;
use kasl_ir::{Block, Value};

impl InstTranslator<'_> {
    pub(super) fn inst_jump(&mut self, block: &Block, args: &[Value]) {
        if let Some(func_params) = self
            .kasl_func
            .get_block(block)
            .map(|data| data.get_params())
        {
            // Set the local curresponding to the block arguments
            for (param_val, arg_val) in func_params.iter().zip(args) {
                self.wasm_func
                    .instructions()
                    .local_get(self.ctx.val_map[arg_val]);
                self.wasm_func
                    .instructions()
                    .local_set(self.ctx.val_map[param_val]);
            }
        }

        // Get the number of nests to the block to jump to
        let wasm_block_index = self.ctx.block_ids[block];
        self.wasm_func
            .instructions()
            .i32_const(wasm_block_index as i32);
        self.wasm_func
            .instructions()
            .local_set(self.ctx.l_current_block);
    }
}
