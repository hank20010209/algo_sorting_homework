import csv
file_list = ['log25000.txt', 'log50000.txt', 'log100000.txt', 'log150000.txt', 'log200000.txt', 'log250000.txt']
case_title = ['Case 25000', 'Case 50000', 'Case 100000', 'Case 150000', 'Case 200000', 'Case 250000']
for file_ele, case_ele in zip(file_list, case_title):
    with open(file_ele, 'r') as file:
        with open('result.csv', 'a+', newline='') as csvfile:
            writer = csv.writer(csvfile)
            # initialize variables to store time elapsed for each sorting algorithm
            # csvfile = open('output.csv', 'w', newline = '')
            bubble_sort_times_cal = []
            insertion_sort_times_cal = []
            selection_sort_times_cal = []
            quick_sort_times_cal = []
            heap_sort_times_cal = []

            bubble_sort_times = []
            insertion_sort_times = []
            selection_sort_times = []
            quick_sort_times = []
            heap_sort_times = []
            
            # loop through each line in the file
            for line in file:
                # check if the line contains time elapsed for bubble_sort
                if 'bubble_sort()' in line:
                    # extract the time elapsed value and append to the list
                    bubble_sort_times_cal.append(float(line.split(':')[1][:-1].strip()[:-2]))
                    # bubble_sort_times.append(line.split(':')[1][:-1].strip())
                # check if the line contains time elapsed for insertion_sort
                elif 'insertion_sort()' in line:
                    # extract the time elapsed value and append to the list
                    insertion_sort_times_cal.append(float(line.split(':')[1][:-1].strip()[:-2]))
                    # insertion_sort_times.append(line.split(':')[1][:-1].strip())
                # check if the line contains time elapsed for selection_sort
                elif 'selection_sort()' in line:
                    # extract the time elapsed value and append to the list
                    selection_sort_times_cal.append(float(line.split(':')[1][:-1].strip()[:-2]))
                    # selection_sort_times.append(line.split(':')[1][:-1].strip())
                # check if the line contains time elapsed for quick_sort
                elif 'quick_sort()' in line:
                    # extract the time elapsed value and append to the list
                    quick_sort_times_cal.append(float(line.split(':')[1][:-1].strip()[:-2]) / 1000)
                    # quick_sort_times.append(line.split(':')[1][:-1].strip())
                # check if the line contains time elapsed for heap_sort
                elif 'heap_sort()' in line:
                    # extract the time elapsed value and append to the list
                    heap_sort_times_cal.append(float(line.split(':')[1][:-1].strip()[:-2]) / 1000)
                    # heap_sort_times.append(line.split(':')[1][:-1].strip())
            total_time_zip = zip(bubble_sort_times_cal, insertion_sort_times_cal, selection_sort_times_cal, quick_sort_times_cal, heap_sort_times_cal)
            # calculate the average time elapsed for bubble_sort and insertion_sort
            bubble_sort_avg_time = sum(bubble_sort_times_cal) / len(bubble_sort_times_cal)
            insertion_sort_avg_time = sum(insertion_sort_times_cal) / len(insertion_sort_times_cal)
            selection_sort_avg_time = sum(selection_sort_times_cal) / len(selection_sort_times_cal)
            quick_sort_avg_time = sum(quick_sort_times_cal) / len(quick_sort_times_cal)
            heap_sort_avg_time = sum(heap_sort_times_cal) / len(heap_sort_times_cal)
            
            
            
            writer.writerow([case_ele])
            writer.writerows(total_time_zip)
            writer.writerow(['Avg'])
            writer.writerow([bubble_sort_avg_time, insertion_sort_avg_time, selection_sort_avg_time, quick_sort_avg_time, heap_sort_avg_time])    
            print(case_ele)
            # print the results
            print('Average time elapsed in bubble_sort():', bubble_sort_avg_time, 's')
            print('Average time elapsed in insertion_sort():', insertion_sort_avg_time, 's')
            print('Average time elapsed in selection_sort():', selection_sort_avg_time, 's')
            print('Average time elapsed in quick_sort():', quick_sort_avg_time, 's')
            print('Average time elapsed in heap_sort():', heap_sort_avg_time, 's')
            print("----------------------------------------------------------")

    

