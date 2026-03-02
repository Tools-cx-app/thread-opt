#!/system/bin/sh

# Ensure the binary is executable
chmod 755 "$MODPATH/thread-opt" || abort "! Failed to set permissions"

mkdir -p "/data/adb/thread-opt/"

if [ ! -f "/data/adb/thread-opt/config.prop" ]; then
  ui_print "- config is not exist, using empty config"
  echo "" >/data/adb/thread-opt/config.prop
fi

ui_print "- Installation complete"
