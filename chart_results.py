#!/usr/bin/env python3

import matplotlib.pyplot as plt

results = dict()
with open('results-SLATE.txt', 'r') as lines:
    for line in lines:
        words = line.split(', ')
        count = len(words)
        if count not in results:
            results[count] = 0
        results[count] += 1

fig, ax = plt.subplots(1,1)

labels = list(results.keys())
values = list(results.values())
plt.bar(labels, values, color=(96.0/255.0, 160.0/255.0, 94.0/255.0, 1.0))
ax.set_xlabel('Guesses per answer')
ax.set_ylabel('Words solved')

# Get rid of the border and tick marks which look cheap
ax.spines['top'].set_visible(False)
ax.spines['right'].set_visible(False)
#ax.spines['bottom'].set_visible(False)
ax.spines['left'].set_visible(False)
ax.get_yaxis().set_ticks([])

rects = ax.patches
for rect, label in zip(rects, values):
    height = rect.get_height()
    ax.text(rect.get_x() + rect.get_width() / 2, height+0.01, label,
            ha='center', va='bottom')

plt.savefig(f'results-SLATE.png')
