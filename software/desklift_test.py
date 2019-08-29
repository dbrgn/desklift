import pytest

import desklift


@pytest.mark.parametrize(['ms', 'direction', 'expected'], [
    # Up
    (1, 'up', []),
    (9, 'up', []),
    (10, 'up', [1]),
    (20, 'up', [2]),
    (200, 'up', [20]),
    (1260, 'up', [126]),
    (1270, 'up', [127]),
    (1280, 'up', [127, 1]),
    (2530, 'up', [127, 126]),
    (2540, 'up', [127, 127]),
    (2549, 'up', [127, 127]),
    (2570, 'up', [127, 127, 3]),
    (10000, 'up', [127, 127, 127, 127, 127, 127, 127, 111]),

    # Down
    (1, 'down', []),
    (9, 'down', []),
    (10, 'down', [255]),
    (30, 'down', [253]),
    (1270, 'down', [129]),
    (1280, 'down', [129, 255]),
])
def test_command(ms, direction, expected):
    cmd = desklift.make_commands(direction, ms)
    assert list(cmd) == expected
