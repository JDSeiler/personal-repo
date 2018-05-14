import time
import json
import urllib.request

def position():
    rawData = urllib.request.urlopen('https://api.wheretheiss.at/v1/satellites/25544')
    parsed = json.loads(rawData.read().decode())
    lat = round(parsed['latitude'], 4)
    long = round(parsed['longitude'], 4)
    data_tuple = (lat, long)
    return data_tuple
