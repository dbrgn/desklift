set -x
openocd \
    -f jlink-swd.cfg \
    -f /usr/share/openocd/scripts/target/stm32f1x.cfg \
    -c 'init; reset; halt'
