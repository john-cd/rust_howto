import urllib.request
import json
import time

with open('../all_crates.txt', 'r') as f:
    crates = [line.strip() for line in f if line.strip() and line.strip() != 'std' and line.strip() != 'RustQuant' and line.strip() != 'zed']

headers = {'User-Agent': 'rust_howto_agent (agent@example.com)'}
category_crates = {}
missing = []

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
                    for cat_id in categories:
                        if cat_id not in category_crates:
                            category_crates[cat_id] = []
                        category_crates[cat_id].append(crate)
        time.sleep(1)
    except Exception as e:
        print(f'Exception for {crate}: {e}')

with open('category_crates.json', 'w') as f:
    json.dump(category_crates, f, indent=2)

with open('missing_categories.json', 'w') as f:
    json.dump(missing, f, indent=2)

print("Done grouping crates by category.")
