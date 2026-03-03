#!/system/bin/sh

MODDIR=${0%/*}

killall thread-opt
RUST_BACKTRACE=1 nohup $MODDIR/thread-opt 2>&1 &
