#[macro_export]
macro_rules! slice_up_to {
    ($content:expr, $max_len:expr) => {
        $content[..std::cmp::min($content.len(), $max_len)].as_ref()
    };
}
