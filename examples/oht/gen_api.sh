OE_ROOT=/opt/openenclave

oeedger8r --search-path $OE_ROOT/include --search-path $OE_ROOT/include/openenclave/edl/sgx oht.edl --untrusted --untrusted-dir host/api
oeedger8r --search-path $OE_ROOT/include --search-path $OE_ROOT/include/openenclave/edl/sgx oht.edl --trusted --trusted-dir enclave/api
