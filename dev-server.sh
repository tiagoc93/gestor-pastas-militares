#!/bin/bash
cd "$(dirname "$0")/frontend"
exec python3 -m http.server 8080
