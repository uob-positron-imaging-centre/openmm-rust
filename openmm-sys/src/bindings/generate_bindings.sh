bindgen OpenMMCWrapper.h -o lib.rs --whitelist-function "OpenMM_.*" --whitelist-type "OpenMM_.*" --whitelist-var "OpenMM_.*"
