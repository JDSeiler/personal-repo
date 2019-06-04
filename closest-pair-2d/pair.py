"""Class representing a pair of Point objects"""

from point import Point

class Pair:
    def __init__(self, p1: Point, p2: Point):
        """Create a Pair using Point objects p1 and p2.
        Also stores the distance between p1 and p2 as self.dist"""
        self.p1 = p1
        self.p2 = p2
        self.dist = p1.dist_from(p2)