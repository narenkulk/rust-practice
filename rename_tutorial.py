import os

with open("./filenames.txt", "r") as f:
    filenames_list = f.readlines()
    filenames_dict = {}
    for i, item in enumerate(filenames_list):
        filenames_dict[f"{i+1}.mp4"] = item.strip(" \n")

for file in os.listdir("../Rust_Training"):
    # print(file)
    if file.startswith("lesson"):  # change this to mp4 later
        print(file)
        # print()
        ind = file.strip("lesson").strip(".mp4")
        # os.rename("../Rust_Training"+file, str(ind)+" "+filenames_dict[ind])
        print(filenames_dict[ind])
# print(filenames_dict)
