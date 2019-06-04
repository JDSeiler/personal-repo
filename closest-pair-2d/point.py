"""Class that represents a point in the cartesian plane"""

import typing

class Point:
    def __init__(self, x: float, y: float):
        """Create a point at x,y where x and y are floating point numbers."""
        self.x = x
        self.y = y