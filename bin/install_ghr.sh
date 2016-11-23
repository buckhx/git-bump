#!/bin/bash

mkdir -p tmp && cd tmp
curl -sLO "https://github.com/tcnksm/ghr/releases/download/v0.5.3/ghr_v0.5.3_linux_amd64.zip"
unzip "ghr_v0.5.3_linux_amd64.zip"
mv ghr /usr/local/bin && chmod +x /usr/local/bin/ghr
cd .. && rm -rf tmp
