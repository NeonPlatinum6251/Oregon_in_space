import random
import time
import os
'''
import keyboard
'''
Playing = True
distance = 0.0
food = 0
aliens = 0
speed = 1

def get_terminal_size():
    try:
        size = os.get_terminal_size()
        return size.columns, size.lines
    except OSError:
        return 80, 24

def drawship(screen, width, height):
    ship = [
        "     ___",
        " ___/   \\___",
        "/   '---'   \\",
        "'--_______--'"
    ]
    ship_height = len(ship)
    ship_width = max(len(line) for line in ship)
    start_row = height // 2 - ship_height // 2
    start_col = width // 2 - ship_width // 2

    for i, line in enumerate(ship):
        for j, char in enumerate(line):
            row = start_row + i
            col = start_col + j
            if 0 <= row < height and 0 <= col < width:
                screen[row][col] = char

def initscreen(screen, width, height):
    for k in range(height):
        for l in range(width):
            screen[k][l] = '*' if random.randint(0,7) == 0 else ' '
    drawship(screen, width, height)
    for row in screen:
        print(''.join(row))

def updatescreen(screen, width, height):
    os.system('clear')
    for row in screen:
        row.pop(0)
        row.append('*' if random.randint(0,7) == 0 else ' ')
    drawship(screen, width, height)
    for row in screen:
        print(''.join(row))

def playgame(speed):
    width, height = get_terminal_size()
    screen = [[' ' for _ in range(width)] for _ in range(height)]
    initscreen(screen, width, height)
    for i in range(45):
        time.sleep(0.5)
        updatescreen(screen,width,height)
playgame(speed)