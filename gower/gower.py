from typing import Union

import numpy as np
import pandas as pd
from gower_rust import gower_matrix

def get_gower(arr: Union[np.array, pd.DataFrame]) -> np.array:
  """Prepare data for gower computation + run gower matrix."""

  # Ensure columns are consistently marked with correct dtypes

  # Split the data into categorical and numerical

  return gower_matrix(arr)
