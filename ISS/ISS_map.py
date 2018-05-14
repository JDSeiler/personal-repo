import api_fetch
import time
import turtle

ISS = turtle.Turtle()
ISS.color('white')
screen = ISS.getscreen()
screen.bgpic('worldmap.gif')
screen.setup(1920,1080)
screen.screensize(1920,1080)
screen.mode('world')
screen.setworldcoordinates(-180.0000, -90.0000, 180.0000, 90.0000)

x_y = api_fetch.position()
print(x_y)
x = x_y[1]
y = x_y[0]

ISS.color('white')
ISS.shape("circle")
ISS.penup()
running = True
while running == True:
    time.sleep(2)
    x_y = api_fetch.position()
    x = x_y[1]
    y = x_y[0]
    ISS.setx(x * 1.1235) #because of the resolution, a scaling factor must be applied to longitude in order to work properly
    ISS.sety(y)

screen.mainloop()






