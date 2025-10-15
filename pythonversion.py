import random
import time
import os

distance = 0
food = 3000
aliens = 30
speed = 1

#Todo write a function that makes a default menu box that all menu function calls when they are created 
#write it in rust you pussy



class Ships:
    def __init__(self,capacity,maxspeed,name):
        self.capacity = capacity
        self.maxspeed = maxspeed
        self.name = name

    def shipstats(self,aliens,speed):
        print("Ship Name: ", self.name, "\n current capacity: ", aliens,"/",self.capacity, "Current Speed: ", speed,"/",self.maxspeed)

myship = Ships(50,10,'Flying Saucer')

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

def updatescreen(screen, width, height,food,aliens,speed,distance):
    os.system('clear')
    print("food: ",food, "\n aliens: ", aliens, "\n speed", speed, "\n distance:", distance)
    for row in screen:
        row.pop(0)
        row.append('*' if random.randint(0,7) == 0 else ' ')
    drawship(screen, width, height)
    for row in screen:
        print(''.join(row))

def updatefood(food,aliens):
    for i in range(aliens):
        food = food - 1
        if food < 0: 
            aliens = aliens - 1
    return food, aliens

def updatedistance(speed,distance):
    distance = distance + speed
    return distance 

def buymenu(myship):
    os.system('clear')
    print("")
#i should make it like cookie clicker for now i reckon and also do it in rust 
def playgame(speed,food,aliens,distance):
    width, height = get_terminal_size()
    screen = [[' ' for _ in range(width)] for _ in range(height)]
    initscreen(screen, width, height)
    while aliens > 0:
        action = input()
        if action == '':
            updatescreen(screen,width,height,food,aliens,speed,distance)
            food , aliens  = updatefood(food,aliens)
            distance = updatedistance(speed,distance)
        elif action == 's':
            myship.shipstats(aliens,speed)
        else: 
            continue
    print("YOU TRAVELLED: ", distance, "light years")

playgame(speed,food,aliens,distance)