import sys
import os
import json


ANB = os.path.join(
    os.path.dirname(__file__), "ambiguous.json")

with open(ANB) as f:
    anb = json.load(f)

# anb_new = {}
# for k, vs in anb.items():
#     vs_dedup = []
#     for v in vs:
#         if not v in vs_dedup:
#             vs_dedup.append(v)
#     anb_new.update({k: vs_dedup})

with open(ANB, 'w') as f:
    json.dump(anb, f)
