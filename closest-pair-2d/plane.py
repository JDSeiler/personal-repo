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

    def add_points(self, new_points: typing.List[Point]):
        """Add a list of Point objects to the plane"""
        self.points.append(new_points)

    
    
    