// #[cfg(not(target_arch = "wasm32"))]
// pub async fn spawn_local<F>(future: F)
// where
//     F: Future<Output = ()> + 'static,
// {
//     use log::trace;
//     trace!("spawning with tokio::task::spawn_local");
//     tokio::task::spawn_local(future).await.unwrap();
//     trace!("after spawning with tokio::task::spawn_local");
// }

// #[cfg(target_arch = "wasm32")]
// pub async fn spawn_local<F>(future: F)
// where
//     F: Future<Output = ()> + 'static,
// {
//     use log::trace;
//     trace!("spawning with wasm_bindgen_futures::spawn_local");
//     wasm_bindgen_futures::spawn_local(future);
//     trace!("after spawning with wasm_bindgen_futures::spawn_local");
// }

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_local_block_or_not<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    use pollster::FutureExt as _;
    future.block_on();
}

#[cfg(target_arch = "wasm32")]
pub fn spawn_local_block_or_not<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}
