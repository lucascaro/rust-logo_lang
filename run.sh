#!/bin/bash

./target/release/logo test.logo > result.svg && convert result.svg result.png
echo updated