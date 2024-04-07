PROJ=oht
SUB_PROJ=oht-enclave

oesign sign -e build/enclave/$SUB_PROJ -c $PROJ.conf -k $PROJ.key
