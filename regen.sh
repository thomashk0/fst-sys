#!/bin/bash

set -eu -o pipefail

bindgen fstapi/fstapi.h \
  --whitelist-function "fstReader.*" \
  --whitelist-type "fst.*" \
  --opaque-type "FILE" \
  -o src/raw.rs


