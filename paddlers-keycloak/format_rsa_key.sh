#!/bin/bash
if [ "$#" -ne 1 ]; then
    echo 'Usage: ./format_rsa_key.sh "publicKeyAsCopiedFromKeycloakAdminInterface" > pub_rsa.der'
else
    echo -e "-----BEGIN PUBLIC KEY-----\r\n$1\r\n-----END PUBLIC KEY-----" | openssl rsa -pubin -RSAPublicKey_out -outform DER
fi
