import requests
import sys
import datetime
import os
from dotenv import load_dotenv

load_dotenv()

# Get environment variables
COOKIE = os.getenv('COOKIE')

if len(sys.argv) > 2:
    year = sys.argv[1]
    day = sys.argv[2]
else:
    year = datetime.datetime.now().year
    day = datetime.datetime.now().day

print("Downloading input for day {} of year {}".format(day, year))

URL = f"https://adventofcode.com/{year}/day/{day}/input"
resp = requests.get(URL, allow_redirects=True, cookies={'session': COOKIE})

if resp.status_code == 200:
    with open(f'src/bin/day{day}_input.txt', 'wb') as f:
        f.write(resp.content)
    print(f"Input saved to day{day}_input.txt")
else:
    print(resp.status_code)
