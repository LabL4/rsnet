import scipy as sp
import numpy as np

from typing import TypedDict
from utils import Range, RangeType

kb = sp.constants.Boltzmann
q = sp.constants.e


class StanfordModelParams(TypedDict):
    """
    Parameters for the RRAM Stanford model
    """

    model_switch: int
    """Switch between the standard model (0) or the dynamic model (1). [N/A]"""

    g0: float
    """Average switching parameter. Represents the resistance window. [m]"""
    V0: float
    """Average switching parameter. Depicts the nonlinarity of the resistance curve. [V]"""
    Vel0: float
    """Average switching parameter. Determines the voltage level at wich the gap starts to grow. It mainly changes the
    RESET knee-point voltage. [nm/ns]"""
    I0: float
    """Average switching parameter. Shifts the curve to different current levels. [A]"""
    beta: float
    """Average switching parameter. Captures the RESET slope (slope of the RESET slope as per the paper). [N/A]"""
    alpha: float
    """Average switching parameter. Changes the curvature of the Reset slope. [N/A]"""
    gamma0: float
    """Average switching parameter. Determines the voltage level at wich the gap starts to grow. It mainly changes the
    SET voltage. [N/A]"""

    T_crit: float
    """Threshold temperature for significant random variations in the dynamic model. [K]"""

    deltaGap0: float
    """Variations fitting parameters. [m]"""

    T_smth: float
    """Variations smoothing parameter. [K]"""
    Ea: float
    """Activation energy for vacancy generation. [eV]"""

    a0: float
    """Lattice parameter. Atom spacing. [m]"""

    T_ini: float
    """Initial room temperature in devices. [K]"""

    F_min: float
    """Minimum field requierement to enhance gap formation. [V/m]"""

    gap_ini: float
    """Initial gap distance. [m]"""

    gap_min: float
    """Minimum gap distance. [m]"""

    gap_max: float
    """Maximum gap distance. [m]"""

    Rth: float
    """Thermal resistance. [K/W]"""

    tox: float
    """Oxide thickness. [m]"""

    time_step: float
    """Time step bound for the simulation. [s]"""


STANFORD_MODEL_DEFAULT_PARAMS: StanfordModelParams = {
    "model_switch": 0,
    "g0": 0.25e-9,
    "V0": 0.25,
    "Vel0": 10,
    "I0": 1000e-6,
    "beta": 0.8,
    "alpha": 3,
    "gamma0": 16,

    "T_crit": 450,

    "deltaGap0": 0.02,

    "T_smth": 500,
    "Ea": 0.6,

    "a0": 0.25e-9,

    "T_ini": 273 + 25,

    "F_min": 1.4e9,

    "gap_ini": 2e-10,

    "gap_min": 2e-10,

    "gap_max": 17e-10,

    "Rth": 2.1e3,

    "tox": 12e-9,

    "time_step": 100e-6,
}

STANFORD_PARAMS_RANGES = {
    "model_switch": Range(0, 1, RangeType.ClosedClosed),

    "g0": Range(0, 2e-9, RangeType.OpenOpen),
    "V0": Range(0, 10, RangeType.OpenOpen),
    "Vel0": Range(0, 20, RangeType.OpenOpen),
    "I0": Range(0, 1e-2, RangeType.OpenOpen),
    "beta": Range(0, np.Inf, RangeType.OpenOpen),
    "alpha": Range(0, np.Inf, RangeType.OpenOpen),
    "gamma0": Range(0, np.Inf, RangeType.OpenOpen),

    "T_crit": Range(390, 460, RangeType.OpenOpen),

    "deltaGap0": Range(0, 0.1, RangeType.ClosedOpen),

    "T_smth": Range(400, 600, RangeType.OpenOpen),
    "Ea": Range(0, 1, RangeType.OpenOpen),

    "a0": Range(0, np.Inf, RangeType.OpenOpen),

    "T_ini": Range(0, np.Inf, RangeType.OpenOpen),

    "F_min": Range(0, 3e9, RangeType.OpenOpen),

    "gap_ini": Range(0, 100e-10, RangeType.OpenOpen),

    "gap_min": Range(0, 100e-10, RangeType.OpenOpen),

    "gap_max": Range(0, 100e-10, RangeType.OpenOpen),

    "Rth": Range(0, np.Inf, RangeType.OpenOpen),

    "tox": Range(0, 100e-9, RangeType.OpenOpen),

    "time_step": Range(1e-15, 1, RangeType.OpenOpen),
}


class StanfordModel:

    def __init__(self, params: StanfordModelParams):

        self.prev_gap_ddt = None
        self.prev_gap_random_ddt = None

        self.gap_random_ddt = None
        self.deltaGap = None
        self.gap_ddt = None
        self.gamma = None
        self.gap = 0
        self.T_cur = None
        self.gamma_ini = None

        for key in StanfordModelParams.__annotations__.keys():
            if key not in params:
                raise ValueError(f"Missing parameter {key}")
            else:
                is_valid = STANFORD_PARAMS_RANGES[key].validate(params[key])
                if not is_valid:
                    raise ValueError(
                        f"Invalid value ({key} = {params[key]}) for parameter `{key}`, range is {STANFORD_PARAMS_RANGES[key]}")

        self.params = params
        self.prev_time = 0
        self.prev_gap = self.params["gap_ini"]

        self.TE = 0
        """Voltage at top electrode. [V]"""
        self.BE = 0
        """Voltage at bottom electrode. [V]"""

        self.Itb = 0
        """Current across the device. [A]"""

        self.times = []
        self.gaps_ddt = []

    def step(self, current_time: float) -> float:
        self.times.append(current_time)

        vtb = self.TE - self.BE

        self.T_cur = self.params["T_ini"] + np.abs(vtb * self.Itb * self.params["Rth"])

        self.gamma_ini = self.params["gamma0"]
        if vtb < 0.0:
            self.gamma_ini = 16
        
        self.gamma = self.gamma_ini - self.params["beta"] * np.power(self.gap / 1e-9, self.params["alpha"])

        if self.gamma * np.abs(vtb) / self.params["tox"] < self.params["F_min"]:
            self.gamma = 0

        self.gap_ddt = -self.params["Vel0"] * np.exp(- q * self.params["Ea"] / kb / self.T_cur) * np.sinh(
            self.gamma * self.params["a0"] / self.params["tox"] * q * vtb / kb / self.T_cur)

        self.gaps_ddt.append(self.gap_ddt)

        self.deltaGap = self.params["deltaGap0"] * self.params["model_switch"]
        self.gap_random_ddt = np.random.randn() * self.deltaGap / (
                1 + np.exp((self.params["T_crit"] - self.T_cur) / self.params["T_smth"]))


        self.gap = self._compute_gap_int(current_time)

        self.prev_time = current_time
        self.prev_gap_random_ddt = self.gap_random_ddt
        self.prev_gap_ddt = self.gap_ddt
        self.prev_gap = self.gap
        
        if self.gap < self.params["gap_min"]:
            self.gap = self.params["gap_min"]
        elif self.gap > self.params["gap_max"]:
            self.gap = self.params["gap_max"]



        return self.params["I0"] * np.exp(-self.gap / self.params["g0"]) * np.sinh(vtb / self.params["V0"])

    def _compute_gap_int(self, current_time: float) -> float:
        if self.prev_gap_ddt is None:
            self.prev_gap_ddt = self.gap_ddt

        if self.prev_gap_random_ddt is None:
            self.prev_gap_random_ddt = self.gap_random_ddt

            return self.prev_gap

        # print(np.trapz(
        #     [self.prev_gap + self.prev_gap_random_ddt, self.gap + self.gap_random_ddt],
        #     [self.prev_time, current_time]))

        # return self.params["gap_ini"] + np.trapz([self.prev_gap], self.times)

        return self.prev_gap + np.trapz(
            [self.prev_gap_ddt + self.prev_gap_random_ddt, self.gap_ddt + self.gap_random_ddt],
            [self.prev_time, current_time])
