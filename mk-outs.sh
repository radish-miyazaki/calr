#!/usr/bin/env bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

cal 2024 > $OUTDIR/2024.txt
cal 2 2024 > $OUTDIR/2-2024.txt
cal 4 2024 > $OUTDIR/4-2024.txt
cal 5 2024 > $OUTDIR/5-2024.txt
