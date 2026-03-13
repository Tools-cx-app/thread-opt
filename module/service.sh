#!/system/bin/sh

MODDIR=${0%/*}

killall thread-opt
RUST_BACKTRACE=1 nohup $MODDIR/thread-opt >"/data/adb/thread-opt/run.log" 2>&1 &
