import urllib.request
import json
import time

with open('all_crates.txt', 'r') as f:
    crates = [line.strip() for line in f if line.strip() and line.strip() != 'std' and line.strip() != 'RustQuant']

missing = []
headers = {'User-Agent': 'rust_howto_agent (agent@example.com)'}
for crate in crates:
    try:
        req = urllib.request.Request(f'https://crates.io/api/v1/crates/{crate}', headers=headers)
        with urllib.request.urlopen(req) as response:
            if response.getcode() == 200:
                data = json.loads(response.read().decode())
                categories = data.get('crate', {}).get('categories', [])
                if not categories:
                    missing.append(crate)
            else:
                print(f'Error getting {crate}: {response.getcode()}')
        time.sleep(1)
    except Exception as e:
        print(f'Exception for {crate}: {e}')

print('Crates with no categories:', missing)
