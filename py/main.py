import csv
import matplotlib.pyplot as plt
import math


def avg(array) -> float:
    return sum(array) / len(array)


def render_graph(i_dataset_avg, filename):
    plt.figure(dpi=1200)
    plt.margins(0)

    for x, y in i_dataset_avg:
        plt.scatter(x, y, s=2, c='r')

    plt.savefig(filename)


def render_graph_with_avg(i_dataset, i_dataset_avg, filename):
    plt.figure(dpi=1200)
    plt.margins(0)

    for x, y_set in i_dataset:
        for y in y_set:
            plt.scatter(x, y, s=2, c='b')

    for x, y in i_dataset_avg:
        plt.scatter(x, y, s=2, c='r')

    plt.savefig(filename)


raw_dataset = []

box_count = []
a, b, c, d, e = [], [], [], [], []
a_avg, b_avg, c_avg, d_avg, e_avg = [], [], [], [], []

with open('dataset.csv') as csv_file:
    csv_reader = csv.reader(csv_file, delimiter=',')
    for index, row in enumerate(csv_reader):
        if index != 0:
            raw_dataset.append([int(i) for i in row])

for n_index in range(0, 100):
    temp_a, temp_b, temp_c, temp_d, temp_e = [], [], [], [], []
    n = (n_index + 1) * 1000

    for k_index in range(0, 50):
        temp_a.append(raw_dataset[n_index + 100 * k_index][2])
        temp_b.append(raw_dataset[n_index + 100 * k_index][3])
        temp_c.append(raw_dataset[n_index + 100 * k_index][4])
        temp_d.append(raw_dataset[n_index + 100 * k_index][5])
        temp_e.append(raw_dataset[n_index + 100 * k_index][6])

    a.append([n, temp_a])
    b.append([n, temp_b])
    c.append([n, temp_c])
    d.append([n, temp_d])
    e.append([n, temp_e])

    a_avg.append([n, avg(temp_a)])
    b_avg.append([n, avg(temp_b)])
    c_avg.append([n, avg(temp_c)])
    d_avg.append([n, avg(temp_d)])
    e_avg.append([n, avg(temp_e)])

render_graph_with_avg(a, a_avg, "wykres_a")
render_graph_with_avg(b, b_avg, "wykres_b")
render_graph_with_avg(c, c_avg, "wykres_c")
render_graph_with_avg(d, d_avg, "wykres_d")
render_graph_with_avg(e, e_avg, "wykres_e")

a1, a2 = [], []
for x, y in a_avg:
    a1.append([x, y / x])
    a2.append([x, y / (x ** 0.5)])

b1 = []
for x, y in b_avg:
    b1.append([x, y / x])

c1, c2, c3 = [], [], []
for x, y in c_avg:
    c1.append([x, y / x])
    c2.append([x, y / (x * math.log(x))])
    c3.append([x, y / (x * x)])

d1, d2, d3 = [], [], []
for x, y in d_avg:
    d1.append([x, y / x])
    d2.append([x, y / (x * math.log(x))])
    d3.append([x, y / (x * x)])

e1, e2, e3 = [], [], []
for x, y in e_avg:
    e1.append([x, y / x])
    e2.append([x, y / (x * math.log(x))])
    e3.append([x, y / (x * math.log(math.log(x)))])

render_graph(a1, "dod_wykres_a1")
render_graph(a2, "dod_wykres_a2")
render_graph(b1, "dod_wykres_b1")
render_graph(c1, "dod_wykres_c1")
render_graph(c2, "dod_wykres_c2")
render_graph(c3, "dod_wykres_c3")
render_graph(d1, "dod_wykres_d1")
render_graph(d2, "dod_wykres_d2")
render_graph(d3, "dod_wykres_d3")
render_graph(e1, "dod_wykres_e1")
render_graph(e2, "dod_wykres_e2")
render_graph(e3, "dod_wykres_e3")
