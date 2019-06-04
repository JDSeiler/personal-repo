"""Container class for Point objects"""

from point import Point
import random as rnd
import typing

class Plane:
    def __init__(self):
        """Initializes the list used for storing Point objects
        but does not populate it.
        
        self.points = [] # list where points are stored
        """
        self.points = []

    def generate_random_plane(self, N: int, span: int):
        """Populates the plane with N random points, where span is the maximum value a point
        could have for either its x or y value. For instance, if the span is 5, the x and y value
        of every point will be between -5 and 5.
        """
        rnd.seed()
        for i in range(0, N):
            randx = float(rnd.randrange(-span, span))
            randy = float(rnd.randrange(-span, span))
            new_point = Point(randx, randy)
            self.points.append(new_point)


    
    