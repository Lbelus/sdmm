#!/bin/bash

# don't forget to souce the file . script.sh or source script.sh

make -C ft_malloc/
cp ft_malloc/sdmm_malloc.o sdmm/src/bpf/sdmm_malloc.o
cp ft_malloc/include/wrapper.h sdmm/src/bpf/wrapper.h

export C_INCLUDE_PATH=/workspace/ft_malloc/include/