import numpy as np
from gower_rust import gower

def get_gower(arr: np.ndarray) -> np.ndarray:
  """Compute gower matrix for array where vertical
  axis correspond to columns of the same dtype."""

  # Ensure columns are consistently marked with correct dtypes

  return gower(arr)