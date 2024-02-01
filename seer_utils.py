"""
Utility functionality for SEER
"""
import requests
from typing import List
from dataclasses import dataclass

@dataclass
class Disease:
    """Container for holding a disease (Cancer)
    """
    cancer_id: int # Numerical ID of cancer
    cancer_type: str # Type of cancer
    cancer_name: str # Name of cancer

def get_all(url: str, header: dict, count: int=25) -> List:
    """Fetch all records from a paginated API

    Args:
        url (str): API endpoint.
        header(dict): Header for the API call. (i.e. has API key).
        count (int, optional): Max page size. Defaults to 25.

    Returns:
        List: A list of dictionaries representing the objects.
    """
    all_elements = []
    def get_total() -> int:
        params = {'count': '1'}
        r = requests.get(base_url, headers=header, params=params)
        return r.json()['total']
    nrecords = get_total()
    npages = ( nrecords + count - 1 ) // count
    n = 0
    for i in range(npages):
        params = {'count': count, 'offset': i * count}
        r = requests.get(base_url, headers=header, params=params)
        all_elements.extend(r.json()['results'])
    return all_elements