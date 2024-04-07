PROJ=oht

# The deprecated `-3` is (sadly) required, and 3072 is the largest feasible value already
openssl genrsa -out $PROJ.key -3 3072
openssl rsa -in $PROJ.key -pubout -out $PROJ.pem
