#!/bin/sh

cargo tarpaulin --engine llvm --skip-clean --stderr $@ --include-files src/methods/*

