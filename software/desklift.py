#!/usr/bin/env python3

import ctypes
import math
import sys

import serial


DEV = '/dev/ttyACM0'
BAUD = 115200


def make_commands(direction: str, milliseconds: int) -> bytes:
    assert milliseconds > 0, 'Duration may not be negative'
    steps = milliseconds // 10
    sign = 1 if direction == 'up' else -1
    cmd_max = 127
    iterations = math.ceil((steps + 1) / 127)
    commands = []
    for i in range(iterations):
        if i + 1 < iterations:
            commands.append(ctypes.c_ubyte(cmd_max * sign).value)
        elif (steps % cmd_max) > 0:
            commands.append(ctypes.c_ubyte((steps % cmd_max) * sign).value)
    return commands


def main(direction: str, milliseconds: int):
    assert milliseconds <= 15000, 'Duration may not be larger than 15 seconds'
    desklift = serial.Serial(DEV, baudrate=BAUD)
    commands = make_commands(direction, milliseconds)
    desklift.write(bytes(commands))


if __name__ == '__main__':
    if len(sys.argv) != 3 or sys.argv[1] not in ['up', 'down']:
        print('Usage: {} (up|down) <milliseconds>'.format(sys.argv[0]))
        sys.exit(1)
    main(sys.argv[1], int(sys.argv[2]))
