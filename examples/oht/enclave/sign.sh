PROJ=oht
SUB_PROJ=oht-enclave

oesign sign -e build/$SUB_PROJ -c ../$PROJ.conf -k ../$PROJ.key
