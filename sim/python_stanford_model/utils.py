from enum import Enum


class RangeType(Enum):
    OpenOpen = "()"
    OpenClosed = "(]"
    ClosedOpen = "[)"
    ClosedClosed = "[]"


class Range:
    def __init__(self, min_v: float, max_v: float, ty: RangeType):
        self.min = min_v
        self.max = max_v
        self.ty = ty

    def validate(self, value: float):
        match self.ty:
            case RangeType.OpenOpen:
                return self.min < value < self.max
            case RangeType.OpenClosed:
                return self.min < value <= self.max
            case RangeType.ClosedOpen:
                return self.min <= value < self.max
            case RangeType.ClosedClosed:
                return self.min <= value <= self.max

    def __str__(self):
        ty_str = self.ty.value
        return f"{ty_str[0]}{self.min}, {self.max}{ty_str[1]}"
