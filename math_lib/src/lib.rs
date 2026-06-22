#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// --- BINARY OPERATIONS ---

#[unsafe(no_mangle)]
pub extern "C" fn wasm_pow(base: f32, exp: f32) -> f32 {
    libm::powf(base, exp)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_atan2(y: f32, x: f32) -> f32 {
    libm::atan2f(y, x)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_log(y: f32, x: f32) -> f32 {
    libm::logf(y) / libm::logf(x)
}

// --- UNARY OPERATIONS ---

#[unsafe(no_mangle)]
pub extern "C" fn wasm_sin(x: f32) -> f32 {
    libm::sinf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_cos(x: f32) -> f32 {
    libm::cosf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_tan(x: f32) -> f32 {
    libm::tanf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_asin(x: f32) -> f32 {
    libm::asinf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_acos(x: f32) -> f32 {
    libm::acosf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_atan(x: f32) -> f32 {
    libm::atanf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_exp(x: f32) -> f32 {
    libm::expf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_log10(x: f32) -> f32 {
    libm::log10f(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_log2(x: f32) -> f32 {
    libm::log2f(x)
}
