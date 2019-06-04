"""Class that represents a point in the cartesian plane"""

import math
import typing

class Point:
    def __init__(self, x: float, y: float):
        """Create a point at x,y where x and y are floating point numbers."""
        self.x = x
        self.y = y

    def abs_dist_from(self, other: Point):
        """Returns the absolute distance between self and another point."""
        relative_x = self.x - other.x
        relative_y = self.y - other.y
        
        x_sqrd = math.pow(relative_x, 2)
        y_sqrd = math.pow(relative_y, 2)

        return math.sqrt((x_sqrd + y_sqrd))