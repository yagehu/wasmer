# wai-bindgen-wasmer

Runtime utility crate for Wasmer host WAI bindings generated by wai-bindgen-gen-wasmer.

This crate was moved from the https://github.com/wasmerio/wai repository.

wai-bindgen-wasmer needs Wasmer as a dependency, which makes it extremely awkard to use if the
crate is not in the same repo.
This is necessary because wai is now used for wasix_http_client bindings, and will also be used
for all WASI and WASIX syscalls in the future.

The medium-term plan is to rewrite wai-bindgen-gen-wasmer to make this create redundant.

See https://github.com/wasmerio/wai/issues/31 .